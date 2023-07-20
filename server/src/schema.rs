// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 25]
        nickname -> Varchar,
        #[max_length = 40]
        authkey -> Varchar,
    }
}
