// @generated automatically by Diesel CLI.

diesel::table! {
    conversation (id) {
        id -> Int8,
        #[max_length = 40]
        title -> Varchar,
    }
}

diesel::table! {
    conversation_messages (id) {
        id -> Int8,
        conversation_id -> Int8,
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
        user_from_public_key -> Bytea,
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

diesel::joinable!(conversation_messages -> conversation (conversation_id));
diesel::joinable!(conversation_messages -> users (sender_id));

diesel::allow_tables_to_appear_in_same_query!(
    conversation,
    conversation_messages,
    order_add_keys,
    users,
);
