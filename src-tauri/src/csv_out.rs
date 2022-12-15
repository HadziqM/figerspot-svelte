use csv;
use crate::crud;
use crate::Users;
use crate::SholatCol;


impl crud::Table{
    async fn get_sholat_count(&self,con:&crud::Collection,id:&str)->usize{
        let collection:SholatCol = serde_json::from_str(&self
            .list_all(con,Some(format!("filter=(user='{}')",id).as_str())).await).unwrap();
        collection.items.unwrap().len()
    }
}


pub async fn get_all_data(host:String,port:u16,path:String)->String{
    let con =   crud::Collection{
        host,port
    };
    let user:Users  = serde_json::from_str(&crud::Table::User.list_all(&con, None).await).unwrap();
    if user.items.is_none(){
        return "error".to_string();
    }
    let items = user.items.clone().unwrap();
    let mut writer = csv::Writer::from_path(&path).expect("idk");
    writer.write_record(&["name", "pin", "duhur", "asyar", "maghrib","isya","subuh","total"]).expect("idk");
    for i in &items{
        println!("Getting {}'s data",i.name.clone().to_owned());
        let id = i.id.to_owned().unwrap();
        let dhuhur = crud::Table::Duhur.get_sholat_count(&con, &id).await;
        let ashar = crud::Table::Ashar.get_sholat_count(&con, &id).await;
        let maghrib = crud::Table::Maghrib.get_sholat_count(&con, &id).await;
        let isya = crud::Table::Isya.get_sholat_count(&con, &id).await;
        let subuh = crud::Table::Subuh.get_sholat_count(&con, &id).await;
        let total = dhuhur+ashar+maghrib+isya+subuh;
        writer.write_record(&[
            i.name.to_owned(),
            i.pin.to_owned().to_string(),
            dhuhur.to_string(),
            ashar.to_string(),
            maghrib.to_string(),
            isya.to_string(),
            subuh.to_string(),
            total.to_string()
        ]).expect("please work");
    };
    writer.flush().expect("idk");
    "success".to_string()
}


