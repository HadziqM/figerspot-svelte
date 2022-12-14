
use chrono::prelude::*;
use crate::crud;
use serde::{Serialize,Deserialize};


#[derive(PartialEq)]
enum Sholat{
    Duhur,
    Ashar,
    Maghrib,
    Isya,
    Subuh
}
#[derive(PartialEq)]
struct Hold{
    name:String,
    day:(u32,u32),
    sholat:Sholat
}
#[derive(Serialize,Deserialize)]
struct Useritems{
    name:String,
    id:Option<String>,
    pin:u32
}

#[derive(Serialize,Deserialize)]
struct Users{
    items:Option<Vec<Useritems>>
}

fn parse_csv(input:&str)->Vec<Vec<String>>{
    input.lines().enumerate().filter(|(i,_)|i.to_owned()!= 0 && i.to_owned()!= 1).map(|(_,e)|e).map(|j|j.split(",").map(|k|k.to_owned()).collect::<Vec<_>>()).collect::<Vec<_>>()
}

fn parse_time(input:&str)->DateTime<FixedOffset>{
    DateTime::parse_from_str(input, "%d-%m-%Y %H-%M-%S").unwrap()
}

fn parse_sholat(input:u32)->Option<Sholat>{
    if input>660 && input<720{
        return Some(Sholat::Duhur);
    }else if input>870 && input<930 {
        return Some(Sholat::Ashar);
    }else if input>1030 && input<1090 {
        return Some(Sholat::Maghrib);
    }else if input>1120 && input<1180 {
        return Some(Sholat::Isya);
    }else if input>190 && input<250 {
        return Some(Sholat::Subuh);
    }else{
        return None;
    }
}

#[tauri::command]
pub async fn testing(path:String,host:String,port:u16)->String{
    let con = crud::Collection{
        host,port
    };
    let user:Users = serde_json::from_str(&crud::Table::User.list_all(&con, None).await).unwrap();
    if user.items.is_none(){
        return "error".to_string();
    }
    let items = &user.items.unwrap();
    let file = parse_csv(&std::fs::read_to_string(&path).unwrap());
    let mut holder:Vec<Hold> = Vec::new();
    for i in &file{
        let time = parse_time(&i[0]);
        let sholat = parse_sholat(time.hour()*time.minute());
        if sholat.is_some(){
            let day = (time.month(),time.day());
            let new_struct = Hold{
                name:i[2].to_owned(),day,sholat:sholat.unwrap()
            };
            let mut id = String::new();
            if !holder.contains(&new_struct){
                holder.push(new_struct);
                let count = items.iter()
                    .filter(|&e|e.name==i[2].to_owned()).count();
                if count==0{
                    let new_user = Useritems{
                        name:i[2].to_owned(),
                        id:None,
                        pin:i[3].parse::<u32>().unwrap()
                    };
                    let new_id:Useritems = serde_json::from_str(&crud::Table::User
                        .create(&con, &serde_json::to_string(&new_user).unwrap()).await).unwrap();
                    id  = new_id.id.unwrap().to_owned();
                }
           }
        }
    }
  "success".to_string()
}
