// Module declarations for OrgDesk backend
pub mod commands;
pub mod models;
pub mod parser;
pub mod store;
pub mod watcher;

use commands::AppState;
use parser::org_parser::OrgParser;
use store::task_store::TaskStore;
use std::sync::{Arc, Mutex};
use watcher::file_watcher::FileWatcher;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let task_store = Arc::new(Mutex::new(TaskStore::new()));
    let parser = Arc::new(OrgParser::new());

    let mut watcher =
        FileWatcher::new(Arc::clone(&task_store), Arc::clone(&parser)).expect("Failed to create watcher");
    watcher
        .watch_directory(&".".into())
        .expect("Failed to watch directory");

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .manage(AppState {
            store: task_store,
            parser,
        })
        .invoke_handler(tauri::generate_handler![
            commands::hello_from_rust,
            commands::parse_org_content,
            commands::parse_org_file,
            commands::create_task,
            commands::update_task,
            commands::delete_task,
            commands::list_tasks,
            commands::get_agenda_range
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
