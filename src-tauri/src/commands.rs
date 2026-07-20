// Tauri command handlers for IPC communication with React frontend
// Contains all the API endpoints as Tauri commands

use crate::{
    models::task::{Priority, Task},
    parser::org_parser::{OrgHeadline, OrgParser, ParsedOrgFile, ParserError},
    settings::Settings,
    store::task_store::TaskStore,
    watcher::file_watcher::{FileWatcher, TASKS_CHANGED_EVENT},
};
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, State};
use uuid::Uuid;

// Placeholder function to ensure compilation
#[tauri::command]
pub fn hello_from_rust(name: &str) -> String {
    format!("Hello, {}! You've successfully connected to Rust backend.", name)
}

#[tauri::command]
pub fn parse_org_content(content: String) -> Result<Vec<OrgHeadline>, String> {
    let parser = OrgParser::new();
    
    parser.parse_content(&content)
        .map_err(|e| format!("Failed to parse org content: {}", e))
}

#[tauri::command]
pub fn parse_org_file(file_path: String) -> Result<ParsedOrgFile, String> {
    let parser = OrgParser::new();
    
    // Check if it's a valid org file
    if !OrgParser::is_org_file(&file_path) {
        return Err("File is not a valid .org file".to_string());
    }
    
    // Check if file exists
    if !Path::new(&file_path).exists() {
        return Err(format!("File does not exist: {}", file_path));
    }
    
    parser.parse_file(&file_path)
        .map_err(|e| format!("Failed to parse org file: {}", e))
}

#[derive(Debug, Serialize)]
pub enum CommandError {
    Store(String),
    Parser(String),
    NotFound(String),
}

impl From<ParserError> for CommandError {
    fn from(err: ParserError) -> Self {
        CommandError::Parser(err.to_string())
    }
}

impl From<String> for CommandError {
    fn from(err: String) -> Self {
        CommandError::Store(err)
    }
}

pub struct AppState {
    pub store: Arc<Mutex<TaskStore>>,
    pub parser: Arc<OrgParser>,
    pub watcher: Arc<Mutex<FileWatcher>>,
    pub settings: Arc<Mutex<Settings>>,
    pub settings_path: PathBuf,
}

/// Persists the in-memory `Settings` to disk, logging (not failing the
/// command) on error — a save failure shouldn't roll back an
/// already-applied in-memory change.
fn persist_settings(state: &AppState, settings: &Settings) {
    if let Err(e) = settings.save(&state.settings_path) {
        log::error!("Failed to save settings to {:?}: {:?}", state.settings_path, e);
    }
}

/// Appends `\n* TODO {title}\n` to the configured inbox file (creating it,
/// and any missing parent directories, on first use), reparses that file so
/// the new headline gets its real stable id, and refreshes every task the
/// store had cached for it (their ranges are unaffected by an append, but
/// this keeps a single code path for "write file, then resync store from
/// disk" across create/update/delete). Returns the newly created task.
fn create_task_impl(state: &AppState, title: String) -> Result<Task, CommandError> {
    let inbox_file = state
        .settings
        .lock()
        .unwrap()
        .inbox_file
        .clone()
        .ok_or_else(|| {
            CommandError::Store("No inbox file configured — set one in Settings".to_string())
        })?;
    let inbox_path = Path::new(&inbox_file);

    if let Some(parent) = inbox_path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(|e| CommandError::Store(e.to_string()))?;
        }
    }
    let mut content = std::fs::read_to_string(inbox_path).unwrap_or_default();
    if !content.is_empty() && !content.ends_with('\n') {
        content.push('\n');
    }
    content.push_str(&format!("* TODO {}\n", title));
    std::fs::write(inbox_path, &content).map_err(|e| CommandError::Store(e.to_string()))?;

    let parsed = state.parser.parse_file(inbox_path)?;
    let new_task = parsed
        .headlines
        .last()
        .map(|h| Task::new(h, &parsed.file_path))
        .ok_or_else(|| {
            CommandError::Store("Failed to parse the task just written to disk".to_string())
        })?;

    state.store.lock().unwrap().add_tasks_from_file(parsed);
    Ok(new_task)
}

#[tauri::command]
pub fn create_task(
    title: String,
    app_handle: AppHandle,
    state: State<AppState>,
) -> Result<Task, CommandError> {
    let task = create_task_impl(&state, title)?;
    let _ = app_handle.emit(TASKS_CHANGED_EVENT, ());
    Ok(task)
}

