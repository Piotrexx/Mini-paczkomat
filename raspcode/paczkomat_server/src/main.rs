// rust_gpiozero = "0.2.0" tylko na raspberry pi
use lib::{ping, return_local_ipaddress};
mod lib;
#[macro_use] extern crate rocket;


#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/check")]
fn check() -> String {
    ping();
    format!("Wysłano requesta do sprawdzenia IP")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .configure(rocket::Config::figment().merge(("address", return_local_ipaddress().unwrap())))
    .mount("/", routes![hello, check])
}

