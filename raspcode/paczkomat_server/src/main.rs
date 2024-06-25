use std::vec;
use rust_gpiozero::*;
use lib::{create_locker, get_avaible_port, ping_or_create, return_local_ipaddress};
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

#[post("/add_locker")]
async fn add_locker() -> () {
    let mut pins: Vec<u16> = vec![4, 27, 22];
    for pin in pins {
        // match Led::new(pin) {
        //     Ok(_) => {
        //         create_locker(pin).await;
        //         break;
        //     },
        //     Err(_) => continue
        // }
        let led = LED::new(pin)?;
        create_locker(pin).await?;
        break;
    }
    
}




#[launch]
fn rocket() -> _ {
    rocket::build()
    .configure(rocket::Config::figment()
    .merge(("address", return_local_ipaddress().unwrap()))
    .merge(("port", get_avaible_port())))
    .mount("/", routes![hello, check, add_locker])
}

