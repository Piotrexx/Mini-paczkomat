use std::env::{self, VarError};
use std::error::Error;
use diesel::{insert_into, RunQueryDsl};
use diesel::prelude::Insertable;
use reqwest::Response;
use rocket::http::ContentType;
use rocket::response::content;
use rocket::serde::json::Json;
use serde_json::{json, Value};
use crate::models::{Locker, NewLocker};
use crate::utils::{request_base, return_local_ipaddress, establish_connection};
use crate::serializers::PingSerializer;
use rocket::serde::uuid::Uuid;
use dotenv::dotenv;
use lazy_static::lazy_static;
use rocket::serde::{Serialize, Deserialize};
use crate::schema::locker::dsl::*;
use std::thread;
use crate::schema::locker;
#[cfg(target_os = "linux")]
use rppal::i2c::I2c;

pub fn get_paczkomat_uuid() -> Result<String, VarError>{
    dotenv().ok();
    env::var("uuid")
}

lazy_static! {
    static ref PACZKOMAT_ID: Uuid = Uuid::parse_str(&get_paczkomat_uuid().unwrap().as_str()).unwrap();
    static ref BUS_ADRESS: u8 = 0x20;
}

#[derive(Serialize, Deserialize)]
#[serde(crate="rocket::serde")]
struct LockerSerializer {
    paczkomat_id: Uuid,
    gpio: u8,
    iotype: bool,
    locker_id: Uuid
}

pub async fn ping_or_create() -> Value{
    dotenv().ok();
    let port_string = std::env::var("PORT").unwrap().parse::<u16>().unwrap();
    let data = PingSerializer{id: *PACZKOMAT_ID, ip_address: return_local_ipaddress().unwrap(), port: port_string};

    match request_base("paczkomat/add_paczkomat_or_check", json!(data)).await {
        Ok(callback_info) => json!({"info": callback_info}),
        Err(err) => json!({"error_info": err.to_string()})
        }
}
    


#[derive(Deserialize)]
#[serde(crate="rocket::serde")]
struct LockerCreation {
    paczkomat_id: Uuid,
    locker_id: Uuid,
}

#[cfg(target_os = "linux")]
pub fn turn_led_on() {
    thread::spawn(|| {
        let mut i2c = I2c::new().unwrap();
        i2c.set_slave_address(BUS_ADRESS).unwrap();   
        i2c.smbus_write_byte(0x00, 0x7A).unwrap(); // GPA
        // ustawienie które gpio są jako input lub output  dla GPB (w tym przypadku 11111111), input = 1 output = 0
        i2c.smbus_write_byte(0x01, 0xFF).unwrap(); // GPB
        // ustawia "output state" czyli czy pin jest aktywny dla GPA aktywny = 1 nieaktywny = 0 (w tym przypdaku 10000101), 
        i2c.smbus_write_byte(0x14, 0x85).unwrap(); // GPA
        // ustawia "output state" czyli czy pin jest aktywny dla GPB aktywny = 1 nieaktywny = 0 (w tym przypdaku 00000000)
        i2c.smbus_write_byte(0x15, 0x00).unwrap(); // GPB
        // sleep(Duration::from_secs(5));
        // i2c.smbus_write_byte(0x14, 0x00).unwrap();
        // i2c.smbus_write_byte(0x15, 0x00).unwrap();
        loop {
            
        }
    });
}

#[cfg(target_os = "linux")]
fn check_if_in_use() -> Result<u8, ()>{
    let mut i2c = I2c::new().unwrap();
    i2c.set_slave_address(BUS_ADRESS).unwrap();
    for n in 1..10 {
        match i2c.smbus_read_byte(u8::pow(2, n)){
            Ok(data) => {
                if data == 1 {
                    continue;
                };
                return Ok(u8::pow(2, n))
            },
            Err(err) => return Err(println!("błąd w busie")),
        }
    };
    
    // match buf[0] {
    //     1 => Ok(true),
    //     0 => Ok(false),
    //     _ => Err(println!("Błąd podczas odczytywania"))
    // }
}


fn create_locker(data: Json<LockerCreation>) -> Result<Value, Value>{
    let data = data.into_inner();
    if data.paczkomat_id != *PACZKOMAT_ID {
        return Err(json!({"err":"Podany klucz nie jest poprawny".to_string()}));
    }





    // LOGIKA ZWIĄZANA Z SZUKANIEM DOSTĘPNYCH PINÓW ORAZ BUSÓW
    let mut conn = establish_connection();
    match diesel::insert_into(locker).values(NewLocker{id: data.locker_id.to_string(), locker_gpio: 00000001, io_type: true}).execute(&mut conn){
        Ok(_) => Ok(json!({"paczkomat_id": *PACZKOMAT_ID,"locker_id": data.locker_id.to_string(),"gpio": 00000001, "iotype": true})),
        Err(err) => Err(json!({"err": err.to_string()}))
    }
    
    

}

// pub async fn create_locker(data: ) -> Value{
//     dotenv().ok();

    // let locker_id = Uuid::new_v4().to_string();
    // match request_base("paczkomat/create_locker", json!({"paczkomat_id": *PACZKOMAT_ID,"locker_id": locker_id,"gpio": 00000001, "iotype": true})).await {
    //     Ok(callback_info) => {
    //         let mut conn = establish_connection();
    //         diesel::insert_into(locker).values(NewLocker{id: locker_id.to_string(), locker_gpio: 00000001, io_type: true}).execute(&mut conn).unwrap();
    //         json!({"info": callback_info}) // "Pomyślnie utworzono skrzynke"
    //     },
    //     Err(err) => {
    //         json!({"info": err.to_string()})
    //     }
    // }

// }

// test endpoint
pub async fn return_paczkomat_id() -> Value {
    json!({"paczkomat_id": *PACZKOMAT_ID.to_string()})
}