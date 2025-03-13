// use std::{collections::HashMap, net::IpAddr};
// use reqwest::{Client, Response, Url};

// use serde_json;
// use std::any::type_name;
// use std::net::TcpListener;

// use crate::models::{Locker, PackageModel};
// use dotenv::dotenv;



// pub async fn return_gpio_pin(locker_id: &String) -> u8{
//     use crate::schema::lockers::dsl::lockers;
//     let connection = &mut establish_connection();

//     let locker = lockers
//     .find(locker_id)
//     .select(Locker::as_select())
//     .first(connection)
//     .optional();

//     match locker {
//         Ok(Some(locker)) => {
//             u8::try_from(locker.gpio).unwrap()
//         },
//         Ok(None) => panic!("Nie znaleziono takiej szafki"),
//         Err(err) => panic!("ERROR: {}", err)
//     }

// }
// pub async fn locker_exists(locker_id: &String) -> bool {
//     use crate::schema::lockers::dsl::lockers;
//     let connection = &mut establish_connection();

//     let locker = lockers
//     .find(locker_id)
//     .select(Locker::as_select())
//     .first(connection)
//     .optional();

//     match locker {
//         Ok(Some(locker)) => {
//             println!("Locker id: {}, GPIO: {}", locker.lockerid, locker.gpio);
//             true
//     },
//         Ok(None) => false,
//         Err(_) => false
//     }
// }
// 

// pub fn get_all_packages() -> Vec<PackageModel> {
//     use crate::schema::package::dsl::*;
//     let mut connection = establish_connection();
//     package.load::<PackageModel>(&mut connection).unwrap()
// }








// // }

use std::net::{TcpListener, IpAddr};
use local_ip_address::local_ip;
use reqwest::{Client, Response, Url};
use serde_json::Value;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use diesel::prelude::*;

pub async fn request_base(url_sufix: &str, data: Value) -> Result<String, reqwest::Error>{
    let url = format!("{}/{}", &std::env::var("server_url").expect("Nie znaleziono url servera w pliku .env."), url_sufix);
    let client = Client::new();
    client
    .post(Url::parse(&url).unwrap())
    .json(&data)
    .send()
    .await
    .unwrap().text().await
} 

pub fn return_local_ipaddress() ->  Result<IpAddr,String>{
    let paczkomat_ip = local_ip();
    match paczkomat_ip {
        Ok(ip) => Ok(ip),
        Err(err) => Err(format!("Wystąpił błąd: {:?}", err))
    }
}
pub fn get_avaible_port() -> Option<u16> {
    (8001..9000).find(|port| port_is_available(*port))
}

pub fn port_is_available(port: u16) -> bool{
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