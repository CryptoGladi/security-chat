use crate::command::CommandError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// The error happened within the command.
    #[error("problem in command: {0}")]
    Command(#[from] CommandError),

    #[error("not found command")]
    NotFoundCommand,

    #[error("two identical id's")]
    IdenticalId,
}

pub type VimError<T> = Result<T, Error>;
