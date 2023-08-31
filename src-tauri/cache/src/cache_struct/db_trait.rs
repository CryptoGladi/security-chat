pub mod sqlite_impl;

use super::error::{CacheResult, Error};
use async_trait::async_trait;
use std::path::{Path, PathBuf};

pub struct DBOptions {
    path: PathBuf,
}

impl DBOptions {
    pub fn new(path: impl AsRef<Path>) -> DBOptions {
        DBOptions {
            path: path.as_ref().into(),
        }
    }
}

#[async_trait]
pub trait DB
where
    Self: Sized,
{
    async fn new(options: DBOptions) -> CacheResult<Self>;

    async fn put(&mut self, chat_name: &str, data: Vec<u8>) -> CacheResult<()>;

    async fn get(&self, chat_name: &str, limit_desc: usize) -> CacheResult<Vec<u8>>;
}

pub use sqlite_impl::SQLite;
