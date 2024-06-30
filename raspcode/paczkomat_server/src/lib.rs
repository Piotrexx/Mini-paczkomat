use std::fs::File;
use std::{net::IpAddr, io::prelude::*};
use local_ip_address::local_ip;
use dotenv::dotenv;
use reqwest::{Client, Url};
use rocket::serde::json::Json;
use serde_json::{json, Value};
use std::net::TcpListener;
use std::str::FromStr;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use sqlite::Connection;
// use rust_gpiozero::*;

#[derive(Serialize)]
pub struct Locker {
    locker_id: String,
    gpio: u16
}

#[derive(Deserialize)]
pub struct Package {
    pub locker_id: String,
    pub paczkomat_id: String
}


pub fn return_local_ipaddress() ->  Result<IpAddr,String>{
    let paczkomat_ip = local_ip();
    match paczkomat_ip {
        Ok(ip) => Ok(ip),
        Err(err) => Err(format!("Wystąpił błąd: {:?}", err))
    }
}

// dokończyć !!!
pub async fn create_package(package: Json<Package>) -> Result<u16, String>{
    dotenv().ok();
    let uuid = std::env::var("uuid").expect("Nie znaleziono uuid w pliku .env");
    if !uuid.eq(&package.paczkomat_id) {
        return Err(String::from("Error: 400"));
    }

    let mut file = File::open("lockers.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data);

    let json: Value = match serde_json::from_str(&data) {
        Ok(value) => value,
        Err(err) => return Err(err.to_string()),
    };
    if let Some(_) = json.get(&uuid) {
        println!("{:?}", json.get(&uuid).unwrap());
        let url = format!("{}/locker/{}/change_emptyness/", &std::env::var("server_url").expect("Nie znaleziono url servera w pliku .env."), uuid);
        let client = Client::new();
        let response = client
        .post(Url::parse(&url).unwrap())
        .send()
        .await
        .unwrap();
        // let locker = LED::new(u8::try_from(json.get(&uuid).unwrap().as_i64().unwrap()).unwrap());
        println!("{:?}", u8::try_from(json.get(&uuid).unwrap().as_i64().unwrap()).unwrap());
        // locker.on();
        Ok(200)
    }else{
        println!("{:?}", json.get(&uuid).unwrap());
        return Err(String::from("Error: 404"));
    }
}
    
pub async fn create_locker(gpio: u16) -> Result<String> {
    dotenv().ok();
    let url = format!("{}/locker/add_locker/", &std::env::var("server_url").expect("Nie znaleziono url servera w pliku .env."));
    let client = Client::new();
    let uuid = std::env::var("uuid").expect("Nie znaleziono uuid w pliku .env");
    let locker_id = Uuid::new_v4().to_string();
    let data = json!({
        "paczkomat_id": uuid,
        "locker_id": locker_id,
        "gpio": gpio,
    });


    // let data_to_save = Locker {
    //     locker_id: locker_id.to_string(),
    //     gpio: gpio
    // };

    let connection = sqlite::open("locekrs.sqlite3")?;
    let query = format!("
        INSERT INTO lockers VALUES ('{locker_id}', {gpio})
    ");
    connection.execute(query)?;


    let response = client
        .post(Url::parse(&url)?)
        .json(&data)
        .send()
        .await
        ?;

    Ok(format!("Dane zostały zapisane poprawnie, \n Kod odpowiedzi requesta: {}", response.status()))
}

pub async fn ping_or_create() {
    dotenv().ok(); 
        
    // let mut data = HashMap::new();
    // data.insert("id", std::env::var("uuid").expect("Nie znaleziono uuid w pliku .env."));
    // data.insert("ip", return_local_ipaddress().unwrap().to_string());
    let url = format!("{}/paczkomat/add_paczkomat_or_check/", &std::env::var("server_url").expect("Nie znaleziono url servera w pliku .env."));
    let client = Client::new();
    let uuid = std::env::var("uuid").expect("Nie znaleziono uuid w pliku .env");
    let ip = return_local_ipaddress().unwrap().to_string();
    let port_string = std::env::var("PORT").unwrap();
    let port_num = u32::from_str(&port_string).unwrap();

    let data = json!({
        "id": uuid,
        "ip_address": ip,
        "port": port_num
    });

    let response = client
        .post(Url::parse(&url).unwrap())
        .json(&data)
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        println!("Request wysłany pomyślnie!");
    } else {
        println!("Wystąpił błąd: {}", response.status());
    }
    
}


// fn get_avaible_pin(pin_set: HashMap<&str, i32>) -> i32 {
//     for (key, pin) in pin_set.into_iter() {
//         if pin_avaible(pin) {
//             pin
//         }
//         continue;
//     }
// }

// fn pin_avaible(pin: i32) -> bool{
//     let pin = LED::new(pin);
//     if(pin.is_active()) {
//         true
//     }
//     false
// }

pub fn get_avaible_port() -> Option<u16> {
    (8001..9000).find(|port| port_is_available(*port))
}

fn port_is_available(port: u16) -> bool{
    let ip_address = return_local_ipaddress().unwrap();
    match TcpListener::bind((ip_address, port)){
        Ok(_) => {
            std::env::set_var("PORT", port.to_string());
            true
        },
        Err(_) => false
    }
}


pub fn setup_db()  -> Result<String>{
    File::create("lockers.sqlite3")?;
    let connection = Connection::open("lockers.sqlite3")?;
    let query = "
    CREATE TABLE lockers (
        lockerid VARCHAR(50) PRIMARY KEY,
        gpio INT
    );
    ";
    connection.execute(query)?;
    Ok(format!("Database ready !"))
}

