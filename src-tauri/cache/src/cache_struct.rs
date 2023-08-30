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