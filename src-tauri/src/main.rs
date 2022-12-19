#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize,Serialize};      
pub mod csv_in;
pub mod csv_out;
pub mod crud;
#[derive(Serialize,Deserialize,Clone)]
struct MachineItems{
    id:Option<String>,
    name:String
}
#[derive(Serialize,Deserialize,Clone)]
struct Machine{
    items:Option<Vec<MachineItems>>
}
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
    code:String,
    machine:String
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
    timer:String) -> String{
    println!("invoked");
    let k = timer.split(',').map(|e|e.to_owned()).collect::<Vec<_>>();
    csv_in::testing(path,host,port
        ,k[0].to_owned(),k[1].to_owned(),k[2].to_owned(),k[3].to_owned(),k[4].to_owned()
        ,k[5].to_owned(),k[6].to_owned(),k[7].to_owned(),k[8].to_owned(),k[9].to_owned()
        ,k[10].to_owned(),k[11].to_owned()).await
}
#[tauri::command()]
async fn get_all(host:String,port:u16,path:String,start:String,stop:String,machine:String) -> String{
    println!("{}",&path);
    csv_out::get_all_data(host, port, path,start,stop,machine).await
}
#[tauri::command()]
async fn remove(host:String,port:u16)->String{
    let con = crud::Collection{host,port};
    crud::Table::User.delete_all(&con).await;
    crud::Table::Machine.delete_all(&con).await
}
#[tauri::command()]
async fn get_machine(host:String,port:u16)->String{
    let con = crud::Collection{host,port};
    crud::Table::Machine.list_all(&con, None).await
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,parse,get_all,remove,get_machine])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
