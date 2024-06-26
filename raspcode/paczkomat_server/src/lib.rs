use std::{net::IpAddr, fs::{File, OpenOptions}, io::prelude::*};
use local_ip_address::local_ip;
use dotenv::dotenv;
use reqwest::{Client, Url};
use serde_json::json;
use std::net::TcpListener;
use std::str::FromStr;
use uuid::Uuid;
use serde::{Serialize, Deserialize};


#[derive(Serialize)]
struct Locker {
    locker_id: String,
    gpio: u16
}

#[derive(Deserialize)]
pub struct Package {
    locker_id: String
}



pub fn return_local_ipaddress() ->  Result<IpAddr,String>{
    let paczkomat_ip = local_ip();
    match paczkomat_ip {
        Ok(ip) => Ok(ip),
        Err(err) => Err(format!("Wystąpił błąd: {:?}", err))
    }
}

pub async fn create_locker(gpio: u16) {
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


    let data_to_save = Locker {
        locker_id: locker_id.to_string(),
        gpio: gpio
    };
    let json_data = serde_json::to_string(&data_to_save).unwrap();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("lockers.json")
        .unwrap();
    
        let mut metadata = file.metadata();
        let is_empty = metadata.len() == 0;
    
        if !is_empty {
            file.write_all(b",\n");
        }
    
        file.write_all(json_data.as_bytes());

    let response = client
        .post(Url::parse(&url).unwrap())
        .json(&data)
        .send()
        .await
        .unwrap();
    format!("Wystąpił błąd: {}", response.status());
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

