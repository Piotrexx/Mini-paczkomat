use std::vec;
use lib::{create_locker, create_package, get_avaible_port, ping_or_create, return_local_ipaddress, setup_db, Package};
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

#[get("/db_setup_test")]
fn db_setup_test() -> String {
    setup_db().unwrap()
}

// dokończyć !!!
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



#[launch]
fn rocket() -> _ {
    rocket::build()
    .configure(rocket::Config::figment()
    .merge(("address", return_local_ipaddress().unwrap()))
    .merge(("port", get_avaible_port())))
    .mount("/", routes![hello, check, add_locker, add_package, db_setup_test])
}