/// Rewrites the headline `task.id` was parsed from with `task`'s title,
/// state, tags, and priority, then reparses the file so the store's cached
/// ranges for every headline in it (not just this one — a rewrite can shift
/// later headlines' offsets) stay valid for the next edit. Returns the task
/// as freshly parsed back from disk.
///
/// `scheduled`/`deadline` are deliberately left untouched: `Task` only
/// carries them as plain ISO dates (see `models::task::org_timestamp_to_iso_date`),
/// and nothing in the frontend edits them yet, so regenerating an org
/// timestamp from just a date would silently drop any repeater (`+1w`) or
/// weekday text already in the file.
fn update_task_impl(state: &AppState, task: Task) -> Result<Task, CommandError> {
    let old_headline = state
        .store
        .lock()
        .unwrap()
        .get_headline(task.id)
        .cloned()
        .ok_or_else(|| CommandError::NotFound("Task not found".to_string()))?;

    let mut new_headline = old_headline.clone();
    new_headline.title = task.title.clone();
    // IN_PROGRESS/CANCELED aren't in OrgParser's hardcoded keyword config yet
    // (see M2 in the code review) so writing them is safe but a later
    // reparse won't recognize the keyword — it'll be swallowed into the
    // title text instead of `todo_state`.
    new_headline.todo_state = Some(task.state.as_org_keyword().to_string());
    new_headline.tags = task.tags.clone();
    new_headline.priority = task.priority.as_ref().map(Priority::as_char);
    new_headline.properties = task.properties.clone();

    state
        .parser
        .update_headline(&task.file_path, &old_headline, &new_headline)?;

    let parsed = state.parser.parse_file(&task.file_path)?;
    let updated_task = parsed
        .headlines
        .iter()
        .map(|h| Task::new(h, &parsed.file_path))
        .find(|t| t.id == task.id);

    state.store.lock().unwrap().add_tasks_from_file(parsed);

    updated_task.ok_or_else(|| CommandError::NotFound("Task not found after update".to_string()))
}

#[tauri::command]
pub fn update_task(
    task: Task,
    app_handle: AppHandle,
    state: State<AppState>,
) -> Result<Task, CommandError> {
    let updated = update_task_impl(&state, task)?;
    let _ = app_handle.emit(TASKS_CHANGED_EVENT, ());
    Ok(updated)
}

/// Deletes the headline's full subtree from its source file, then reparses
/// that file (rather than just dropping this one task from the store) so
/// every remaining headline's stored range — shifted by the deletion — is
/// refreshed for future edits.
fn delete_task_impl(state: &AppState, task_id: Uuid) -> Result<Task, CommandError> {
    let (removed_task, headline) = {
        let store = state.store.lock().unwrap();
        store
            .get_entry(task_id)
            .cloned()
            .ok_or_else(|| CommandError::NotFound("Task not found".to_string()))?
    };

    state.parser.delete_headline(&removed_task.file_path, &headline)?;

    let parsed = state.parser.parse_file(&removed_task.file_path)?;
    state.store.lock().unwrap().add_tasks_from_file(parsed);

    Ok(removed_task)
}

#[tauri::command]
pub fn delete_task(
    task_id: String,
    app_handle: AppHandle,
    state: State<AppState>,
) -> Result<Task, CommandError> {
    let uuid = Uuid::parse_str(&task_id).map_err(|e| CommandError::Parser(e.to_string()))?;
    let removed = delete_task_impl(&state, uuid)?;
    let _ = app_handle.emit(TASKS_CHANGED_EVENT, ());
    Ok(removed)
}

#[tauri::command]
pub fn list_tasks(
    filter: Option<String>,
    state: State<AppState>,
) -> Result<Vec<Task>, CommandError> {
    let store = state.store.lock().unwrap();
    let tasks = store
        .filter_tasks(|task| {
            if let Some(filter) = &filter {
                task.title.contains(filter)
            } else {
                true
            }
        })
        .into_iter()
        .cloned()
        .collect();
    Ok(tasks)
}

/// Whether `date_str` (a `Task.scheduled`/`.deadline` ISO date) falls within
/// `[start, end]` inclusive. Unparseable/missing dates are never in range.
fn date_in_range(date_str: &str, start: chrono::NaiveDate, end: chrono::NaiveDate) -> bool {
    chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map(|d| d >= start && d <= end)
        .unwrap_or(false)
}

