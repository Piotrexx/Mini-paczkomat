// rust_gpiozero = "0.2.0" tylko na raspberry pi
use reqwest::{Client, Url};
use serde_json::json;
use lib::{create_locker, get_avaible_port, ping_or_create, return_local_ipaddress};
use dotenv::dotenv;
use uuid::Uuid;
mod lib;
#[macro_use] extern crate rocket;


#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/check")]
async fn check() -> () {
    ping_or_create().await;
}
// DOKOŃCZYĆ PI)YUEIO)U*()@
#[post("/add_locker/<gpio>")]
async fn add_locker(gpio: u16) -> () {
    create_locker(gpio).await
}




#[launch]
fn rocket() -> _ {
    rocket::build()
    .configure(rocket::Config::figment()
    .merge(("address", return_local_ipaddress().unwrap()))
    .merge(("port", get_avaible_port())))
    .mount("/", routes![hello, check, add_locker])
}

