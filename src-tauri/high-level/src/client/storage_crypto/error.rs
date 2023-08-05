use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("key already exists")]
    AlreadyExists,

    #[error("not found")]
    NotFound
}