use crate::models::task::Task;
use crate::parser::org_parser::ParsedOrgFile;
use std::collections::HashMap;
use uuid::Uuid;

pub struct TaskStore {
    tasks_by_file: HashMap<String, Vec<Task>>,
}

impl TaskStore {
    pub fn new() -> Self {
        Self {
            tasks_by_file: HashMap::new(),
        }
    }

    pub fn add_tasks_from_file(&mut self, parsed_file: ParsedOrgFile) {
        let tasks = parsed_file
            .headlines
            .iter()
            .map(|h| {
                let mut task = Task::from(h);
                task.file_path = parsed_file.file_path.clone();
                task
            })
            .collect();
        self.tasks_by_file
            .insert(parsed_file.file_path.clone(), tasks);
    }

    pub fn remove_tasks_by_file(&mut self, file_path: &str) -> Option<Vec<Task>> {
        self.tasks_by_file.remove(file_path)
    }

    pub fn get_task(&self, task_id: Uuid) -> Option<&Task> {
        self.tasks_by_file
            .values()
            .flat_map(|tasks| tasks.iter())
            .find(|task| task.id == task_id)
    }

    pub fn get_all_tasks(&self) -> Vec<&Task> {
        self.tasks_by_file.values().flat_map(|tasks| tasks.iter()).collect()
    }

    pub fn filter_tasks<F>(&self, filter: F) -> Vec<&Task>
    where
        F: Fn(&&Task) -> bool,
    {
        self.get_all_tasks().into_iter().filter(filter).collect()
    }

    pub fn update_task(&mut self, updated_task: Task) -> Option<Task> {
        for tasks in self.tasks_by_file.values_mut() {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == updated_task.id) {
                let old_task = std::mem::replace(task, updated_task);
                return Some(old_task);
            }
        }
        None
    }

    pub fn remove_task(&mut self, task_id: Uuid) -> Option<Task> {
        for tasks in self.tasks_by_file.values_mut() {
            if let Some(index) = tasks.iter().position(|t| t.id == task_id) {
                return Some(tasks.remove(index));
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
    fn test_filter_tasks() {
        let store = create_test_store();
        let todo_tasks = store.filter_tasks(|task| task.state == TodoState::Todo);
        assert_eq!(todo_tasks.len(), 1);
        assert_eq!(todo_tasks[0].title, "Task 1");

        let done_tasks = store.filter_tasks(|task| task.state == TodoState::Done);
        assert_eq!(done_tasks.len(), 1);
        assert_eq!(done_tasks[0].title, "Task 2");
    }
} 