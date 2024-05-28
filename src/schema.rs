// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id, name) {
        id -> Int4,
        details -> Jsonb,
        name -> Text,
    }
}
