// Tauri command handlers for IPC communication with React frontend
// Contains all the API endpoints as Tauri commands

use crate::{
    models::task::{Task, TodoState},
    parser::org_parser::{OrgParser, ParserError},
    store::task_store::TaskStore,
};
use serde::Serialize;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tauri::State;

// TODO: Implement command handlers for:
// - createTask
// - updateTask  
// - deleteTask
// - listTasks
// - getAgendaRange

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
}

#[tauri::command]
pub fn create_task(
    title: String,
    state: State<AppState>,
) -> Result<Task, CommandError> {
    let mut new_task = Task {
        id: uuid::Uuid::new_v4(),
        title,
        state: TodoState::Todo,
        tags: vec![],
        priority: None,
        scheduled: None,
        deadline: None,
        properties: std::collections::HashMap::new(),
        file_path: "new_tasks.org".to_string(),
    };
    
    // This is a simplified implementation. In a real app, we would
    // append the new task to an existing or new org file.
    // For now, we'll just add it to the in-memory store.
    let mut store = state.store.lock().unwrap();
    // This is not ideal, we should have a proper method for this.
    // store.add_task(new_task.clone());
    
    Ok(new_task)
}

#[tauri::command]
pub fn update_task(task: Task, state: State<AppState>) -> Result<Task, CommandError> {
    let mut store = state.store.lock().unwrap();
    let old_task = store
        .update_task(task.clone())
        .ok_or_else(|| CommandError::NotFound("Task not found".to_string()))?;

    // In a real app, we would also need to update the org file
    // let parser = state.parser.lock().unwrap();
    // let old_headline = ...; // We need to get the original headline
    // let new_headline = ...; // We need to convert the task back to a headline
    // parser.update_headline(&task.file_path, &old_headline, &new_headline)?;

    Ok(old_task)
}

#[tauri::command]
pub fn delete_task(task_id: String, state: State<AppState>) -> Result<Task, CommandError> {
    let mut store = state.store.lock().unwrap();
    let uuid = uuid::Uuid::parse_str(&task_id).map_err(|e| CommandError::Parser(e.to_string()))?;
    let removed_task = store
        .remove_task(uuid)
        .ok_or_else(|| CommandError::NotFound("Task not found".to_string()))?;

    // In a real app, we would also need to update the org file
    // let parser = state.parser.lock().unwrap();
    // ...

    Ok(removed_task)
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

#[tauri::command]
pub fn get_agenda_range(
    _start_date: String,
    _end_date: String,
    _state: State<AppState>,
) -> Result<Vec<Task>, CommandError> {
    // TODO: Implement date parsing and filtering
    Ok(vec![])
} 