fn get_agenda_range_impl(
    state: &AppState,
    start_date: &str,
    end_date: &str,
) -> Result<Vec<Task>, CommandError> {
    let start = chrono::NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
        .map_err(|e| CommandError::Parser(format!("Invalid start_date: {}", e)))?;
    let end = chrono::NaiveDate::parse_from_str(end_date, "%Y-%m-%d")
        .map_err(|e| CommandError::Parser(format!("Invalid end_date: {}", e)))?;

    let store = state.store.lock().unwrap();
    let tasks = store
        .filter_tasks(|task| {
            task.scheduled
                .as_deref()
                .is_some_and(|d| date_in_range(d, start, end))
                || task
                    .deadline
                    .as_deref()
                    .is_some_and(|d| date_in_range(d, start, end))
        })
        .into_iter()
        .cloned()
        .collect();
    Ok(tasks)
}

#[tauri::command]
pub fn get_agenda_range(
    start_date: String,
    end_date: String,
    state: State<AppState>,
) -> Result<Vec<Task>, CommandError> {
    get_agenda_range_impl(&state, &start_date, &end_date)
}

/// Starts watching `path` (after a synchronous recursive scan for `.org`
/// files to populate the store immediately — see `FileWatcher::add_watched_folder`),
/// persists it to disk so it's restored on next launch, and notifies the
/// frontend that tasks may have changed. Returns the full watched-folder list.
#[tauri::command]
pub fn add_watched_folder(
    path: String,
    app_handle: AppHandle,
    state: State<AppState>,
) -> Result<Vec<String>, CommandError> {
    let mut watcher = state.watcher.lock().unwrap();
    watcher
        .add_watched_folder(Path::new(&path))
        .map_err(|e| CommandError::Store(e.to_string()))?;

    let folders = watcher.watched_folders();
    {
        let mut settings = state.settings.lock().unwrap();
        settings.watched_folders = folders.clone();
        persist_settings(&state, &settings);
    }
    let _ = app_handle.emit(TASKS_CHANGED_EVENT, ());
    Ok(folders)
}

/// Stops watching `path` and drops every task parsed from underneath it,
/// persists the updated folder list, and notifies the frontend. Returns the
/// full watched-folder list.
#[tauri::command]
pub fn remove_watched_folder(
    path: String,
    app_handle: AppHandle,
    state: State<AppState>,
) -> Result<Vec<String>, CommandError> {
    let mut watcher = state.watcher.lock().unwrap();
    watcher.remove_watched_folder(Path::new(&path));

    let folders = watcher.watched_folders();
    {
        let mut settings = state.settings.lock().unwrap();
        settings.watched_folders = folders.clone();
        persist_settings(&state, &settings);
    }
    let _ = app_handle.emit(TASKS_CHANGED_EVENT, ());
    Ok(folders)
}

#[tauri::command]
pub fn get_watched_folders(state: State<AppState>) -> Result<Vec<String>, CommandError> {
    Ok(state.watcher.lock().unwrap().watched_folders())
}

#[tauri::command]
pub fn get_inbox_file(state: State<AppState>) -> Result<Option<String>, CommandError> {
    Ok(state.settings.lock().unwrap().inbox_file.clone())
}

