// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
use tauri::Manager;
use xrchat::interaction;
use xrchat::connect;
fn main() {
    tauri::Builder::default().
        setup(|app| {
            let window = app.get_window("main").unwrap();
            app.listen_global("login", move |event| {
                connect::tcp_connect(window.clone());
                window.emit("login_result", "success").unwrap();
            });
        Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet,interaction::get_session_list,interaction::get_friend_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
