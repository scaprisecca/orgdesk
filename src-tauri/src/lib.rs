// Module declarations for OrgDesk backend
pub mod commands;
pub mod models;
pub mod parser;
pub mod settings;
pub mod store;
pub mod watcher;

use commands::AppState;
use parser::org_parser::OrgParser;
use settings::Settings;
use std::path::Path;
use std::sync::{Arc, Mutex};
use store::task_store::TaskStore;
use tauri::{Emitter, Manager};
use watcher::file_watcher::{FileWatcher, TASKS_CHANGED_EVENT};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let task_store = Arc::new(Mutex::new(TaskStore::new()));
    let parser = Arc::new(OrgParser::new());

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            let app_handle = app.handle().clone();
            let mut watcher = FileWatcher::new(
                Arc::clone(&task_store),
                Arc::clone(&parser),
                move || {
                    let _ = app_handle.emit(TASKS_CHANGED_EVENT, ());
                },
            )
            .expect("Failed to create watcher");

            let settings_path = app
                .path()
                .app_config_dir()
                .expect("Failed to resolve app config dir")
                .join("settings.json");

            let settings = Settings::load(&settings_path);
            for folder in &settings.watched_folders {
                if let Err(e) = watcher.add_watched_folder(Path::new(folder)) {
                    log::error!("Failed to restore watched folder {}: {:?}", folder, e);
                }
            }

            app.manage(AppState {
                store: task_store.clone(),
                parser: parser.clone(),
                watcher: Arc::new(Mutex::new(watcher)),
                settings_path,
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::hello_from_rust,
            commands::parse_org_content,
            commands::parse_org_file,
            commands::create_task,
            commands::update_task,
            commands::delete_task,
            commands::list_tasks,
            commands::get_agenda_range,
            commands::add_watched_folder,
            commands::remove_watched_folder,
            commands::get_watched_folders
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
