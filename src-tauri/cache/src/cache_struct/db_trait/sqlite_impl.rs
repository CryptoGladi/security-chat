//! [SQLite](https://www.sqlite.org/index.html) database engine

use super::*;
use sqlx::sqlite::{SqliteAutoVacuum, SqliteConnectOptions};
use sqlx::{Pool, Sqlite, SqlitePool};
use log::{trace, warn};

mod sql_command {
    use const_format::formatcp;

    pub const TABLE_NAME: &str = "messages";

    pub const CREATE_TABLE: &str = formatcp!(
        "CREATE TABLE IF NOT EXISTS {} (
        id BIGSERIAL,
        key TEXT NOT NULL,
        by_nickname TEXT NOT NULL,
        body BYTEA NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
    );",
        TABLE_NAME
    );

    pub const INSERT_INTO: &str = formatcp!(
        "INSERT INTO {} (key, by_nickname, body) VALUES ($1, $2, $3)",
        TABLE_NAME
    );

    #[derive(sqlx::FromRow)]
    pub struct Message {
        id: i64,
        key: String,
        by_nickname: String,
        body: Vec<u8>,
    }
}

pub struct SQLite {
    db: Pool<Sqlite>,
}

#[async_trait]
impl DB for SQLite {
    async fn new(options: DBOptions) -> CacheResult<Self> {
        let already_exists = options.path.is_file();
        trace!("new with already_exists: {}", already_exists);

        let db_options = SqliteConnectOptions::new()
            .filename(options.path)
            .create_if_missing(true)
            .auto_vacuum(SqliteAutoVacuum::Incremental)
            .optimize_on_close(true, None);

        let db_connection = SqlitePool::connect_with(db_options)
            .await
            .map_err(|x| Error::Db(x.into()))?;

        if !already_exists {
            warn!("creating table!");

            sqlx::query(sql_command::CREATE_TABLE)
                .execute(&db_connection)
                .await
                .map_err(|x| Error::Db(x.into()))?;
        }

        Ok(SQLite { db: db_connection })
    }

    async fn put(&mut self, key: &str, data: Vec<u8>) -> CacheResult<()> {
        trace!("put with key: {}; data: IS BINARY!", key);

        sqlx::query(sql_command::INSERT_INTO)
            .bind(key)
            .bind("csa") // TODO
            .bind(data)
            .execute(&self.db)
            .await
            .map_err(|x| Error::Db(x.into()))?;

        Ok(())
    }

    async fn get(&self, key: &str, limit_desc: usize) -> CacheResult<Vec<u8>> {
        trace!("get with key: {}; limit_desc: {}", key, limit_desc);

        let sql = format!(
            "SELECT * FROM {} WHERE key = {} ORDER BY created_at DESC LIMIT {};",
            sql_command::TABLE_NAME,
            key,
            limit_desc
        );

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_dir::TempDir;
    use test_log::test;

    async fn create_database() -> (TempDir, SQLite) {
        let temp_dir = TempDir::new().unwrap();
        let sqlite = SQLite::new(DBOptions::new(temp_dir.child("database.sqlite")))
            .await
            .unwrap();

        (temp_dir, sqlite)
    }

    #[test(tokio::test)]
    async fn test_create_database() {
        let _ = create_database().await;
    }

    #[test(tokio::test)]
    async fn new() {
        let temp_dir = TempDir::new().unwrap();
        let sqlite = SQLite::new(DBOptions::new(temp_dir.child("database.sqlite")))
            .await
            .unwrap();
    }

    #[test(tokio::test)]
    async fn put() {
        let (_temp_dir, mut sqlite) = create_database().await;
        sqlite.put("cs", b"cs".to_vec()).await.unwrap();
    }
}
