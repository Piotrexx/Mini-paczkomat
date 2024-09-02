use dotenv::dotenv;
use rocket::http::Status;
use crate::models::Locker;
use reqwest::{Client, Response, Url};
use rocket::serde::json::Json;
use serde_json::json;
use std::str::FromStr;
use uuid::Uuid;
use anyhow::Result;
use rust_gpiozero::*;
use diesel::prelude::*;
use tokio::time::{sleep, Duration};
use geolocation;
use crate::utils::{locker_exists, establish_connection, return_gpio_pin, return_local_ipaddress};
use crate::structs::{Package, CollectPackageStruct};


pub async fn create_package(package: Json<Package>) -> Result<(), Status>{
    dotenv().ok();
    let uuid = std::env::var("uuid").expect("Nie znaleziono uuid w pliku .env");
    if !uuid.eq(&package.paczkomat_id) {
        return Err(Status::NotFound);
    }

    let exists = locker_exists(&package.locker_id).await;
    if exists == false {
        return Err(Status::NotFound)
    }

    use crate::schema::lockers;
    let connection = &mut establish_connection();

    diesel::update(lockers::table)
    .filter(lockers::lockerid.eq(&package.locker_id))
    .set(lockers::is_empty.eq(false))
    .execute(connection).unwrap();

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
    }
    Ok(())
}

pub async fn empty_locker(data: Json<CollectPackageStruct>) -> Result<Status> {
    dotenv().ok();
    use crate::schema::lockers;
    let connection = &mut establish_connection();

    diesel::update(lockers::table)
    .filter(lockers::lockerid.eq(&data.locker_id))
    .set(lockers::is_empty.eq(true))
    .execute(connection)?;

    std::env::set_var(format!("locker_{}", data.locker_id), "true");
    tokio::task::yield_now().await;
    
    Ok(Status::Ok)
}
    
pub async fn create_locker(gpio: i32) -> Result<()> {
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

    let response = client
        .post(Url::parse(&url)?)
        .json(&data)
        .send()
        .await
        ?;

    Ok(println!("reposnse code (debug): {}", response.status()))
}

pub async fn ping_or_create() -> Result<Response>{
    dotenv().ok(); 

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

    Ok(client
        .post(Url::parse(&url).unwrap())
        .json(&data)
        .send()
        .await?)   
}

