// @generated automatically by Diesel CLI.

diesel::table! {
    lockers (lockerid) {
        lockerid -> Text,
        gpio -> Integer,
        is_empty -> Bool,
    }
}

diesel::table! {
    package (packageid) {
        packageid -> Integer,
        locker_id -> Text,
    }
}

diesel::joinable!(package -> lockers (locker_id));

diesel::allow_tables_to_appear_in_same_query!(
    lockers,
    package,
);
