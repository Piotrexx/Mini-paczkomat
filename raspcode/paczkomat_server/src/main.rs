// pub mod schema;
// pub mod models;
// use std::vec;
// use diesel::RunQueryDsl;
// use reqwest::{Response, Error};
// use views::{create_locker, create_package,empty_locker, ping_or_create};
// use utils::{establish_connection, get_avaible_port, return_local_ipaddress, get_all_packages};
// use structs::{Package, CollectPackageStruct, LockerCreationStruct, ResponseStruct};
// use models::{Locker, PackageModel};
// use rocket::{response::Responder, serde::json::Json};
// use rocket_cors::{CorsOptions, AllowedOrigins, AllowedHeaders};
// use rocket::http::Method;
// use dotenv::dotenv;
// // use async_cron_scheduler::{Job, Scheduler};
// // use chrono::offset::Local;
// // use std::time::Duration;
// use rocket::http::Status;
// mod views;
// mod structs;

// #[macro_use] extern crate rocket;

// #[get("/check_or_create")]
// async fn check() -> Json<Responder<Response, Error>> {
//     Json(ping_or_create().await.unwrap())
// }

// #[post("/add_locker", format="json", data="<locker_creation_data>")]
// async fn add_locker(locker_creation_data: Json<LockerCreationStruct>) -> Status {
//     dotenv().ok();

//     if locker_creation_data.paczkomat_id != std::env::var("uuid").expect("Nie znaleziono url servera w pliku .env.") {
//         return Status::Forbidden;
//     }
    
//     let pins = locker_creation_data.locker_pin.clone();
//     for pin in pins {
//             create_locker(pin).await;
//     }

//     Status::Created
// }

// #[get("/all_lockers")]
// fn all_lockers() -> Json<Vec<Locker>> {
//     use crate::schema::lockers::dsl::*;
//     let mut connection = establish_connection();
//     let lockers_query = lockers.load::<Locker>(&mut connection).unwrap();

//     Json(lockers_query)
// }  

// #[post("/add_package", format="json", data="<package>")]
// async fn add_package(package: Json<Package>) -> Json<ResponseStruct>{
//     match create_package(package).await {
//         Ok(_) => Json(ResponseStruct { massage: String::from("Dodano paczkę do szafki"), status: Status::Created}),
//         Err(err) => {
//             println!("Error code (Debug): {}", err);
//             Json(ResponseStruct { massage: String::from("Wystąpił błąd (error wyświetlony w konsoli)"), status: err })
//         }
//     }
// }

// #[patch("/collect_package", format="json", data="<data>")]
// async fn collect_package(data: Json<CollectPackageStruct>) -> Json<ResponseStruct>{
//     match empty_locker(data).await {
//         Ok(status_code) => Json(ResponseStruct { massage: String::from("Opróżniono szafkę"), status: status_code }),
//         Err(err) => Json(ResponseStruct { massage: format!("Error: {}", err), status: Status::InternalServerError })
//     }
    
// }


// #[get("/all_packages")]
// fn all_packages() -> Json<Vec<PackageModel>> {
//     Json(get_all_packages())
// }


// #[launch]
// #[tokio::main]
// async fn rocket() -> _ {
//     dotenv().ok();
//     let url = std::env::var("server_url").expect("Nie znaleziono url servera w pliku .env.");
//     let cors = CorsOptions {
//         allowed_origins: AllowedOrigins::some_exact(&[url]),
//         allowed_methods: vec![
//             Method::Get,
//             Method::Post,
//             Method::Put,
//             Method::Delete,
//             Method::Options,
//         ].into_iter().map(From::from).collect(),
//         allowed_headers: AllowedHeaders::some(&["Authorization","Content-Type",]),
//         allow_credentials: true,
//         ..Default::default()
//     }
//     .to_cors()
//     .expect("error while building CORS");

//     // tokio::spawn(async move {
//     //     let (mut scheduler, sched_service) = Scheduler::<Local>::launch(tokio::time::sleep);
//     //     let job = Job::cron("*/15 * * * * *").unwrap();
//     //     let fizz_id = scheduler.insert(job, |_id| ping_or_create()).await; // naprawić / dodać cronjoby

        

//     // });

//     rocket::build()
//     .attach(cors)
//     .configure(rocket::Config::figment()
//     .merge(("address", return_local_ipaddress().unwrap()))
//     .merge(("port", get_avaible_port())))
//     .mount("/package", routes![add_package,collect_package,all_packages])
//     .mount("/locker", routes![add_locker, all_lockers])
//     .mount("/", routes![check])

// }
#[macro_use] extern crate rocket;
mod views;
mod utils;
mod serializers;
mod schema;
mod models;
use reqwest::Response;
use rocket::{http::ContentType, serde::json::{json, Json, Value}, Config};
use rocket::serde::uuid::Uuid;
use dotenv::dotenv;
use std::env;
use utils::{return_local_ipaddress, get_avaible_port};
use views::{ping_or_create, return_paczkomat_id, get_paczkomat_uuid};
#[cfg(target_os="linux")]
use views::{turn_led_on, check_if_in_use};




#[get("/ping")]
async fn ping() -> Value {
    ping_or_create().await
}

#[get("/info")]
async fn paczkomat_info() -> Value {
    return_paczkomat_id().await
}


fn initializer() -> String{
    dotenv().ok();
    match get_paczkomat_uuid() {
        Ok(_) => "initialialization already happened".to_string(),
        Err(_) => {
            env::set_var("uuid", Uuid::new_v4().to_string());
            "initialization finished".to_string()
        }
    }
}


// #[post("/add_locker", format="json", data="<locker_creation_data>")]
// async fn add_locker(locker_creation_data: Json<LockerCreationStruct>) -> Status {
//     dotenv().ok();

//     if locker_creation_data.paczkomat_id != std::env::var("uuid").expect("Nie znaleziono url servera w pliku .env.") {
//         return Status::Forbidden;
//     }
    
//     let pins = locker_creation_data.locker_pin.clone();
//     for pin in pins {
//             create_locker(pin).await;
//     }

//     Status::Created
// }


#[launch]
fn rocket() -> _ {
    let config = Config {
        port: get_avaible_port().unwrap(),
        address: return_local_ipaddress().unwrap(),
        ..Default::default()
    };
    
    #[cfg(target_os="linux")]
    turn_led_on();
    #[cfg(target_os="linux")]
    println!("{:?}", check_if_in_use());

    rocket::build()
    .mount("/machine", routes![ping, paczkomat_info])
    .configure(&config)




}

