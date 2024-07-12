// @generated automatically by Diesel CLI.

diesel::table! {
    lockers (lockerid) {
        lockerid -> Text,
        gpio -> Integer,
        is_empty -> Bool,
    }
}
