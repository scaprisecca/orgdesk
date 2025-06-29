// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use orgdesk::{
    commands::{self, AppState},
    parser::org_parser::OrgParser,
    store::task_store::TaskStore,
    watcher::file_watcher::FileWatcher,
};
use std::sync::{Arc, Mutex};

fn main() {
    let task_store = Arc::new(Mutex::new(TaskStore::new()));
    let parser = Arc::new(OrgParser::new());

    let mut watcher =
        FileWatcher::new(Arc::clone(&task_store), Arc::clone(&parser)).expect("Failed to create watcher");
    watcher
        .watch_directory(&".".into())
        .expect("Failed to watch directory");

    tauri::Builder::default()
        .manage(AppState {
            store: task_store,
            parser,
        })
        .invoke_handler(tauri::generate_handler![
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
