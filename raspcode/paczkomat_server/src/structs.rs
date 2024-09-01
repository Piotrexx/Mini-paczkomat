use serde::Deserialize;

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