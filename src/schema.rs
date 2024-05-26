// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id, name) {
        id -> Int4,
        details -> Text,
        name -> Text,
    }
}
