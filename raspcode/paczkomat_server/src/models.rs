use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Serialize, Selectable, Queryable, Insertable, Deserialize)]
#[diesel(table_name=crate::schema::lockers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Locker {
    pub lockerid: String,
    pub gpio: i32,
    pub is_empty: bool
}


#[derive(Serialize, Selectable, Queryable, Insertable, Deserialize)]
#[diesel(table_name=crate::schema::package)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PackageModel {
    pub packageid: i32,
    pub locker_id: String
}
