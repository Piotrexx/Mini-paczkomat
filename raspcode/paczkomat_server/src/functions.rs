use std::net::IpAddr;
use local_ip_address::local_ip;
use dotenv::dotenv;
use crate::models::Locker;
use reqwest::{Client, Url};
use rocket::serde::json::Json;
use serde_json::json;
use std::net::TcpListener;
use std::str::FromStr;
use uuid::Uuid;
use serde::Deserialize;
use anyhow::Result;
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use rust_gpiozero::*;
use tokio::sync::{oneshot, mpsc};
// use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Package {
    pub locker_id: String,
    pub paczkomat_id: String
}


struct Actor {
    receiver: mpsc::Receiver<ActorMessage>,
    // empty: bool,
    // locker_id: String
    locker_data: HashMap<String, bool>
}

enum ActorMessage {
    CheckIfEmpty {
        respond_to: oneshot::Sender<HashMap<String, bool>>
    }
}

impl Actor {
    fn new(receiver: mpsc::Receiver<ActorMessage>, locker_id: String) -> Self {
        let mut m: HashMap<String, bool> = HashMap::new();
        m.insert(locker_id, false);
        Actor{
            receiver,
            // empty: false,
            // locker_id
            locker_data: m
        }
    }

    fn handle_message(&mut self, msg: ActorMessage, locker_id: String) {
        match msg {
            ActorMessage::CheckIfEmpty { respond_to } => {
                let cloned_id = locker_id.clone();
                *self.locker_data.entry(locker_id).or_insert(false) = true;
                let empty = *self.locker_data.get(&cloned_id).unwrap();
                let mut m = HashMap::new();
                m.insert(cloned_id, empty);
                let _ = respond_to.send(m);
            }
        }
    }
}


// lazy_static! {
//     static ref ARRAY_OF_EMPTYNESS: HashMap<String, bool> = {
//         let mut m = HashMap::new();
//         m
//     };
// }

async fn run_actor(mut actor: Actor) {
    
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg, actor.locker_data.keys().into_iter().next().unwrap().clone())
    }
}


#[derive(Clone)]
struct ActorHandle{
    sender: mpsc::Sender<ActorMessage>
}

impl ActorHandle {
    fn new(locker_id: String) -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = Actor::new(receiver, locker_id);
        tokio::spawn(run_actor(actor));

        Self{sender}
    }

    async fn check_if_empty(&self) -> HashMap<String, bool> {
        let (send, recv) = oneshot::channel();
        let msg = ActorMessage::CheckIfEmpty { respond_to: send };
        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor killed")
    }

}


pub fn return_local_ipaddress() ->  Result<IpAddr,String>{
    let paczkomat_ip = local_ip();
    match paczkomat_ip {
        Ok(ip) => Ok(ip),
        Err(err) => Err(format!("Wystąpił błąd: {:?}", err))
    }
}



pub async fn create_package(package: Json<Package>) -> Result<String>{
    dotenv().ok();
    let uuid = std::env::var("uuid").expect("Nie znaleziono uuid w pliku .env");
    if !uuid.eq(&package.paczkomat_id) {
        return Ok(String::from("Error: 400"));
    }

    let exists = locker_exists(&package.locker_id).await;
    if exists == false {
        return Ok(String::from("Error, przesłane ID skrzynki nie istnieje"))
    }
    let url = format!("{}/locker/{}/change_emptyness/", &std::env::var("server_url").expect("Nie znaleziono url servera w pliku .env."), &package.locker_id);
    let client = Client::new();
    let response = client
    .post(Url::parse(&url)?)
    .send()
    .await
    .unwrap();
    if cfg!(unix) {
        let locker_pin = return_gpio_pin(&package.locker_id).await;
        tokio::spawn(async move {
            let mut locker = LED::new(locker_pin);
            locker.on();
            let id = package.locker_id.clone();
            loop {
                if *ActorHandle::new(id).check_if_empty().await.get(&id).unwrap(){
                    locker.off()
                }
            }
          });
        return Ok(String::from("LED załączony"));
    }
    return Ok(String::from("Wszystko poszło (w trybie windows)"))
}



pub fn empty_locker(data: Json<Package>) -> Result<String> {
    dotenv().ok();
    use crate::schema::lockers;
    let connection = &mut establish_connection();

    diesel::update(lockers::table)
    .filter(lockers::lockerid.eq(&data.locker_id))
    .set(lockers::is_empty.eq(true))
    .execute(connection)?;

    ActorHandle::new(data.locker_id.clone());

    Ok(String::from("DEV"))

}


