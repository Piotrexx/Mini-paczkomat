use serde::Serialize;
use diesel::prelude::*;

#[derive(Serialize, Selectable, Queryable, Insertable)]
#[diesel(table_name=crate::schema::lockers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Locker {
    pub lockerid: String,
    pub gpio: i32,
    pub is_empty: bool
}
