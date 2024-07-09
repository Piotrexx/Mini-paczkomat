pub mod schema;
pub mod models;
use std::vec;
use functions::{create_locker, create_package, establish_connection, get_avaible_port, ping_or_create, return_local_ipaddress, Package};
use models::Locker;
use rocket::serde::json::Json;
mod functions;
#[macro_use] extern crate rocket;
use tokio;



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
    let pins: Vec<i32> = vec![23, 27, 22];
    for pin in pins {
            create_locker(pin).await;
    }
}

#[get("/all_lockers")]
fn all_lockers() {
    // use self::schema::lockers::dsl::*;
    // let connection = &mut establish_connection();
    // let results: Vec<Locker> = Locker::load(&connection);
    // let results = lockers.select(Locker::as_select()).load(connection);
    // let results = lockers::table.load::<Locker>(&connection);
    // let results = lockers.select(Locker::as_select()).;
    // for locker in results {
    //     println!("{:?}", locker)
    // }
}   

// #[get("/db_setup_test")]
// fn db_setup_test() -> String {
//     setup_db().unwrap()
// }

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


#[get("/led_on")]
fn led_test() {
    use rust_gpiozero::*;
    let mut locker = LED::new(4);
    tokio::spawn(async move {
        locker.on();
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    });
}



#[launch]
fn rocket() -> _ {
    rocket::build()
    .configure(rocket::Config::figment()
    .merge(("address", return_local_ipaddress().unwrap()))
    .merge(("port", get_avaible_port())))
    .mount("/", routes![hello, check, add_locker, add_package, all_lockers, led_test])
}

