use csv;
use crate::crud;
use crate::Users;

struct Cluster{
    perusahaan:String,
    devisi:String
}
#[derive(PartialEq, PartialOrd,Eq, Ord)]
struct GetFormat{
    name:String,
    pin:String,
    dhuhur:usize,
    ashar:usize,
    maghrib:usize,
    isya:usize,
    subuh:usize,
    tahajud:usize,
    total:usize
}


fn pin_parse(pin:&str)->Option<Cluster>{
    let mut correct_pin = pin.to_owned();
    if pin.len() < 8 || pin.len() >9{
        return None;
    }else if pin.len() ==8{
        correct_pin = ["0",pin].concat()
    }
    let pr = &correct_pin[0..2].parse::<u16>().unwrap_or(99);
    let dv = &correct_pin[2..4].parse::<u16>().unwrap_or(99);
    Some(Cluster { perusahaan: perusahaan(pr.to_owned()), devisi: devisi(dv.to_owned()) })
}
fn perusahaan(pin:u16)->String{
    match pin {
        1=>"PT. POLOWIJO GOSARI INDONESIA  ( PGI & PG )".to_string(),
        2=>"PT. PUPUK KARYA POLOWIJO  ( PKP )".to_string(),
        3=> "PT. GUNUNG EMAS PUTIH  ( GEP )".to_string(),
        4=> "PT. SARI GUNUNG POLOWIJO  ( SGP )".to_string(),
        5=>"PT. POLOWIJO GRAHA NIAGA  ( PGN )".to_string(),
        6=>"PT. GALASARI GUNUNG SEJAHTERA  ( GGS )".to_string(),
        7=>"PT. GALASARI AGRO NIAGA SEJAHTERA  ( GANIS )".to_string(),
        8=>"PT. DIPO INVESTAMA INDONESIA".to_string(),
        9=>"PT. BUMI SARI TEKNOLOGI".to_string(),
        10=>"PT. INTISARI MEDIA NUSANTARA".to_string(),
        11=>"YAYASAN HARFIN GOSARI".to_string(),
        12=>"MASJID AKBAR MOED'HAR ARIFIN".to_string(),
        13=>"HARFIN MART".to_string(),
        14=>"PT. MAGNESIUM GOSARI INTERNASIONAL  ( MGI )".to_string(),
        15=>"GUEST HOUSE".to_string(),
        _=>"UNIDENTIFIED".to_string()
    }
}
fn devisi(pin:u16)->String{
    match pin {
         1=>"BOD / BOC / STAF AHLI".to_string(),
         2=>"FAT".to_string(),
         3=>"HRGA / LEGAL / SAFETY / RUMAH TANGGA".to_string(),
         4=>"HUMAS / IT / SEKRETARIAT / PA".to_string(),
         5=>"SECURITY / WAKER".to_string(),
         6=>"DRIVER OPERASIONAL / P5".to_string(),
         7=>"PRODUKSI / OPERASIONAL / CHEKEER".to_string(),
         8=>"MAINTENANCE / MEKANIK / WELDER / FUELMAN".to_string(),
         9=>"CLEANING SERVICE / HOUSE KEEPING".to_string(),
         10=>"OPERATOR ALAT BERAT".to_string(),
         11=>"CUSTOMER SERVICE".to_string(),
         12=>"PPIC / PROCUREMENT".to_string(),
         13=>"LABORATORIUM / QA".to_string(),
         14=>"RISET & DEVELOPMENT".to_string(),
         15=>"MARKETING".to_string(),
         16=>"KANTOR JAKARTA".to_string(),
         17=>"HARIAN".to_string(),
         18=>"PRAMUNIAGA".to_string(),
         _=>"UNIDENTIFIED".to_string()
    }
}

pub async fn get_all_data(host:String,port:u16,path:String,start:String,stop:String)->String{
    let con =   crud::Collection{
        host,port
    };
    let user:Users  = serde_json::from_str(&crud::Table::User.list_all(&con, None).await).unwrap();
    if user.items.is_none(){
        return "error".to_string();
    }
    let items = user.items.clone().unwrap();
    let mut writer = csv::Writer::from_path(&path).expect("idk");
    writer.write_record(&["name", "pin","perusahaan","devisi", "duhur", "asyar", "maghrib","isya","subuh","tahajud","total"]).expect("idk");
    let mut csv_mat:Vec<GetFormat> = Vec::new();
    for i in &items{
        println!("Getting {}'s data",i.name.clone().to_owned());
        let id = i.id.to_owned().unwrap();
        let filter = format!("time>={}&&time<={}&&user={}",start,stop,id);
        let dhuhur = crud::Table::Duhur.length(&con, Some(&filter)).await;
        let ashar = crud::Table::Ashar.length(&con, Some(&filter)).await;
        let maghrib = crud::Table::Maghrib.length(&con, Some(&filter)).await;
        let isya = crud::Table::Isya.length(&con, Some(&filter)).await;
        let subuh = crud::Table::Subuh.length(&con, Some(&filter)).await;
        let tahajud = crud::Table::Tahajud.length(&con, Some(&filter)).await;
        let total = dhuhur+ashar+maghrib+isya+subuh+tahajud;
        csv_mat.push(GetFormat { name: i.name.to_owned(),pin: i.pin.to_owned(), dhuhur, ashar, maghrib, isya, subuh, tahajud,total});
    };
    csv_mat.sort_by(|a,b|b.total.cmp(&a.total));
    println!("Writing Filtered and Sorted Data");
    for i in &csv_mat{
        let parsed_pin = pin_parse(&i.pin.as_str());
        let pt;
        let dv;
        match parsed_pin{
            Some(d)=>{
                dv=d.devisi;
                pt=d.perusahaan;
            }
            None=>{
                dv="UNIDENTIFIED".to_string();
                pt="UNIDENTIFIED".to_string()
            }
        };
        writer.write_record(
            &[
                i.name.to_owned(),
                i.pin.to_owned(),
                pt,
                dv,
                i.dhuhur.to_string(),
                i.ashar.to_string(),
                i.maghrib.to_string(),
                i.isya.to_string(),
                i.subuh.to_string(),
                i.tahajud.to_string(),
                i.total.to_string()
            ]
        ).expect("idk");
    }
    writer.flush().expect("idk");
    "success".to_string()
}


