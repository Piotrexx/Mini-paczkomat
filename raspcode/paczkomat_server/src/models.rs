use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[serde(crate="rocket::serde")]
#[diesel(table_name = crate::schema::locker)]
pub struct Locker {
    pub id: String,
    pub locker_gpio: i32,
    pub is_empty: bool,
    pub io_type: bool,
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::locker)]
pub struct NewLocker {
    pub id: String,
    pub locker_gpio: i32,
    pub io_type: bool
}