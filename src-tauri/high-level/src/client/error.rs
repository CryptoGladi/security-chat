use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("from client: {0}")]
    Client(#[from] lower_level::client::error::Error),

    #[error("from config: {0}")]
    Config(#[from] crate::config::Error),

    #[error("account is invalid")]
    AccoutIsInvalid,
}
