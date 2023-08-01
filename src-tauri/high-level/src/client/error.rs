use thiserror::Error;

use super::storage_crypto::Nickname;

#[derive(Error, Debug)]
pub enum Error {
    #[error("from client: {0}")]
    Client(#[from] lower_level::client::error::Error),

    #[error("from config: {0}")]
    Config(#[from] crate::bincode_config::Error),

    #[error("account is invalid")]
    AccoutIsInvalid,

    #[error("nickname is same: {0}")]
    NicknameSame(Nickname),

    #[error("storage already have nickname: {0}")]
    StorageAlreadyHaveNickname(Nickname),
}
