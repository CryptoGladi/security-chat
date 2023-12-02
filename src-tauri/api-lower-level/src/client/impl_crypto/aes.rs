//! Module for [AES-256-GCM](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard)

use crate::client::impl_crypto::error::Error;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, Nonce},
    Aes256Gcm, Key,
};
use fcore::prelude::get_crypto;
use log::{debug, trace};
use p384::ecdh::SharedSecret;
use serde::{Deserialize, Serialize};

pub const SIZE_KEY: usize = 256 / 8; // = 32 (for AES-256)
pub const SIZE_NONCE: usize = 96 / 8; // = 12 (for AES-256)

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Aes {
    key: [u8; SIZE_KEY],
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EncryptedMessage {
    pub data: Vec<u8>,
    pub nonce: [u8; SIZE_NONCE],
}

impl Aes {
    /// Generate key by [`get_crypto`]
    ///
    /// # Panics
    ///
    /// **Impossible**, but only if the constants have the correct `SIZE_KEY` value
    #[must_use]
    pub fn generate() -> Self {
        trace!("generating key...");

        let key_array = Aes256Gcm::generate_key(get_crypto());
        let key: [u8; SIZE_KEY] = key_array.try_into().unwrap();

        Self { key }
    }

    /// Import key by [`SharedSecret`]
    ///
    /// # Panics
    ///
    /// If the [`SharedSecret`] is incorrect or corrupted, there will be a panic
    #[must_use]
    pub fn with_shared_key(shared_secret: &SharedSecret) -> Self {
        trace!("with_shared_key");

        let key: [u8; 32] = shared_secret.raw_secret_bytes()[..SIZE_KEY]
            .try_into()
            .unwrap();

        Self { key }
    }

    /// Encrypting message
    ///
    /// # Panics
    ///
    /// **Impossible**, but only if the constants have the correct `SIZE_KEY` value
    #[allow(clippy::unwrap_in_result)]
    pub fn encrypt(&self, message: &[u8]) -> Result<EncryptedMessage, Error> {
        debug!("encypting message...");

        let key = Key::<Aes256Gcm>::from_slice(&self.key);
        let cipher = Aes256Gcm::new(key);
        let nonce = Aes256Gcm::generate_nonce(&mut get_crypto());

        let data = cipher.encrypt(&nonce, message).map_err(Error::Aes)?;

        Ok(EncryptedMessage {
            data,
            nonce: nonce.try_into().unwrap(),
        })
    }

    pub fn decrypt(&self, message: &EncryptedMessage) -> Result<Vec<u8>, Error> {
        debug!("decrypting message...");

        let key = Key::<Aes256Gcm>::from_slice(&self.key);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::<Aes256Gcm>::clone_from_slice(&message.nonce);

        let data = cipher
            .decrypt(&nonce, message.data.as_ref())
            .map_err(Error::Aes)?;

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::impl_crypto::ecdh::{get_public_info, get_shared_secret};

    fn decrypt(crypto: &Aes, message_for_crypto: &[u8]) -> Vec<u8> {
        let encrypted_message = crypto.encrypt(message_for_crypto).unwrap();
        let decrypted_message = crypto.decrypt(&encrypted_message).unwrap();

        log::trace!(
            "MESSAGE_FOR_CRYPTO: {}",
            String::from_utf8(message_for_crypto.to_vec()).unwrap()
        );

        log::trace!(
            "decrypted_message: {}",
            String::from_utf8(decrypted_message.clone()).unwrap()
        );

        decrypted_message
    }

    #[test]
    fn with_generate() {
        const MESSAGE_FOR_CRYPTO: &[u8] = b"test message";

        let crypto = Aes::generate();
        let decrypted_message = decrypt(&crypto, MESSAGE_FOR_CRYPTO);

        assert_eq!(MESSAGE_FOR_CRYPTO, decrypted_message);
    }

    #[test]
    fn with_ecdh() {
        const MESSAGE_FOR_CRYPTO: &[u8] = b"test message";
        let (alice_secret, _) = get_public_info().unwrap();
        let (_, bob_public_key) = get_public_info().unwrap();

        let shared_secret = get_shared_secret(&alice_secret, &bob_public_key);

        let crypto = Aes::with_shared_key(&shared_secret);
        let decrypted_message = decrypt(&crypto, MESSAGE_FOR_CRYPTO);

        assert_eq!(MESSAGE_FOR_CRYPTO, decrypted_message);
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: Aes(Error)")]
    fn frong_key() {
        const MESSAGE_FOR_CRYPTO: &[u8] = b"super secret message";

        let crypto1 = Aes::generate();
        let crypto2 = Aes::generate();

        let encrypted_message = crypto1.encrypt(MESSAGE_FOR_CRYPTO).unwrap();
        let _decrypted_message = crypto2.decrypt(&encrypted_message).unwrap();

        // assert_eq!(matches!(decrypted_message, Aes));
    }
}
