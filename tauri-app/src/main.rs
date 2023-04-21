// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ogts_backend::child::ChildService;
use ogts_common::model::Child;
use ogts_common::ListOptions;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/*
#[tauri::command]
async fn create_child(child: Child) -> anyhow::Result<()> {
    ChildService::new().create(child).await
}

#[tauri::command]
async fn list_child(opts: ListOptions) -> anyhow::Result<Vec<Child>> {
    ChildService::new().list(opts).await
}
*/

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        //.invoke_handler(tauri::generate_handler![greet, create_child, list_child])
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
