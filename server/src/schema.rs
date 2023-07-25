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
        #[max_length = 40]
        sender_nickname -> Nullable<Varchar>,
        message -> Json,
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

diesel::allow_tables_to_appear_in_same_query!(
    conversation,
    conversation_messages,
    users,
);
