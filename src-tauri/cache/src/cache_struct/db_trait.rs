pub mod sqlite_impl;

use super::error::{CacheResult, Error};
use async_trait::async_trait;
use std::path::PathBuf;

pub struct DBOptions {
    path: PathBuf,
}

#[async_trait]
pub trait DB
where
    Self: Sized,
{
    async fn new(options: DBOptions) -> CacheResult<Self>;

    async fn put(&mut self, chat_name: &str, data: Vec<u8>) -> CacheResult<Self>;

    async fn get(&self, chat_name: &str, limit_desc: usize) -> CacheResult<Self>;
}

pub use sqlite_impl::SQLite;