pub mod schema;
pub mod models;
use std::vec;
use diesel::RunQueryDsl;
use functions::{create_locker, create_package,empty_locker, ping_or_create};
use utils::{establish_connection, get_avaible_port, return_local_ipaddress};
use structs::{Package, CollectPackageStruct};
use models::Locker;
use rocket::serde::json::Json;
use rocket_cors::{CorsOptions, AllowedOrigins, AllowedHeaders};
use rocket::http::Method;
use dotenv::dotenv;
mod functions;
mod utils;
mod structs;
#[macro_use] extern crate rocket;
use tokio;

#[get("/check_or_create")]
async fn check() -> () {
    ping_or_create().await;
}

#[post("/add_locker")]
async fn add_locker() -> () {
    let pins: Vec<i32> = vec![23, 27, 22];
    for pin in pins {
            create_locker(pin).await;
    }
}

#[get("/all_lockers")]
fn all_lockers() {
    use crate::schema::lockers::dsl::*;
    let mut connection = establish_connection();
    let lockers_query = lockers.load::<Locker>(&mut connection);

    for locker in lockers_query.unwrap() {
        println!("locker id: {}, is empty: {}", locker.lockerid, locker.is_empty)
    }
}  

#[post("/add_package", format="json", data="<package>")]
async fn add_package(package: Json<Package>) -> String{
    // format!("Code Returned: {}", create_package(package).await.unwrap())
    match create_package(package).await {
        Ok(code) => format!("Code: {}", code),
        Err(err) => {
            println!("Error: {}", err);
            format!("Error: {}", err)
        }
    }
}

#[patch("/collect_package", format="json", data="<data>")]
async fn collect_package(data: Json<CollectPackageStruct>) -> String{
    empty_locker(data).await.unwrap()
}


#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let url = std::env::var("server_url").expect("Nie znaleziono url servera w pliku .env.");
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::some_exact(&[url]),
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Put,
            Method::Delete,
            Method::Options,
        ].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization","Content-Type",]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS");


    rocket::build()
    .attach(cors)
    .configure(rocket::Config::figment()
    .merge(("address", return_local_ipaddress().unwrap()))
    .merge(("port", get_avaible_port())))
    .mount("/", routes![check, add_locker, add_package, all_lockers, collect_package])
}

