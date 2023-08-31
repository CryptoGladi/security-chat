//! # Main struct
//!
//! ## Example
//!
//! ```no_run
//! # use cache::cache_struct::Cache;
//! # use cache::cache_struct::db_trait::sqlite_impl::SQLite;
//! # use cache::cache_struct::db_trait::DBOptions;
//! #
//! let cache = Cache::<SQLite>::new("path_for_cache");
//! ```

pub mod db_trait;
pub mod error;

use db_trait::DBOptions;
use error::CacheResult;
use log::debug;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::path::Path;

#[derive(Debug)]
pub struct Cache<T: db_trait::DB> {
    db: T,
}

impl<T: db_trait::DB> Cache<T> {
    /// Create or open cache
    pub async fn new(path: impl AsRef<Path>) -> CacheResult<Self> {
        debug!("new database with path: {:?}", path.as_ref());

        Ok(Self {
            db: T::new(DBOptions::new(path)).await?,
        })
    }

    /// Add to new element to cache
    pub async fn put(&mut self, key: &str, value: &(impl Serialize + Debug)) -> CacheResult<()> {
        debug!("put for key: {}; value: {:?}", key, value);

        let bincode = bincode::serialize(value)?;
        self.db.put(key, bincode).await?;

        Ok(())
    }

    /// Get element from cache
    pub async fn get<D: DeserializeOwned>(
        &mut self,
        key: &str,
        limit_desc: usize,
    ) -> CacheResult<Vec<D>> {
        debug!("get for key: {}", key);

        let raw_bincode = self.db.get(key, limit_desc).await?;

        let mut bincode = Vec::with_capacity(raw_bincode.len());
        for x in raw_bincode.into_iter() {
            let y = bincode::deserialize::<D>(&x)?;
            bincode.push(y);
        };

        Ok(bincode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::warn;
    use temp_dir::TempDir;
    use test_log::test;
    use crate::prelude::CacheSQLite;
    use serde::Deserialize;

    type CacheForTest = CacheSQLite;

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct MessageBody {
        pub text: String
    }

    async fn create_cache() -> (TempDir, CacheForTest) {
        let temp_dir = TempDir::new().unwrap();
        let db = CacheForTest::new(temp_dir.child("cache.db"))
            .await
            .unwrap();

        (temp_dir, db)
    }

    #[test(tokio::test)]
    async fn test_create_cache() {
        let _ = create_cache().await;
    }

    #[test(tokio::test)]
    async fn new() {
        let temp_dir = TempDir::new().unwrap();
        let _sqlite = CacheForTest::new(temp_dir.child("cache.db"))
            .await
            .unwrap();
    }

    #[test(tokio::test)]
    async fn put_and_get() {
        let (_temp_dir, mut db) = create_cache().await;
        let message = MessageBody {
            text: "text_message".to_string()
        };

        db.put("nickname", &message).await.unwrap();

        assert_eq!(db.get::<MessageBody>("nickname", 1).await.unwrap()[0], message);
    }

    #[test(tokio::test)]
    async fn put_and_get_with_desc() {
        let (_temp_dir, mut db) = create_cache().await;
        let many_message = MessageBody {
            text: "SUPER SECRET DATA".to_string()
        };

        for _ in 0..100 {
            db.put("nickname", &many_message).await.unwrap();
        }

        let one_message = MessageBody {
            text: "many secret data".to_string()
        };
        db.put("nickname", &one_message).await.unwrap();

        assert_eq!(db.get::<MessageBody>("nickname", 100).await.unwrap()[0], one_message);
    }

    #[test(tokio::test)]
    async fn check_error_bincode() {
        let (_temp_dir, mut db) = create_cache().await;

        db.put("nickname", &120).await.unwrap();
        let error = db.get::<MessageBody>("nickname", 1).await.err().unwrap();

        if let crate::cache_struct::error::Error::Bincode(_) = error {
            warn!("Done!");
        } else {
            assert!(false);
        }
    }
}