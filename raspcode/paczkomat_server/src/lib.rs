use std::fs::File;
use std::net::IpAddr;
use local_ip_address::local_ip;
use dotenv::dotenv;
use reqwest::{Client, Url};
use rocket::serde::json::Json;
use serde_json::json;
use std::net::TcpListener;
use std::str::FromStr;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use rust_gpiozero::*;


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
pub async fn create_package(package: Json<Package>) -> Result<String>{
    dotenv().ok();
    let uuid = std::env::var("uuid").expect("Nie znaleziono uuid w pliku .env");
    if !uuid.eq(&package.paczkomat_id) {
        return Ok(String::from("Error: 400"));
    }
    // dokończyć  załącznie diody !!!
    let exists = locker_exists(&package.locker_id).await;
    if exists == false {
        return Ok(String::from("Error, przesłane ID skrzynki nie istnieje"))
    }
    println!("before sending");
    let url = format!("{}/locker/{}/change_emptyness/", &std::env::var("server_url").expect("Nie znaleziono url servera w pliku .env."), &package.locker_id);
    let client = Client::new();
    let response = client
    .post(Url::parse(&url)?)
    .send()
    .await
    .unwrap();
    println!("test lol xd");
    if cfg!(unix) {
        let locker_pin = return_gpio_pin(&package.locker_id).await?;
        println!("raz dwa trzy");
        tokio::spawn(async move {
            println!("{}", locker_pin);
            let mut locker = LED::new(locker_pin);
            println!("test1234242");
            locker.on();
            loop {}
          });
        Ok(String::from("LED załączony"));
    }
    println!("o co chodzi");
    Ok(String::from("Wszystko poszło (w trybie windows)"))
}


async fn return_gpio_pin(locker_id: &String) -> Result<u8> {
    let query = format!("SELECT * FROM lockers WHERE lockerid LIKE '{locker_id}' LIMIT 1;");
    println!("{}", query);
    let connection = sqlx::sqlite::SqlitePool::connect("lockers.sqlite3").await?;
    let res = sqlx::query(&query).fetch_one(&connection).await?;
    return Ok(res.get::<u8>("gpio")?);
}

async fn locker_exists(locker_id: &String) -> bool {
    
    let query = "SELECT * FROM lockers";
    let connection = sqlx::sqlite::SqlitePool::connect("lockers.sqlite3").await.unwrap();
    let res = sqlx::query(&query).fetch_all(&connection).await.unwrap();
    let mut exists = false;
    
    for (index, row) in res.iter().enumerate() {
        if locker_id.clone() != row.get::<String>("lockerid").unwrap() {
            continue;
        }else{
            exists = true;
            break;
        }
    }
    exists
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

    // let connection = sqlx::sqlite::SqlitePool::connect("lockers.sqlite3").await?;
    let query = format!("INSERT INTO lockers VALUES ('{locker_id}', {gpio})");
    let connection = sqlx::sqlite::SqlitePool::connect("lockers.sqlite3").await?;
    let result = sqlx::query(query.as_str()).execute(&connection).await?;
    println!("Created locker data");

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


pub async fn setup_db()  -> Result<String>{
    File::create("lockers.sqlite3")?;
    let connection = sqlx::sqlite::SqlitePool::connect("lockers.sqlite3").await?;
    sqlx::migrate!("./migrations").run(&connection).await?;
    Ok(format!("Database ready !"))
}

