use super::crypto::CryptoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("cryptography problem: `{0}`")]
    Crypto(#[from] CryptoError),

    #[error("transport problem: `{0}`")]
    Transport(#[from] tonic::transport::Error),

    #[error("nickname is taken")]
    NicknameIsTaken,

    #[error("api problem: `{0}`")]
    Api(#[from] tonic::Status),
}
