// rust_gpiozero = "0.2.0" tylko na raspberry pi
use reqwest::{Client, Url};
use serde_json::json;
use lib::{get_avaible_port, ping_or_create, return_local_ipaddress};
use dotenv::dotenv;
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

#[post("/add_locker/<gpio>")]
async fn add_locker(gpio: u16) -> () {
    dotenv().ok();
    let url = format!("{}/locker/add_locker/", &std::env::var("server_url").expect("Nie znaleziono url servera w pliku .env."));
    let client = Client::new();
    let uuid = std::env::var("uuid").expect("Nie znaleziono uuid w pliku .env");
    let data = json!({
        "id": uuid,
        "locker_id": gpio,
    });
    let response = client
        .post(Url::parse(&url).unwrap())
        .json(&data)
        .send()
        .await
        .unwrap();
    format!("Wystąpił błąd: {}", response.status());
}




#[launch]
fn rocket() -> _ {
    rocket::build()
    .configure(rocket::Config::figment()
    .merge(("address", return_local_ipaddress().unwrap()))
    .merge(("port", get_avaible_port())))
    .mount("/", routes![hello, check, add_locker])
}

