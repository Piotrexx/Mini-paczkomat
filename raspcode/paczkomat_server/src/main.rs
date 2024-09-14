pub mod schema;
pub mod models;
use std::vec;
use diesel::RunQueryDsl;
use views::{create_locker, create_package,empty_locker, ping_or_create};
use utils::{establish_connection, get_avaible_port, return_local_ipaddress, get_all_packages};
use structs::{Package, CollectPackageStruct, LockerCreationStruct, ResponseStruct};
use models::{Locker, PackageModel};
use rocket::serde::json::Json;
use rocket_cors::{CorsOptions, AllowedOrigins, AllowedHeaders};
use rocket::http::Method;
use dotenv::dotenv;
// use async_cron_scheduler::{Job, Scheduler};
// use chrono::offset::Local;
// use std::time::Duration;
use rocket::http::Status;
mod views;
mod utils;
mod structs;

#[macro_use] extern crate rocket;

#[get("/check_or_create")]
async fn check() -> Json<ResponseStruct> {
    match ping_or_create().await {
        Ok(response) => Json(ResponseStruct { massage: format!("Zpingowano główny serwer (kod statusu od głównego serwera: {}", response.status()), status: Status::Ok  }),
        Err(err_response) => Json(ResponseStruct { massage: format!("Wystąpił błąd podacz wysyłania zapytania do głównego serwera: {}", err_response), status: Status::InternalServerError })
    }
}

#[post("/add_locker", format="json", data="<locker_creation_data>")]
async fn add_locker(locker_creation_data: Json<LockerCreationStruct>) -> Status {
    dotenv().ok();

    if locker_creation_data.paczkomat_id != std::env::var("uuid").expect("Nie znaleziono url servera w pliku .env.") {
        return Status::Forbidden;
    }
    
    let pins = locker_creation_data.locker_pin.clone();
    for pin in pins {
            create_locker(pin).await;
    }

    Status::Created
}

#[get("/all_lockers")]
fn all_lockers() -> Json<Vec<Locker>> {
    use crate::schema::lockers::dsl::*;
    let mut connection = establish_connection();
    let lockers_query = lockers.load::<Locker>(&mut connection).unwrap();

    Json(lockers_query)
}  

#[post("/add_package", format="json", data="<package>")]
async fn add_package(package: Json<Package>) -> Json<ResponseStruct>{
    match create_package(package).await {
        Ok(_) => Json(ResponseStruct { massage: String::from("Dodano paczkę do szafki"), status: Status::Created}),
        Err(err) => {
            println!("Error code (Debug): {}", err);
            Json(ResponseStruct { massage: String::from("Wystąpił błąd (error wyświetlony w konsoli)"), status: err })
        }
    }
}

#[patch("/collect_package", format="json", data="<data>")]
async fn collect_package(data: Json<CollectPackageStruct>) -> Json<ResponseStruct>{
    match empty_locker(data).await {
        Ok(status_code) => Json(ResponseStruct { massage: String::from("Opróżniono szafkę"), status: status_code }),
        Err(err) => Json(ResponseStruct { massage: format!("Error: {}", err), status: Status::InternalServerError })
    }
    
}


#[get("/all_packages")]
fn all_packages() -> Json<Vec<PackageModel>> {
    Json(get_all_packages())
}


#[launch]
#[tokio::main]
async fn rocket() -> _ {
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

    // tokio::spawn(async move {
    //     let (mut scheduler, sched_service) = Scheduler::<Local>::launch(tokio::time::sleep);
    //     let job = Job::cron("*/15 * * * * *").unwrap();
    //     let fizz_id = scheduler.insert(job, |_id| ping_or_create()).await; // naprawić / dodać cronjoby

        

    // });

    rocket::build()
    .attach(cors)
    .configure(rocket::Config::figment()
    .merge(("address", return_local_ipaddress().unwrap()))
    .merge(("port", get_avaible_port())))
    .mount("/package", routes![add_package,collect_package,all_packages])
    .mount("/locker", routes![add_locker, all_lockers])
    .mount("/", routes![check])

}

