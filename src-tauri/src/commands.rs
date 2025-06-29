// Tauri command handlers for IPC communication with React frontend
// Contains all the API endpoints as Tauri commands

// TODO: Implement command handlers for:
// - createTask
// - updateTask  
// - deleteTask
// - listTasks
// - getAgendaRange

// Placeholder function to ensure compilation
#[tauri::command]
pub fn hello_from_rust() -> String {
    "Hello from Rust backend!".to_string()
} 