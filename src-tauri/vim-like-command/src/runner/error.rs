use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("problem in command: {0}")]
    Command(#[from] high_level::prelude::ClientError),

    #[error("not found command")]
    NotFoundCommand,

    #[error("two identical id's")]
    IdenticalId,
}

pub type VimError<T> = Result<T, Error>;
