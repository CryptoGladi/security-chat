use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("AES problem: {0}")]
    Aes(aes_gcm::Error),

    #[error("ECDH problem: {0}")]
    Ecdh(p384::elliptic_curve::Error),

    #[error("key has already been accepted from user: {0}")]
    KeyAlreadyAccepted(String),
}
