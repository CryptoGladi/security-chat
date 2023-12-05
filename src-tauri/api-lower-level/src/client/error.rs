use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("cryptography problem: `{0}`")]
    Crypto(#[from] crate::client::impl_crypto::error::Error),

    #[error("problem in authentication: `{0}`")]
    Authentication(#[from] crate::authentication::error::Error),

    #[error("transport problem: `{0}`")]
    Transport(#[from] tonic::transport::Error),

    #[error("nickname is taken")]
    NicknameIsTaken,

    #[error("api problem: `{0}`")]
    Api(#[from] tonic::Status),

    #[error("too big message")]
    TooBigMessage,

    #[error("invalid argument for function: `{0}`")]
    InvalidArgument(&'static str),
}
