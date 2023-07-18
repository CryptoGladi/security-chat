use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("cryptography problem: `{0}`")]
    Crypto(aes_gcm::Error),
}
