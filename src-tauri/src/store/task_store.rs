use crate::models::task::Task;
use crate::parser::org_parser::{OrgHeadline, ParsedOrgFile};
use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;

/// A task plus the `OrgHeadline` it was parsed from. The headline's
/// `header_range` is what a command needs to find and rewrite the right
/// spot in the source file (see `OrgParser::update_headline`) — `Task`
/// alone, being the IPC-facing struct, doesn't carry that.
type TaskEntry = (Task, OrgHeadline);

pub struct TaskStore {
    tasks_by_file: HashMap<String, Vec<TaskEntry>>,
}

impl TaskStore {
    pub fn new() -> Self {
        Self {
            tasks_by_file: HashMap::new(),
        }
    }

    /// Only headlines with a todo keyword become tasks — a plain note
    /// headline (no TODO/DONE/... keyword) has no `todo_state` and would
    /// otherwise default to `TodoState::Todo` in `Task::new`, showing up as
    /// an open task even though it's just a note (see M1 in the code
    /// review).
    pub fn add_tasks_from_file(&mut self, parsed_file: ParsedOrgFile) {
        let entries = parsed_file
            .headlines
            .iter()
            .filter(|h| h.todo_state.is_some())
            .map(|h| (Task::new(h, &parsed_file.file_path), h.clone()))
            .collect();
        self.tasks_by_file
            .insert(parsed_file.file_path.clone(), entries);
    }

    pub fn remove_tasks_by_file(&mut self, file_path: &str) -> Option<Vec<Task>> {
        self.tasks_by_file
            .remove(file_path)
            .map(|entries| entries.into_iter().map(|(task, _)| task).collect())
    }

    /// Drops every file whose (canonicalized) path is under `folder_path`,
    /// e.g. when a watched folder is removed. `folder_path` is expected to
    /// already be canonicalized, matching the keys `add_tasks_from_file`
    /// inserts (see `OrgParser::normalize_path`).
    pub fn remove_tasks_by_folder(&mut self, folder_path: &str) {
        self.tasks_by_file
            .retain(|file_path, _| !Path::new(file_path).starts_with(folder_path));
    }

    /// The stored `(Task, OrgHeadline)` pair for `task_id`, if any. Commands
    /// that need both the IPC-facing task and the headline to rewrite on
    /// disk (update/delete) should use this instead of two separate lookups.
    pub fn get_entry(&self, task_id: Uuid) -> Option<&TaskEntry> {
        self.tasks_by_file
            .values()
            .flat_map(|entries| entries.iter())
            .find(|(task, _)| task.id == task_id)
    }

    pub fn get_task(&self, task_id: Uuid) -> Option<&Task> {
        self.get_entry(task_id).map(|(task, _)| task)
    }

    /// The `OrgHeadline` a task was parsed from — the source of the
    /// `range`/`header_range` needed to rewrite or delete it on disk.
    pub fn get_headline(&self, task_id: Uuid) -> Option<&OrgHeadline> {
        self.get_entry(task_id).map(|(_, headline)| headline)
    }

    pub fn get_all_tasks(&self) -> Vec<&Task> {
        self.tasks_by_file
            .values()
            .flat_map(|entries| entries.iter())
            .map(|(task, _)| task)
            .collect()
    }

    pub fn filter_tasks<F>(&self, filter: F) -> Vec<&Task>
    where
        F: Fn(&&Task) -> bool,
    {
        self.get_all_tasks().into_iter().filter(filter).collect()
    }

    pub fn update_task(&mut self, updated_task: Task) -> Option<Task> {
        for entries in self.tasks_by_file.values_mut() {
            if let Some((task, _)) = entries.iter_mut().find(|(t, _)| t.id == updated_task.id) {
                let old_task = std::mem::replace(task, updated_task);
                return Some(old_task);
            }
        }
        None
    }

    pub fn remove_task(&mut self, task_id: Uuid) -> Option<Task> {
        for entries in self.tasks_by_file.values_mut() {
            if let Some(index) = entries.iter().position(|(t, _)| t.id == task_id) {
                return Some(entries.remove(index).0);
            }
        }
        None
    }
}

