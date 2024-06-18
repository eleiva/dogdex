// @generated automatically by Diesel CLI.

diesel::table! {
    dogs (id) {
        id -> Int4,
        name -> Varchar,
        image_path -> Varchar,
    }
}
