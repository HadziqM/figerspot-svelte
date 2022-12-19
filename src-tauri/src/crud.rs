use std::fs;

use reqwest;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize,Debug)]
struct Items {
    id: String,
}
#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
struct Length {
    total_items: Option<usize>,
    items:Option<Vec<Items>>
}
fn construct_headers() -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    headers
}
pub fn url_parser(url:&str)->String{
    let mut out:Vec<char> = Vec::new();
    for i in url.chars(){
        let parsed = match i {
            '&'=>"%26".to_string(),
            '$'=>"%24".to_string(),
            '+'=>"%2B".to_string(),
            ','=>"%2C".to_string(),
            '/'=>"%2F".to_string(),
            ':'=>"%3A".to_string(),
            ';'=>"%3B".to_string(),
            '='=>"%3D".to_string(),
            '?'=>"%3F".to_string(),
            '@'=>"%40".to_string(),
            ' '=>"%20".to_string(),
            '!'=>"%21".to_string(),
            '"'=>"%22".to_string(),
            '#'=>"%23".to_string(),
            '%'=>"%25".to_string(),
            '\''=>"%27".to_string(),
            '('=>"%28".to_string(),
            ')'=>"%29".to_string(),
            '.'=>"%2E".to_string(),
            '<'=>"%3C".to_string(),
            '>'=>"%3E".to_string(),
            _=>i.to_string()
        };
        for j in parsed.chars(){
            out.push(j);
        };
    };
    out.iter().collect::<String>()
}
pub struct Collection {
    pub(crate) host: String,
    pub(crate) port: u16,
}
pub enum Table {
    User,
    Duhur,
    Ashar,
    Maghrib,
    Isya,
    Subuh,
    Tahajud,
    Machine
}
impl Table {
    fn get_string(&self) -> String {
        match self {
            Table::User => "user".to_string(),
            Table::Duhur => "duhur".to_string(),
            Table::Ashar => "ashar".to_string(),
            Table::Maghrib => "maghrib".to_string(),
            Table::Isya => "isya".to_string(),
            Table::Subuh => "subuh".to_string(),
            Table::Tahajud=>"tahajud".to_string(),
            Table::Machine=>"machine".to_string(),
        }
    }
    fn url_struct(&self, con: &Collection) -> String {
        format!(
            "{}:{}/api/collections/{}/records",
            &con.host,
            &con.port,
            &self.get_string()
        )
    }
    pub async fn list(&self, con: &Collection, param: Option<&str>) -> String {
        let mut url = self.url_struct(con);
        if param.is_some() {
            url.push_str(["?", param.unwrap()].concat().as_str())
        }
        let client = reqwest::Client::new();
        match client.get(url).send().await {
            Ok(res) => res.text().await.unwrap_or("no message".to_string()),
            Err(_) => "{\"error\":400}".to_string(),
        }
    }
    pub async fn select(&self, con: &Collection, id: &str) -> String {
        let url = [&self.url_struct(con), "/", id].concat();
        let client = reqwest::Client::new();
        match client.get(&url).send().await {
            Ok(result) => result.text().await.unwrap_or("no message".to_string()),
            Err(_error) => String::from("{\"error\":400}"),
        }
    }
    pub async fn create(&self, con: &Collection, data: &str) -> String {
        let client = reqwest::Client::new();
        match client
            .post(&self.url_struct(con))
            .headers(construct_headers())
            .body(data.to_owned())
            .send()
            .await
        {
            Ok(result) => result.text().await.unwrap_or("no message".to_string()),
            Err(_error) => String::from("{\"error\":400}"),
        }
    }
    pub async fn update(&self, con: &Collection, id: &str, data: &str) -> String {
        let url = [&self.url_struct(con), "/", id].concat();
        let client = reqwest::Client::new();
        match client
            .patch(&url)
            .headers(construct_headers())
            .body(data.to_owned())
            .send()
            .await
        {
            Ok(result) => result.text().await.unwrap_or("no message".to_string()),
            Err(_error) => String::from("{\"error\":400}"),
        }
    }
    pub async fn delete(&self, con: &Collection, id: &str) -> String {
        let url = [&self.url_struct(con), "/", id].concat();
        let client = reqwest::Client::new();
        match client
            .delete(&url)
            .headers(construct_headers())
            .send()
            .await
        {
            Ok(result) => result.text().await.unwrap_or("no message".to_string()),
            Err(_error) => String::from("{\"error\":400}"),
        }
    }
    pub async fn length(&self,con:&Collection,filter:Option<&str>)->usize{
        let param = match filter {
            Some(d)  => url_parser(["perPage=1","&&filter=(",d,")"].concat().as_str()),
            None =>"perPage=1".to_string()
        };
        let len_now:Length = serde_json::from_str(&self.list(con, Some(&param)).await).unwrap();
        match len_now.total_items{
            Some(d)=>d,
            None=>0
        }
    }
    pub async fn list_all(&self, con: &Collection, param: Option<&str>) -> String {
        let now = &self.length(con, None).await;
        println!("{}",now);
        if now.to_owned()==0 {
            return "{\"error\":400}".to_string();
        } else {
            match param {
                Some(e) => {
                    self.list(
                        con,
                        Some(url_parser(format!("perPage={}&&{}",now,e).as_str()).as_str()),
                    )
                    .await
                }
                None => {
                    self.list(con, Some(&format!("perPage={}", now)))
                        .await
                }
            }
        }
    }
    pub async fn delete_all(&self, con: &Collection) -> String {
        let listed: Length = serde_json::from_str(&self.list_all(con, None).await).unwrap();
        println!("{:?}",listed);
        if listed.items.is_some() {
            for i in listed.items.unwrap() {
                println!("{}",&i.id);
                self.delete(con, &i.id).await;
            }
            return "success".to_string();
        }
        "error".to_string()
    }
    pub async fn update_or_create(&self, con: &Collection, id: &str, data: &str) -> String {
        let listed: Length = serde_json::from_str(&self.update(con, id, data).await).unwrap();
        if listed.total_items.is_none() {
            return String::from("{\"error\":400}");
        } else if listed.items.is_none() {
            self.create(con, data).await
        } else {
            serde_json::to_string(&listed).unwrap()
        }
    }
    pub async fn update_form(&self, con: &Collection, id: &str, path: &str) -> String {
        let url = [&self.url_struct(con), "/", id].concat();
        let file = fs::read(path).unwrap();
        let file_part = reqwest::multipart::Part::bytes(file)
            .file_name("bg.jpg")
            .mime_str("image/jpg")
            .unwrap();
        let form = reqwest::multipart::Form::new().part("img", file_part);
        let client = reqwest::Client::new();
        match client.patch(url).multipart(form).send().await {
            Ok(res) => res.text().await.unwrap_or("no message".to_string()),
        Err(_) => "{\"error\":400}".to_string(),
       }
    }
}
