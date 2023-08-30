use super::*;
use sqlx::sqlite::{SqliteAutoVacuum, SqliteConnectOptions};
use sqlx::{Pool, Sqlite, SqlitePool};

pub struct SQLite {
    db: Pool<Sqlite>,
}

#[async_trait]
impl DB for SQLite {
    async fn new(options: DBOptions) -> CacheResult<Self> {
        let db_options = SqliteConnectOptions::new()
            .filename(options.path)
            .create_if_missing(true)
            .auto_vacuum(SqliteAutoVacuum::Incremental)
            .optimize_on_close(true, None);

        let db_connection = SqlitePool::connect_with(db_options)
            .await
            .map_err(|x| Error::Db(x.into()))?;

        Ok(SQLite { db: db_connection })
    }

    async fn put(&mut self, chat_name: &str, data: Vec<u8>) -> CacheResult<Self> {
        todo!()
    }

    async fn get(&self, chat_name: &str, limit_desc: usize) -> CacheResult<Self> {
        todo!()
    }
}
