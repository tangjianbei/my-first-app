// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

mod tray;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        //.system_tray(tauri::SystemTray::default())
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
