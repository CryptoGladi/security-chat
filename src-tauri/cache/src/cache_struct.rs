//! # Main struct
//! 
//! ## Example
//! 
//! ```no_run
//! # use cache::cache_struct::Cache;
//! # use cache::cache_struct::db_trait::sqlite_impl::SQLite;
//! # use cache::cache_struct::db_trait::DBOptions;
//! #
//! let cache = Cache::<SQLite>::new(DBOptions::new("path"));
//! ```

pub mod db_trait;
pub mod error;

use error::CacheResult;

#[derive(Debug)]
pub struct Cache<T: db_trait::DB> {
    db: T,
}

impl<T: db_trait::DB> Cache<T> {
    pub async fn new(options: db_trait::DBOptions) -> CacheResult<Self> {
        Ok(Self {
            db: T::new(options).await?,
        })
    }

    pub fn add(&self) -> CacheResult<()> {
        Ok(())
    }
}
