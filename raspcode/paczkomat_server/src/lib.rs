use std::{collections::HashMap, net::IpAddr};
use local_ip_address::local_ip;
use dotenv::dotenv;
use reqwest::{Client, Url};
use serde_json::json;
use std::net::TcpListener;
use std::env;
use std::str::FromStr;

pub fn return_local_ipaddress() ->  Result<IpAddr,String>{
    let paczkomat_ip = local_ip();
    match paczkomat_ip {
        Ok(ip) => Ok(ip),
        Err(err) => Err(format!("Wystąpił błąd: {:?}", err))
    }
}

// pub async fn create_locker() {
//     dotenv().ok(); 
//     let mut gpio_pinouts = HashMap::new();
//     gpio_pinouts.insert("1", 4);
//     gpio_pinouts.insert("2", 27);
//     gpio_pinouts.insert("3", 22);
//     let uuid = std::env::var("uuid").expect("Nie znaleziono uuid w pliku .env");
//     let url = format!("{}/locker/add_locker/", &std::env::var("server_url").expect("Nie znaleziono url servera w pliku .env."));
    

    
    
//     let data = json!({
//         "id": uuid,
//         // "locker_id": 
//     });
// }

pub async fn ping() {
    dotenv().ok(); 
    
    // let mut data = HashMap::new();
    // data.insert("id", std::env::var("uuid").expect("Nie znaleziono uuid w pliku .env."));
    // data.insert("ip", return_local_ipaddress().unwrap().to_string());
    let url = format!("{}/ip/check/", &std::env::var("server_url").expect("Nie znaleziono url servera w pliku .env."));
    let client = Client::new();
    let uuid = std::env::var("uuid").expect("Nie znaleziono uuid w pliku .env");
    let ip = return_local_ipaddress().unwrap().to_string();
    let port_string = env::var("PORT").unwrap();
    let port_num = u32::from_str(&port_string).unwrap();

    let data = json!({
        "id": uuid,
        "ip_address": ip,
        "port": port_num
    });

    let response = client
        .patch(Url::parse(&url).unwrap())
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
            env::set_var("PORT", port.to_string());
            true
        },
        Err(_) => false
    }
}