async fn return_gpio_pin(locker_id: &String) -> u8{
    use crate::schema::lockers::dsl::lockers;
    let connection = &mut establish_connection();

    let locker = lockers
    .find(locker_id)
    .select(Locker::as_select())
    .first(connection)
    .optional();

    match locker {
        Ok(Some(locker)) => {
            u8::try_from(locker.gpio).unwrap()
        },
        Ok(None) => panic!("Nie znaleziono takiej szafki"),
        Err(err) => panic!("ERROR: {}", err)
    }

}


async fn locker_exists(locker_id: &String) -> bool {
    use crate::schema::lockers::dsl::lockers;
    let connection = &mut establish_connection();

    let locker = lockers
    .find(locker_id)
    .select(Locker::as_select())
    .first(connection)
    .optional();

    match locker {
        Ok(Some(locker)) => {
            println!("Locker id: {}, GPIO: {}", locker.lockerid, locker.gpio);
            true
    },
        Ok(None) => false,
        Err(_) => false
    }
}
    
pub async fn create_locker(gpio: i32) -> Result<String> {
    dotenv().ok();
    let url = format!("{}/locker/add_locker/", &std::env::var("server_url").expect("Nie znaleziono url servera w pliku .env."));
    let client = Client::new();
    let uuid = std::env::var("uuid").expect("Nie znaleziono uuid w pliku .env");
    let locker_id = Uuid::new_v4().to_string();
    let data = json!({
        "paczkomat_id": uuid,
        "locker_id": locker_id,
        "gpio": gpio,
    });

    use crate::schema::lockers;

    let connection = &mut establish_connection();

    let new_locker = Locker {
        lockerid: locker_id, 
        gpio: gpio,
        is_empty: true
    };

    diesel::insert_into(lockers::table).values(&new_locker).execute(connection)?;
    // .returning(Locker::as_returning())
    // .get_result(connection)
    // .expect("Zapis nie udany");

    let response = client
        .post(Url::parse(&url)?)
        .json(&data)
        .send()
        .await
        ?;

    Ok(format!("Dane zostały zapisane poprawnie, \n Kod odpowiedzi requesta: {}", response.status()))
}

pub async fn ping_or_create() {
    dotenv().ok(); 
        
    // let mut data = HashMap::new();
    // data.insert("id", std::env::var("uuid").expect("Nie znaleziono uuid w pliku .env."));
    // data.insert("ip", return_local_ipaddress().unwrap().to_string());
    let url = format!("{}/paczkomat/add_paczkomat_or_check/", &std::env::var("server_url").expect("Nie znaleziono url servera w pliku .env."));
    let client = Client::new();
    let uuid = std::env::var("uuid").expect("Nie znaleziono uuid w pliku .env");
    let ip = return_local_ipaddress().unwrap().to_string();
    let port_string = std::env::var("PORT").unwrap();
    let port_num = u32::from_str(&port_string).unwrap();

    let data = json!({
        "id": uuid,
        "ip_address": ip,
        "port": port_num
    });

    let response = client
        .post(Url::parse(&url).unwrap())
        .json(&data)
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        println!("Request wysłany pomyślnie!");
    } else {
        println!("Wystąpił błąd: {}", response.status());
    }
    
}


// fn get_avaible_pin(pin_set: HashMap<&str, i32>) -> i32 {
//     for (key, pin) in pin_set.into_iter() {
//         if pin_avaible(pin) {
//             pin
//         }
//         continue;
//     }
// }

// fn pin_avaible(pin: i32) -> bool{
//     let pin = LED::new(pin);
//     if(pin.is_active()) {
//         true
//     }
//     false
// }

pub fn get_avaible_port() -> Option<u16> {
    (8001..9000).find(|port| port_is_available(*port))
}

fn port_is_available(port: u16) -> bool{
    let ip_address = return_local_ipaddress().unwrap();
    match TcpListener::bind((ip_address, port)){
        Ok(_) => {
            std::env::set_var("PORT", port.to_string());
            true
        },
        Err(_) => false
    }
}


// pub async fn setup_db()  -> Result<String>{
//     File::create("lockers.sqlite3")?;
//     let connection = sqlx::sqlite::SqlitePool::connect("lockers.sqlite3").await?;
//     sqlx::migrate!("./migrations").run(&connection).await?;
//     Ok(format!("Database ready !"))
// }


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = String::from("lockers.sqlite");

    SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Nie można było połączyć się z {}", database_url))
}