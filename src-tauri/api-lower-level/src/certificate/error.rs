use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("network: {0}")]
    Network(#[from] reqwest::Error),

    #[error("hash is invalid")]
    InvalidHash,
}
