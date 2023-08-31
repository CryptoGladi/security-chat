//! Errors

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// Error in database. Use only in [`db_trait`](crate::cache_struct::db_trait)
    #[error("db problem: `{0}`")]
    Db(anyhow::Error),

    /// Error in [bindcode](https://github.com/bincode-org/bincode)
    #[error("problem in bincode: `{0}`")]
    Bincode(#[from] bincode::Error),
}

pub type CacheResult<T> = Result<T, Error>;
