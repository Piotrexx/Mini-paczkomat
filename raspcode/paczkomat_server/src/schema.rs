// @generated automatically by Diesel CLI.

diesel::table! {
    locker (id) {
        id -> Text,
        locker_gpio -> Integer,
        io_type -> Bool,
        is_empty -> Bool,
    }
}