impl Default for TaskStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::task::TodoState;
    use crate::parser::org_parser::OrgParser;

    fn create_test_store() -> TaskStore {
        let parser = OrgParser::new();
        let file_content = r#"
* TODO Task 1
* DONE Task 2
"#;
        let headlines = parser.parse_content(file_content).unwrap();
        let parsed_file = ParsedOrgFile {
            file_path: "test.org".to_string(),
            content: file_content.to_string(),
            headlines,
        };
        let mut store = TaskStore::new();
        store.add_tasks_from_file(parsed_file);
        store
    }

    #[test]
    fn test_add_and_get_tasks() {
        let store = create_test_store();
        let tasks = store.get_all_tasks();
        assert_eq!(tasks.len(), 2);
    }

    #[test]
    fn test_update_task() {
        let mut store = create_test_store();
        let mut task_to_update = store.get_all_tasks()[0].clone();
        task_to_update.title = "Updated Task 1".to_string();
        task_to_update.state = TodoState::Done;

        let old_task = store.update_task(task_to_update.clone()).unwrap();
        assert_eq!(old_task.title, "Task 1");

        let updated_task = store.get_task(task_to_update.id).unwrap();
        assert_eq!(updated_task.title, "Updated Task 1");
        assert_eq!(updated_task.state, TodoState::Done);
    }

    #[test]
    fn test_remove_task() {
        let mut store = create_test_store();
        let task_to_remove = store.get_all_tasks()[0].clone();
        
        let removed_task = store.remove_task(task_to_remove.id).unwrap();
        assert_eq!(removed_task.id, task_to_remove.id);
        
        let tasks = store.get_all_tasks();
        assert_eq!(tasks.len(), 1);
        
        let remaining_task = tasks[0];
        assert_ne!(remaining_task.id, task_to_remove.id);
    }

    #[test]
    fn test_remove_tasks_by_folder() {
        let mut store = create_test_store(); // inserts under "test.org"
        let parser = OrgParser::new();
        let other_file = "/watched/folder/other.org";
        let parsed_other = ParsedOrgFile {
            file_path: other_file.to_string(),
            content: "* TODO Other\n".to_string(),
            headlines: parser.parse_content("* TODO Other\n").unwrap(),
        };
        store.add_tasks_from_file(parsed_other);
        assert_eq!(store.get_all_tasks().len(), 3);

        store.remove_tasks_by_folder("/watched/folder");

        let remaining = store.get_all_tasks();
        assert_eq!(remaining.len(), 2);
        assert!(remaining.iter().all(|t| t.file_path != other_file));
    }

    #[test]
    fn test_plain_note_headlines_are_not_materialized_as_tasks() {
        let parser = OrgParser::new();
        let file_content = "* TODO Real task\n* Just a note, no keyword\n** Another note\n";
        let parsed_file = ParsedOrgFile {
            file_path: "notes.org".to_string(),
            content: file_content.to_string(),
            headlines: parser.parse_content(file_content).unwrap(),
        };
        let mut store = TaskStore::new();
        store.add_tasks_from_file(parsed_file);

        let tasks = store.get_all_tasks();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "Real task");
    }

    #[test]
    fn test_filter_tasks() {
        let store = create_test_store();
        let todo_tasks = store.filter_tasks(|task| task.state == TodoState::Todo);
        assert_eq!(todo_tasks.len(), 1);
        assert_eq!(todo_tasks[0].title, "Task 1");

        let done_tasks = store.filter_tasks(|task| task.state == TodoState::Done);
        assert_eq!(done_tasks.len(), 1);
        assert_eq!(done_tasks[0].title, "Task 2");
    }

    #[test]
    fn test_get_headline_for_task() {
        let store = create_test_store();
        let task = store.get_all_tasks()[0].clone();

        let headline = store.get_headline(task.id).unwrap();
        assert_eq!(headline.title, task.title);
        assert!(headline.header_range.is_some());
    }

    #[test]
    fn test_get_entry_returns_matching_task_and_headline() {
        let store = create_test_store();
        let task = store.get_all_tasks()[0].clone();

        let (entry_task, entry_headline) = store.get_entry(task.id).unwrap();
        assert_eq!(entry_task.id, task.id);
        assert_eq!(entry_headline.title, task.title);

        assert!(store.get_entry(Uuid::new_v4()).is_none());
    }

    #[test]
    fn test_ids_stable_after_reparsing_same_file() {
        let mut store = create_test_store();
        let ids_before: Vec<Uuid> = store.get_all_tasks().iter().map(|t| t.id).collect();

        // Simulate a watcher reparse of the same file content.
        let parser = OrgParser::new();
        let file_content = "\n* TODO Task 1\n* DONE Task 2\n";
        let parsed_file = ParsedOrgFile {
            file_path: "test.org".to_string(),
            content: file_content.to_string(),
            headlines: parser.parse_content(file_content).unwrap(),
        };
        store.add_tasks_from_file(parsed_file);

        let mut ids_after: Vec<Uuid> = store.get_all_tasks().iter().map(|t| t.id).collect();
        let mut ids_before_sorted = ids_before.clone();
        ids_before_sorted.sort();
        ids_after.sort();
        assert_eq!(ids_before_sorted, ids_after);
    }
} 