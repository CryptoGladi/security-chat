//! All modules needed for **key exchange**.
//! Algorithm of work is described [here](crate::client::impl_crypto)

pub mod aes;
pub mod ecdh;
pub mod error;

pub use aes::{Aes, EncryptedMessage};
pub use error::CryptoError;
