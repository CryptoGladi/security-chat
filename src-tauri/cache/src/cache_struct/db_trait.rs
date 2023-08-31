//! Trait for database engine key - value with desc

pub mod sqlite_impl;

use super::error::{CacheResult, Error};
use async_trait::async_trait;
use std::path::{Path, PathBuf};

/// Options for creating database
#[derive(Debug)]
pub struct DBOptions {
    /// Path to database
    path: PathBuf,
}

impl DBOptions {
    pub fn new(path: impl AsRef<Path>) -> DBOptions {
        DBOptions {
            path: path.as_ref().into(),
        }
    }
}

/// Database engine
///
/// For error handling use [crate::cache_struct::error::Error::Db]
#[async_trait]
pub trait DB
where
    Self: Sized,
{
    /// Create a new database
    async fn new(options: DBOptions) -> CacheResult<Self>;

    /// Put a new element
    async fn put(&mut self, key: &str, value: Vec<u8>) -> CacheResult<()>;

    /// Get element
    async fn get(&self, key: &str, limit_desc: usize) -> CacheResult<Vec<Vec<u8>>>;
}

pub use sqlite_impl::SQLite;
