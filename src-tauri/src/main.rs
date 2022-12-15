#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]      
pub mod csv_in;
pub mod crud;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
async fn parse(host:String,port:u16,path:String) -> String{
    csv_in::testing(path, host, port).await
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,parse])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
