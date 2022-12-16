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
    pin:String,
}
#[derive(Serialize,Deserialize)]
struct Users{
    items:Option<Vec<Useritems>>
}
#[derive(Serialize,Deserialize)]
struct SholatTable{
    user:String,
    time:String,
    code:String
}
#[derive(Serialize,Deserialize)]
struct SholatCol{
    items:Option<Vec<SholatTable>>
}
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
async fn parse(host:String,port:u16,path:String,
    d_s:u32,d_f:u32,a_s:u32,a_f:u32,m_s:u32,m_f:u32,
    i_s:u32,i_f:u32,s_s:u32,s_f:u32,t_s:u32,t_f:u32
) -> String{
    csv_in::testing(path,host,port,d_s,d_f,a_s,a_f,m_s,m_f,i_s,i_f,s_s,s_f,t_s,t_f).await
}
#[tauri::command()]
async fn get_all(host:String,port:u16,path:String,start:String,stop:String) -> String{
    println!("{}",&path);
    csv_out::get_all_data(host, port, path,start,stop).await
}
#[tauri::command()]
async fn remove(host:String,port:u16)->String{
    let con = crud::Collection{host,port};
    crud::Table::User.delete_all(&con, None).await
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,parse,get_all,remove])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
