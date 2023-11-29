//! [SQLite](https://www.sqlite.org/index.html) database engine

use super::{async_trait, CacheResult, DBOptions, Error, DB};
use log::{debug, trace};
use sqlx::sqlite::{SqliteAutoVacuum, SqliteConnectOptions};
use sqlx::{Pool, Row, Sqlite, SqlitePool};

/// All SQL commands
mod sql_command {
    use const_format::formatcp;

    pub const TABLE_NAME: &str = "messages";

    pub const CREATE_TABLE: &str = formatcp!(
        "CREATE TABLE IF NOT EXISTS {} (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        key TEXT NOT NULL,
        body BYTEA NOT NULL
    );",
        TABLE_NAME
    );

    pub const INSERT_INTO: &str =
        formatcp!("INSERT INTO {} (key, body) VALUES ($1, $2)", TABLE_NAME);
}

#[derive(Debug)]
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
            debug!("creating table!");

            sqlx::query(sql_command::CREATE_TABLE)
                .execute(&db_connection)
                .await
                .map_err(|x| Error::Db(x.into()))?;
        }

        Ok(SQLite { db: db_connection })
    }

    async fn put(&mut self, key: &str, value: Vec<u8>) -> CacheResult<()> {
        trace!("put with key: {}; value: IS BINARY!", key);

        sqlx::query(sql_command::INSERT_INTO)
            .bind(key)
            .bind(value)
            .execute(&self.db)
            .await
            .map_err(|x| Error::Db(x.into()))?;

        Ok(())
    }

    async fn get(&self, key: &str, limit_desc: usize) -> CacheResult<Vec<Vec<u8>>> {
        trace!("get with key: {}; limit_desc: {}", key, limit_desc);

        let sql = format!(
            "SELECT * FROM {} WHERE key = '{}' ORDER BY id DESC LIMIT {};",
            sql_command::TABLE_NAME,
            key,
            limit_desc
        );

        let stream = sqlx::query(&sql)
            .fetch_all(&self.db)
            .await
            .map_err(|x| Error::Db(x.into()))?;

        Ok(stream
            .into_iter()
            .map(|x| {
                let y: Vec<u8> = x.get("body");
                y
            })
            .collect())
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
        let _sqlite = SQLite::new(DBOptions::new(temp_dir.child("database.sqlite")))
            .await
            .unwrap();
    }

    #[test(tokio::test)]
    async fn put() {
        let (_temp_dir, mut sqlite) = create_database().await;
        sqlite.put("cs", b"cs".to_vec()).await.unwrap();
    }

    #[test(tokio::test)]
    async fn get() {
        let (_temp_dir, mut sqlite) = create_database().await;
        sqlite.put("ke", b"value".to_vec()).await.unwrap();

        let data = sqlite.get("ke", 100).await.unwrap();

        assert_eq!(data[0], b"value");
        assert_eq!(data.len(), 1);
    }

    #[test(tokio::test)]
    async fn get_check_desc() {
        let (_temp_dir, mut sqlite) = create_database().await;

        for _ in 0..100 {
            sqlite.put("ke", b"many_values".to_vec()).await.unwrap();
        }

        sqlite.put("ke", b"one_value".to_vec()).await.unwrap();
        let data = sqlite.get("ke", 1000).await.unwrap();

        assert_eq!(data[0], b"one_value".to_vec());
    }

    #[test(tokio::test)]
    async fn get_check_limit() {
        let (_temp_dir, mut sqlite) = create_database().await;
        let limit_desc: usize = 10;

        for _ in 0..100 {
            sqlite.put("ke", b"many_values".to_vec()).await.unwrap();
        }

        let data = sqlite.get("ke", limit_desc).await.unwrap();
        assert_eq!(data.len(), limit_desc);
    }

    #[test(tokio::test)]
    async fn check_zero_limit() {
        let (_temp_dir, mut sqlite) = create_database().await;

        for _ in 0..100 {
            sqlite.put("ke", b"many_values".to_vec()).await.unwrap();
        }

        let data = sqlite.get("ke", 0).await.unwrap();
        assert!(data.is_empty());
    }

    #[test(tokio::test)]
    async fn check_one_limit() {
        let (_temp_dir, mut sqlite) = create_database().await;
        sqlite.put("ke", b"1".to_vec()).await.unwrap();
        sqlite.put("ke", b"2".to_vec()).await.unwrap();

        assert_eq!(sqlite.get("ke", 1).await.unwrap().first().unwrap(), b"2");
    }
}
