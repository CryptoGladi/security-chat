// @generated automatically by Diesel CLI.

diesel::table! {
    chat_messages (id) {
        id -> Int8,
        sender_id -> Int8,
        recipient_id -> Int8,
        message_body -> Bytea,
        nonce -> Bytea,
        created_at -> Timestamptz,
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

diesel::allow_tables_to_appear_in_same_query!(chat_messages, order_add_keys, users,);
