// Module declarations for OrgDesk backend
pub mod commands;
pub mod models;
pub mod parser;
pub mod store;
pub mod watcher;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_log::Builder::new().build())
    .invoke_handler(tauri::generate_handler![
      commands::hello_from_rust,
      commands::parse_org_content,
      commands::parse_org_file
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
