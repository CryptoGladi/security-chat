use diesel::pg::PgConnection;
use diesel::r2d2;
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_pooled_connection() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}