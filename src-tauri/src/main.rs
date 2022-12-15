#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize,Serialize};      
pub mod csv_in;
pub mod csv_out;
pub mod crud;

#[derive(Serialize,Deserialize,Clone)]
struct Useritems{
    name:String,
    id:Option<String>,
    pin:u32
}

#[derive(Serialize,Deserialize)]
struct Users{
    items:Option<Vec<Useritems>>
}
#[derive(Serialize,Deserialize)]
struct SholatTable{
    user:String,
    time:String
}
#[derive(Serialize,Deserialize)]
struct SholatCol{
    items:Option<Vec<SholatTable>>
}


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
async fn parse(host:String,port:u16,path:String) -> String{
    csv_in::testing(path, host, port).await
}
#[tauri::command()]
async fn get_all(host:String,port:u16,path:String) -> String{
    csv_out::get_all_data(host, port, path).await
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,parse,get_all])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
