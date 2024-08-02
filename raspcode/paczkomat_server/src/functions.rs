use std::net::IpAddr;
use local_ip_address::local_ip;
use dotenv::dotenv;
use crate::models::Locker;
use reqwest::{Client, Url};
use rocket::serde::json::Json;
use serde_json::json;
use std::net::TcpListener;
use std::str::FromStr;
use uuid::Uuid;
use serde::Deserialize;
use anyhow::Result;
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use rust_gpiozero::*;
use tokio::time::{sleep, Duration};
use geolocation;


#[derive(Deserialize)]
pub struct Package {
    pub locker_id: String,
    pub paczkomat_id: String
}

#[derive(Deserialize)]
pub struct CollectPackageStruct {
    pub locker_id: String,
}


pub fn return_local_ipaddress() ->  Result<IpAddr,String>{
    let paczkomat_ip = local_ip();
    match paczkomat_ip {
        Ok(ip) => Ok(ip),
        Err(err) => Err(format!("Wystąpił błąd: {:?}", err))
    }
}



pub async fn create_package(package: Json<Package>) -> Result<String>{
    dotenv().ok();
    let uuid = std::env::var("uuid").expect("Nie znaleziono uuid w pliku .env");
    if !uuid.eq(&package.paczkomat_id) {
        return Ok(String::from("Error: 400"));
    }

    let exists = locker_exists(&package.locker_id).await;
    if exists == false {
        return Ok(String::from("Error, przesłane ID skrzynki nie istnieje"))
    }

    use crate::schema::lockers;
    let connection = &mut establish_connection();

    diesel::update(lockers::table)
    .filter(lockers::lockerid.eq(&package.locker_id))
    .set(lockers::is_empty.eq(false))
    .execute(connection)?;

    if cfg!(unix) {
        let locker_pin = return_gpio_pin(&package.locker_id).await;
        let locker_id = package.locker_id.clone();

        tokio::spawn(async move {
            
            let mut locker = LED::new(locker_pin);
            std::env::set_var(format!("locker_{}", locker_id), "false");
            locker.on();
            loop {
                let is_empty = std::env::var(format!("locker_{}", locker_id)).expect("Nie znaleziono lockera");
                if is_empty == "true" {
                    locker.off();
                    std::env::remove_var(format!("locker_{}", locker_id));
                    break;
                }
                tokio::task::yield_now().await;
                sleep(Duration::from_millis(500));
            };
          });
        return Ok(String::from("LED załączony"));
    }
    return Ok(String::from("Wszystko poszło (w trybie windows)"))
}

pub async fn empty_locker(data: Json<CollectPackageStruct>) -> Result<String> {
    dotenv().ok();
    use crate::schema::lockers;
    let connection = &mut establish_connection();

    diesel::update(lockers::table)
    .filter(lockers::lockerid.eq(&data.locker_id))
    .set(lockers::is_empty.eq(true))
    .execute(connection)?;

    std::env::set_var(format!("locker_{}", data.locker_id), "true");
    tokio::task::yield_now().await;
    Ok(String::from("DEV"))
}


async fn return_gpio_pin(locker_id: &String) -> u8{
    use crate::schema::lockers::dsl::lockers;
    let connection = &mut establish_connection();

    let locker = lockers
    .find(locker_id)
    .select(Locker::as_select())
    .first(connection)
    .optional();

    match locker {
        Ok(Some(locker)) => {
            u8::try_from(locker.gpio).unwrap()
        },
        Ok(None) => panic!("Nie znaleziono takiej szafki"),
        Err(err) => panic!("ERROR: {}", err)
    }

}


async fn locker_exists(locker_id: &String) -> bool {
    use crate::schema::lockers::dsl::lockers;
    let connection = &mut establish_connection();

    let locker = lockers
    .find(locker_id)
    .select(Locker::as_select())
    .first(connection)
    .optional();

    match locker {
        Ok(Some(locker)) => {
            println!("Locker id: {}, GPIO: {}", locker.lockerid, locker.gpio);
            true
    },
        Ok(None) => false,
        Err(_) => false
    }
}
    
pub async fn create_locker(gpio: i32) -> Result<String> {
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

    use crate::schema::lockers;

    let connection = &mut establish_connection();

    let new_locker = Locker {
        lockerid: locker_id, 
        gpio: gpio,
        is_empty: true
    };

    diesel::insert_into(lockers::table).values(&new_locker).execute(connection)?;
    // .returning(Locker::as_returning())
    // .get_result(connection)
    // .expect("Zapis nie udany");

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


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = String::from("lockers.sqlite");

    SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Nie można było połączyć się z {}", database_url))
}
