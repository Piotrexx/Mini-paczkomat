use std::net::IpAddr;
use rocket::serde::{Serialize, Deserialize, uuid::Uuid};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PingSerializer {
    pub id: Uuid,
    pub ip_address: IpAddr,
    pub port: u16,
}