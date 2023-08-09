use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("db problem: `{0}`")]
    Db(#[from] rocksdb::Error),

    #[error("problem in bincode: `{0}`")]
    Bincode(#[from] bincode::Error),

    #[error("not found")]
    NotFound,
}

pub type CacheResult<T> = Result<T, Error>;
