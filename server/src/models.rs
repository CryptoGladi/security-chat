use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub nickname: String,
    pub authkey: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub nickname: &'a str,
    pub authkey: &'a str,
}

#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::order_add_keys)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Key {
    pub id: i64,
    pub user_to_id: i64,
    pub user_from_id: i64,
    pub user_to_public_key: Vec<u8>,
    pub user_from_public_key: Option<Vec<u8>>,
}

#[derive(Insertable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = crate::schema::order_add_keys)]
pub struct NewKey {
    pub user_to_id: i64,
    pub user_from_id: i64,
    pub user_to_public_key: Vec<u8>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::chat_messages)]
pub struct AddMessage {
    pub sender_id: i64,
    pub recipient_id: i64,
    pub message_body: Vec<u8>,
}
