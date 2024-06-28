use std::{io::Cursor, vec};
use lib::{create_locker, create_package, get_avaible_port, ping_or_create, return_local_ipaddress, Package};
use rocket::{futures::io::Cursor, serde::json::Json, Response};
mod lib;
#[macro_use] extern crate rocket;



#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/check_or_create")]
async fn check() -> () {
    ping_or_create().await;
}

#[post("/add_locker")]
async fn add_locker() -> () {
    let pins: Vec<u16> = vec![4, 27, 22];
    for pin in pins {
            create_locker(pin).await;
    }
}

// dokończyć !!!
#[post("/add_package", format="json", data="<package>")]
fn add_package(package: Json<Package>){
    let body = "MEssage";
    Response::build()
    .sized_body(body.len(), Cursor::new(body))
    .status(create_package(package))
    .finalize()
}



#[launch]
fn rocket() -> _ {
    rocket::build()
    .configure(rocket::Config::figment()
    .merge(("address", return_local_ipaddress().unwrap()))
    .merge(("port", get_avaible_port())))
    .mount("/", routes![hello, check, add_locker, add_package])
}

