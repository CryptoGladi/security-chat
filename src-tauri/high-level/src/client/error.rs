use super::storage_crypto::Nickname;
use lower_level::client::crypto::CryptoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("from client: `{0}`")]
    Client(#[from] lower_level::client::error::Error),

    #[error("from config: `{0}`")]
    Config(#[from] crate::bincode_config::Error),

    #[error("account is invalid")]
    AccoutIsInvalid,

    #[error("nickname is same: `{0}`")]
    NicknameSame(Nickname),

    #[error("storage already have nickname: `{0}`")]
    StorageAlreadyHaveNickname(Nickname),

    #[error("problem in storage crypto: `{0}`")]
    StorageCrypto(#[from] crate::client::storage_crypto::error::Error),

    #[error("cryptography problem: `{0}`")]
    Crypto(#[from] CryptoError),

    #[error("problem in bincode: `{0}`")]
    Bincode(#[from] bincode::Error),
}
