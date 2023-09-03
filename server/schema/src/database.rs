use crate::models::*;
use crate::schema::users::dsl::*;
use bb8::Pool;
use diesel::prelude::*;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use std::env;

pub type DbPool = Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
pub type PoolledDb<'a> = bb8::PooledConnection<
    'a,
    diesel_async::pooled_connection::AsyncDieselConnectionManager<diesel_async::AsyncPgConnection>,
>;

pub async fn establish_pooled_connection() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    Pool::builder().build(config).await.unwrap()
}

pub async fn get_user_by_id<'a>(db: &mut PoolledDb<'a>, user_id: i64) -> Vec<User> {
    users
        .filter(id.eq(user_id))
        .select(User::as_select())
        .load(db)
        .await
        .unwrap()
}

pub async fn get_user_by_nickname<'a>(db: &mut PoolledDb<'a>, user_nickname: &str) -> Vec<User> {
    users
        .filter(nickname.eq(user_nickname))
        .select(User::as_select())
        .load(db)
        .await
        .unwrap()
}

pub async fn check_user<'a>(db: &mut PoolledDb<'a>) -> bool {
    

    true
}