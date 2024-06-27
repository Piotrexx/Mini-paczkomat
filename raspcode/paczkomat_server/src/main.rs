use std::vec;
use lib::{create_locker, create_package, get_avaible_port, ping_or_create, return_local_ipaddress, Package};
use rocket::serde::json::Json;
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
    create_package(package);

}



#[launch]
fn rocket() -> _ {
    rocket::build()
    .configure(rocket::Config::figment()
    .merge(("address", return_local_ipaddress().unwrap()))
    .merge(("port", get_avaible_port())))
    .mount("/", routes![hello, check, add_locker, add_package])
}

