use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("key already exists")]
    AlreadyExists,

    #[error("not found")]
    NotFound,
}
