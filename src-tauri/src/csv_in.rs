
use chrono::prelude::*;
use crate::crud;
use crate::Users;
use crate::Useritems;
use crate::SholatTable;


#[derive(PartialEq,Clone)]
enum Sholat{
    Duhur,
    Ashar,
    Maghrib,
    Isya,
    Subuh,
    Tahajud
}
struct Filter{
    d_s:u32,
    d_f:u32,
    a_s:u32,
    a_f:u32,
    m_s:u32,
    m_f:u32,
    i_s:u32,
    i_f:u32,
    s_s:u32,
    s_f:u32,
    t_s:u32,
    t_f:u32
}
#[derive(PartialEq)]
struct Hold{
    name:String,
    day:(u32,u32),
    sholat:Sholat
}
impl Sholat {
    async fn create_collection(&self,con:&crud::Collection,sholat_json:&str){
        match self {
            Sholat::Duhur => crud::Table::Duhur.create(con,sholat_json).await,
            Sholat::Ashar => crud::Table::Ashar.create(con,sholat_json).await,
            Sholat::Maghrib => crud::Table::Maghrib.create(con,sholat_json).await,
            Sholat::Isya => crud::Table::Isya.create(con, sholat_json).await,
            Sholat::Subuh => crud::Table::Subuh.create(con, sholat_json).await,
            Sholat::Tahajud => crud::Table::Tahajud.create(con, sholat_json).await
        };
    }
    fn get_name(&self)->String{
        match self {
            Sholat::Isya=>"isya".to_string(),
            Sholat::Duhur=>"duhur".to_string(),
            Sholat::Ashar=>"asyar".to_string(),
            Sholat::Subuh=>"shubuh".to_string(),
            Sholat::Maghrib=>"maghrib".to_string(),
            Sholat::Tahajud=>"tahajud".to_string()
        }
    }
}
impl Filter {
    fn parse_sholat(&self,input:u32)->Option<Sholat>{
       if input>self.d_s && input<self.d_f{
            return Some(Sholat::Duhur);
        }else if input>self.a_s && input<self.a_f{
            return Some(Sholat::Ashar);
        }else if input>self.m_s && input<self.m_f{
            return Some(Sholat::Maghrib);
        }else if input>self.i_s && input<self.i_f{
            return Some(Sholat::Isya);
        }else if input>self.s_s && input<self.s_f{
            return Some(Sholat::Subuh);
        }else if input>self.t_s && input<self.t_f{
            return Some(Sholat::Tahajud);
        }else{
            return None;
        }
    }
}

fn parse_csv(input:&str)->Vec<Vec<String>>{
    input.lines().enumerate().filter(|(i,_)|i.to_owned()!= 0 && i.to_owned()!= 1).map(|(_,e)|e).map(|j|j.split(",").map(|k|k.to_owned()).collect::<Vec<_>>()).collect::<Vec<_>>()
}
fn parse_time(input:&str)->DateTime<FixedOffset>{
    DateTime::parse_from_str(input, "%d-%m-%Y %H:%M:%S %z").unwrap()
}

fn parse_sholat(input:u32)->Option<Sholat>{
    if input>660 && input<720{          //11:00 - 12:00
        return Some(Sholat::Duhur);
    }else if input>870 && input<930 {   //14:30 - 15:30
        return Some(Sholat::Ashar);
    }else if input>1030 && input<1090 { //17:10 - 18:10
        return Some(Sholat::Maghrib);
    }else if input>1120 && input<1180 { //18:40 - 19:40
        return Some(Sholat::Isya);
    }else if input>190 && input<250 {   //03:10 - 04:10
        return Some(Sholat::Subuh);
    }else{
        return None;
    }
}

pub async fn testing(path:String,host:String,port:u16,
    d_s:u32,d_f:u32,a_s:u32,a_f:u32,m_s:u32,m_f:u32,
    i_s:u32,i_f:u32,s_s:u32,s_f:u32,t_s:u32,t_f:u32
    )-> String{
    let con = crud::Collection{host,port};
    let filter = Filter{d_s,d_f,a_s,a_f,m_s,m_f,i_s,i_f,s_s,s_f,t_s,t_f};
    let user:Users = serde_json::from_str(&crud::Table::User.list_all(&con, None).await).unwrap();
    if user.items.is_none(){
        return "error".to_string();
    }
    let mut items = user.items.unwrap();
    let file = parse_csv(&std::fs::read_to_string(&path).unwrap());
    let mut holder:Vec<Hold> = Vec::new();
    for i in &file{
        let time_data = parse_time(&[&i[0]," +07:00"].concat());
        let sholat = filter.parse_sholat(time_data.hour()*60+time_data.minute());
        let mut test = "invalid".to_string();
        let mut test2 = "invalid".to_string();
        if sholat.is_some(){
            test = sholat.clone().unwrap().get_name();
            let day = (time_data.month(),time_data.day());
            let new_struct = Hold{
                name:i[5].to_owned(),day,sholat:sholat.clone().unwrap()
            };
            test2 = "double".to_string();
            if !holder.contains(&new_struct){
                holder.push(new_struct);
                test2 = "valid".to_string();
                let id_user:String;
                let filtered = items.iter().filter(|&e|e.name==i[5].to_owned()).collect::<Vec<_>>();
                if filtered.len()==0{
                    let mut new_user = Useritems{
                        name:i[5].to_owned(),
                        id:None,
                        pin:i[3].to_owned()
                    };
                    let new_id:Useritems = serde_json::from_str(&crud::Table::User
                        .create(&con, &serde_json::to_string(&new_user).unwrap()).await).unwrap();
                    id_user  = new_id.clone().id.unwrap().to_owned();
                    new_user.id = Some(new_id.id.unwrap().to_owned());
                    items.push(new_user);
                }else{
                    id_user = filtered[0].id.to_owned().unwrap();
                }
                let data_sholat = SholatTable{
                    user: id_user.to_owned(),
                    time:format!("{}",time_data.format("%+")),
                    code:format!("{}{}",&id_user,time_data.format("%+"))
                };
                let sholat_json = serde_json::to_string(&data_sholat).unwrap();
                sholat.unwrap().create_collection(&con, &sholat_json).await
           }
        }
        println!("{} at {} become {}:{} hasil filter = {}, status = {}",&i[5],&i[0],time_data.hour(),time_data.minute(),test,test2);
    }
  "success".to_string()
}
