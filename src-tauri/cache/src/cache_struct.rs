//! # Main struct
//!
//! ## Example
//!
//! ```no_run
//! # use cache::cache_struct::Cache;
//! # use cache::cache_struct::db_trait::sqlite_impl::SQLite;
//! # use cache::cache_struct::db_trait::DBOptions;
//! #
//! let cache = Cache::<SQLite>::new(DBOptions::new("path_for_cache"));
//! ```

pub mod db_trait;
pub mod error;

use std::fmt::Debug;

use db_trait::DBOptions;
use error::CacheResult;
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Cache<T: db_trait::DB> {
    db: T,
}

impl<T: db_trait::DB> Cache<T> {
    pub async fn new(options: DBOptions) -> CacheResult<Self> {
        debug!("new database with options: {:?}", options);

        Ok(Self {
            db: T::new(options).await?,
        })
    }

    pub async fn put(&mut self, chat: &str, data: impl Serialize + Debug) -> CacheResult<()> {
        debug!("put for chat: {}; data: {:?}", chat, data);

        let bincode = bincode::serialize(&data)?;
        self.db.put(chat, bincode).await?;

        Ok(())
    }
}
