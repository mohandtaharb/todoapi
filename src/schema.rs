// @generated automatically by Diesel CLI.

diesel::table! {
    task (id) {
        id -> Int4,
        name -> Text,
        accomplished -> Bool,
    }
}
