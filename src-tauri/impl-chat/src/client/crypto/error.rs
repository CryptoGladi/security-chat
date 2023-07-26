use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("AES problem: {0}")]
    AES(aes_gcm::Error),

    #[error("ECDH problem: {0}")]
    ECDH(p384::elliptic_curve::Error),
}
