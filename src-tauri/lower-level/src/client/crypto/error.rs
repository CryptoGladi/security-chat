use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum CryptoError {
    #[error("AES problem: {0}")]
    Aes(aes_gcm::Error),

    #[error("ECDH problem: {0}")]
    Ecdh(p384::elliptic_curve::Error),
}
