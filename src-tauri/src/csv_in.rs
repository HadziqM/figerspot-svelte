
use chrono::prelude::*;
use crate::crud;
use crate::Users;
use crate::Useritems;
use crate::SholatTable;
use crate::Machine;
use crate::MachineItems;


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
    d_s:String,
    d_f:String,
    a_s:String,
    a_f:String,
    m_s:String,
    m_f:String,
    i_s:String,
    i_f:String,
    s_s:String,
    s_f:String,
    t_s:String,
    t_f:String
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
fn parse_time(input:&str)->u32{
    let idk = input.split(":").map(|e|e.parse::<u32>().unwrap()).collect::<Vec<_>>();
    idk[0]*60+idk[1]
}
impl Filter {
    fn parse_sholat(&self,input:u32)->Option<Sholat>{
       if input>parse_time(&self.d_s) && input<parse_time(&self.d_f){
            return Some(Sholat::Duhur);
        }else if input>parse_time(&self.a_s) && input<parse_time(&self.a_f){
            return Some(Sholat::Ashar);
        }else if input>parse_time(&self.m_s) && input<parse_time(&self.m_f){
            return Some(Sholat::Maghrib);
        }else if input>parse_time(&self.i_s) && input<parse_time(&self.i_f){
            return Some(Sholat::Isya);
        }else if input>parse_time(&self.s_s) && input<parse_time(&self.s_f){
            return Some(Sholat::Subuh);
        }else if input>parse_time(&self.t_s) && input<parse_time(&self.t_f){
            return Some(Sholat::Tahajud);
        }else{
            return None;
        }
    }
}

fn parse_csv(input:&str)->Vec<Vec<String>>{
    input.lines().enumerate().filter(|(i,_)|i.to_owned()!= 0 && i.to_owned()!= 1).map(|(_,e)|e).map(|j|j.split(",").map(|k|k.to_owned()).collect::<Vec<_>>()).collect::<Vec<_>>()
}
fn parse_one_time(input:&str)->DateTime<FixedOffset>{
    DateTime::parse_from_str(input, "%d-%m-%Y %H:%M:%S %z").unwrap()
}

pub async fn testing(path:String,host:String,port:u16,
    d_s:String,d_f:String,a_s:String,a_f:String,m_s:String,m_f:String,
    i_s:String,i_f:String,s_s:String,s_f:String,t_s:String,t_f:String
    )-> String{
    println!("called");
    let con = crud::Collection{host,port};
    let filter = Filter{d_s,d_f,a_s,a_f,m_s,m_f,i_s,i_f,s_s,s_f,t_s,t_f};
    let user:Users = serde_json::from_str(&crud::Table::User.list_all(&con, None).await).unwrap();
    if user.code.is_some(){
        return "error".to_string();
    }
    let mut items = user.items.unwrap_or(Vec::new());
    let machine:Machine = serde_json::from_str(&crud::Table::Machine.list_all(&con, None).await).unwrap();
    let mut machine_items = machine.items.unwrap_or(Vec::new());
    let file = parse_csv(&std::fs::read_to_string(&path).unwrap());
    let mut holder:Vec<Hold> = Vec::new();
    for i in &file{
        let time_data = parse_one_time(&[&i[0]," +07:00"].concat());
        let sholat = filter.parse_sholat(time_data.hour()*60+time_data.minute());
        let mut name = i[5].to_owned();
        if name.chars().next()==Some('"'){
            let mut j:usize = 6;
            loop{
                name.push(',');
                name.push_str(i[j].to_owned().as_str());
                if name.chars().last()==Some('"'){
                    break;
                }
                j += 1;
            }
            name = name.chars().filter(|&e|e!='"').collect::<String>();
        };
        let mut test = "invalid".to_string();
        let mut test2 = "invalid".to_string();
        if sholat.is_some(){
            test = sholat.clone().unwrap().get_name();
            let day = (time_data.month(),time_data.day());
            let new_struct = Hold{
                name:name.to_owned(),day,sholat:sholat.clone().unwrap()
            };
            test2 = "double".to_string();
            if !holder.contains(&new_struct){
                holder.push(new_struct);
                test2 = "valid".to_string();
                let id_user:String;
                let id_machine:String;
                let filtered = items.iter().filter(|&e|e.name==name.to_owned()).collect::<Vec<_>>();
                let second_filtered = machine_items.iter().filter(|&e|e.name==i[i.len()-1].to_owned()).collect::<Vec<_>>();
                if filtered.len()==0{
                    let mut new_user = Useritems{
                        name:name.to_owned(),
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
                if second_filtered.len()==0{
                    let mut new_machine = MachineItems{name:i[i.len()-1].to_owned(),id:None};
                    let new_id:MachineItems= serde_json::from_str(&crud::Table::Machine
                        .create(&con, &serde_json::to_string(&new_machine).unwrap()).await).unwrap();
                    id_machine = new_id.clone().id.unwrap().to_owned();
                    new_machine.id=Some(new_id.id.unwrap().to_owned());
                    machine_items.push(new_machine);
                }else{
                    id_machine = second_filtered[0].id.to_owned().unwrap();
                }
                let data_sholat = SholatTable{
                    user: id_user.to_owned(),
                    machine:id_machine.to_owned(),
                    time:format!("{}",time_data.format("%+")),
                    code:format!("{}{}",&id_user,time_data.format("%+"))
                };
                let sholat_json = serde_json::to_string(&data_sholat).unwrap();
                sholat.unwrap().create_collection(&con, &sholat_json).await
           }
        }
        println!("{} at {} become {}:{} hasil filter = {}, status = {}",&name,&i[0],time_data.hour(),time_data.minute(),test,test2);
    }
  "success".to_string()
}
