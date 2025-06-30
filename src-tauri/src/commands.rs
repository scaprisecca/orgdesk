// Tauri command handlers for IPC communication with React frontend
// Contains all the API endpoints as Tauri commands

use crate::{
    models::task::{Task, TodoState},
    parser::org_parser::{OrgHeadline, OrgParser, ParsedOrgFile, ParserError},
    store::task_store::TaskStore,
};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tauri::State;

#[derive(Debug, Serialize)]
pub struct FsNode {
    name: String,
    path: PathBuf,
    is_dir: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<FsNode>>,
}

fn read_dir_recursive(path: &Path) -> Result<Vec<FsNode>, String> {
    let mut children = vec![];
    for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let is_dir = path.is_dir();

        let node_children = if is_dir {
            Some(read_dir_recursive(&path)?)
        } else {
            None
        };

        if is_dir || name.ends_with(".org") {
            children.push(FsNode {
                name,
                path,
                is_dir,
                children: node_children,
            });
        }
    }
    children.sort_by(|a, b| {
        if a.is_dir && !b.is_dir {
            std::cmp::Ordering::Less
        } else if !a.is_dir && b.is_dir {
            std::cmp::Ordering::Greater
        } else {
            a.name.cmp(&b.name)
        }
    });
    Ok(children)
}

#[tauri::command]
pub fn read_fs(dir: String) -> Result<FsNode, String> {
    let path = Path::new(&dir);
    let name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let children = read_dir_recursive(path)?;
    Ok(FsNode {
        name,
        path: path.to_path_buf(),
        is_dir: true,
        children: Some(children),
    })
}

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
    let new_task = Task {
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
    let _store = state.store.lock().unwrap();
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

fn find_org_files_recursively(path: &Path, org_files: &mut Vec<PathBuf>) -> Result<(), String> {
    if path.is_dir() {
        for entry in std::fs::read_dir(path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if path.is_dir() {
                find_org_files_recursively(&path, org_files)?;
            } else if OrgParser::is_org_file(&path) {
                org_files.push(path);
            }
        }
    }
    Ok(())
}

#[tauri::command]
pub fn list_tasks(
    filter: Option<String>,
    watched_folders: Vec<String>,
    state: State<AppState>,
) -> Result<Vec<Task>, CommandError> {
    let parser = &state.parser;
    let mut all_tasks = Vec::new();
    let mut org_files = Vec::new();

    for folder in watched_folders {
        find_org_files_recursively(Path::new(&folder), &mut org_files)
            .map_err(|e| CommandError::Store(e))?;
    }

    org_files.sort();
    org_files.dedup();

    for path in org_files {
        if let Ok(parsed_file) = parser.parse_file(&path) {
            let tasks: Vec<Task> = parsed_file
                .headlines
                .iter()
                .map(|headline| {
                    let mut task: Task = headline.into();
                    task.file_path = parsed_file.file_path.clone();
                    task
                })
                .collect();
            all_tasks.extend(tasks);
        }
    }

    if let Some(filter_text) = filter {
        all_tasks.retain(|task| task.title.contains(&filter_text));
    }

    Ok(all_tasks)
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

#[tauri::command]
pub fn update_watched_folders(folders: Vec<String>) -> Result<(), String> {
    // For now, we'll just print the folders to the console.
    // In a future step, we'll use this to update the file watcher.
    println!("Received watched folders: {:?}", folders);
    Ok(())
}

#[tauri::command]
pub fn get_file_content(path: String) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| e.to_string())
} 