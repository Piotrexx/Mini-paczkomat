use rocket::http::Status;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Package {
    pub locker_id: String,
    pub paczkomat_id: String
}

#[derive(Deserialize)]
pub struct CollectPackageStruct {
    pub locker_id: String,
}

#[derive(Deserialize)]
pub struct LockerCreationStruct {
    pub paczkomat_id: String,
    pub locker_pin: Vec<i32>
}

#[derive(Serialize, Deserialize)]
pub struct ResponseStruct {
    pub massage: String,
    pub status: Status
}