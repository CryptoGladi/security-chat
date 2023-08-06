// @generated automatically by Diesel CLI.

diesel::table! {
    chat (id) {
        id -> Int8,
        #[max_length = 40]
        title -> Varchar,
    }
}

diesel::table! {
    chat_messages (id) {
        id -> Int8,
        chat_id -> Int8,
        sender_id -> Int8,
        message -> Json,
    }
}

diesel::table! {
    order_add_keys (id) {
        id -> Int8,
        user_to_id -> Int8,
        user_from_id -> Int8,
        user_to_public_key -> Bytea,
        user_from_public_key -> Nullable<Bytea>,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        #[max_length = 25]
        nickname -> Varchar,
        #[max_length = 40]
        authkey -> Varchar,
    }
}

diesel::joinable!(chat_messages -> chat (chat_id));
diesel::joinable!(chat_messages -> users (sender_id));

diesel::allow_tables_to_appear_in_same_query!(
    chat,
    chat_messages,
    order_add_keys,
    users,
);