#[tauri::command]
pub fn set_inbox_file(path: String, state: State<AppState>) -> Result<(), CommandError> {
    let mut settings = state.settings.lock().unwrap();
    settings.inbox_file = Some(path);
    persist_settings(&state, &settings);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::task::TodoState;
    use tempfile::tempdir;

    /// An `AppState` wired the same way `lib.rs::run()` wires it (store and
    /// parser shared with the watcher), but with no folders watched — these
    /// tests exercise the command `_impl` functions directly against a
    /// temp-dir file, without needing a running Tauri app.
    fn test_state(settings_path: PathBuf) -> AppState {
        let store = Arc::new(Mutex::new(TaskStore::new()));
        let parser = Arc::new(OrgParser::new());
        let watcher = FileWatcher::new(Arc::clone(&store), Arc::clone(&parser), || {}).unwrap();
        AppState {
            store,
            parser,
            watcher: Arc::new(Mutex::new(watcher)),
            settings: Arc::new(Mutex::new(Settings::default())),
            settings_path,
        }
    }

    #[test]
    fn test_create_task_impl_requires_inbox_file() {
        let dir = tempdir().unwrap();
        let state = test_state(dir.path().join("settings.json"));

        let result = create_task_impl(&state, "New task".to_string());
        assert!(matches!(result, Err(CommandError::Store(_))));
    }

    #[test]
    fn test_create_task_impl_appends_and_returns_persisted_task() {
        let dir = tempdir().unwrap();
        let inbox_path = dir.path().join("inbox.org");
        let state = test_state(dir.path().join("settings.json"));
        state.settings.lock().unwrap().inbox_file =
            Some(inbox_path.to_string_lossy().to_string());

        let task = create_task_impl(&state, "Buy milk".to_string()).unwrap();
        assert_eq!(task.title, "Buy milk");
        assert_eq!(task.state, TodoState::Todo);

        let content = std::fs::read_to_string(&inbox_path).unwrap();
        assert!(content.contains("* TODO Buy milk"));

        // A second create appends rather than overwriting, and the store
        // ends up with both tasks.
        create_task_impl(&state, "Walk dog".to_string()).unwrap();
        let content = std::fs::read_to_string(&inbox_path).unwrap();
        assert!(content.contains("* TODO Buy milk"));
        assert!(content.contains("* TODO Walk dog"));
        assert_eq!(state.store.lock().unwrap().get_all_tasks().len(), 2);
    }

    #[test]
    fn test_update_task_impl_rewrites_title_and_state_preserving_body() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("notes.org");
        std::fs::write(&file_path, "* TODO Task\n  Some notes.\n").unwrap();
        let state = test_state(dir.path().join("settings.json"));

        let parsed = state.parser.parse_file(&file_path).unwrap();
        state
            .store
            .lock()
            .unwrap()
            .add_tasks_from_file(parsed.clone());
        let mut task = Task::new(&parsed.headlines[0], &parsed.file_path);

        task.title = "Renamed task".to_string();
        task.state = TodoState::Done;
        let updated = update_task_impl(&state, task).unwrap();

        assert_eq!(updated.title, "Renamed task");
        assert_eq!(updated.state, TodoState::Done);

        let content = std::fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("DONE Renamed task"));
        assert!(
            content.contains("Some notes."),
            "body text was lost: {content:?}"
        );
    }

    #[test]
    fn test_delete_task_impl_removes_headline_and_refreshes_other_ranges() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("notes.org");
        std::fs::write(&file_path, "* TODO First\n* TODO Second\n* TODO Third\n").unwrap();
        let state = test_state(dir.path().join("settings.json"));

        let parsed = state.parser.parse_file(&file_path).unwrap();
        state
            .store
            .lock()
            .unwrap()
            .add_tasks_from_file(parsed.clone());
        let second_id = Task::new(&parsed.headlines[1], &parsed.file_path).id;

        delete_task_impl(&state, second_id).unwrap();
        assert_eq!(state.store.lock().unwrap().get_all_tasks().len(), 2);

        // Deleting "Second" shifted "Third"'s byte range in the file (and,
        // since it has no `:ID:` property, its sibling-index-derived id —
        // see H2 — since it's now the 2nd top-level headline, not the 3rd).
        // Re-fetch it from the store rather than reuse the pre-delete task,
        // and confirm the refreshed entry is still valid for a further edit
        // — i.e. `delete_task_impl`'s reparse actually updated it, rather
        // than leaving a stale range/id behind.
        let mut third_task = state
            .store
            .lock()
            .unwrap()
            .get_all_tasks()
            .into_iter()
            .find(|t| t.title == "Third")
            .cloned()
            .expect("Third should survive the delete with a refreshed entry");

        third_task.title = "Renamed third".to_string();
        let updated = update_task_impl(&state, third_task).unwrap();
        assert_eq!(updated.title, "Renamed third");

        let content = std::fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("Renamed third"));
        assert!(!content.contains("Second"));
        assert!(content.contains("First"));
    }

    #[test]
    fn test_get_agenda_range_filters_by_scheduled_and_deadline() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("notes.org");
        std::fs::write(
            &file_path,
            "* TODO In range\n  SCHEDULED: <2024-08-05 Mon>\n* TODO Out of range\n  SCHEDULED: <2024-09-01 Sun>\n* TODO Deadline in range\n  DEADLINE: <2024-08-10 Sat>\n* TODO No date\n",
        )
        .unwrap();
        let state = test_state(dir.path().join("settings.json"));
        let parsed = state.parser.parse_file(&file_path).unwrap();
        state.store.lock().unwrap().add_tasks_from_file(parsed);

        let result = get_agenda_range_impl(&state, "2024-08-01", "2024-08-15").unwrap();
        let titles: Vec<&str> = result.iter().map(|t| t.title.as_str()).collect();
        assert_eq!(titles.len(), 2);
        assert!(titles.contains(&"In range"));
        assert!(titles.contains(&"Deadline in range"));
    }

    #[test]
    fn test_get_agenda_range_rejects_unparseable_dates() {
        let dir = tempdir().unwrap();
        let state = test_state(dir.path().join("settings.json"));
        let result = get_agenda_range_impl(&state, "not-a-date", "2024-08-15");
        assert!(matches!(result, Err(CommandError::Parser(_))));
    }
} 