use super::ecdh::SharedSecret;
use crate::client::crypto::{common::get_rand, CryptoError};
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, Nonce},
    Aes256Gcm, Key,
};
use log::info;
use serde::{Deserialize, Serialize};

pub const SIZE_KEY: usize = 256 / 8; // = 32
pub const SIZE_NONCE: usize = 96 / 8; // = 12

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
    pub fn generate() -> Self {
        info!("generating key...");
        let key_array = Aes256Gcm::generate_key(get_rand());
        let key: [u8; SIZE_KEY] = key_array.try_into().unwrap();

        Self { key }
    }

    pub fn with_shared_key(shared_secret: SharedSecret) -> Self {
        let key: [u8; SIZE_KEY] = shared_secret.get_key_for_aes_256().try_into().unwrap();

        Self { key }
    }

    pub fn encrypt(&self, message: &[u8]) -> Result<EncryptedMessage, CryptoError> {
        info!("encypting message...");
        let key = Key::<Aes256Gcm>::from_slice(&self.key);
        let cipher = Aes256Gcm::new(key);
        let nonce = Aes256Gcm::generate_nonce(&mut get_rand());

        let data = cipher.encrypt(&nonce, message).map_err(CryptoError::Aes)?;

        Ok(EncryptedMessage {
            data,
            nonce: nonce.try_into().unwrap(),
        })
    }

    pub fn decrypt(&self, message: &EncryptedMessage) -> Result<Vec<u8>, CryptoError> {
        info!("decrypting message...");
        let key = Key::<Aes256Gcm>::from_slice(&self.key);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::<Aes256Gcm>::clone_from_slice(&message.nonce);

        let data = cipher
            .decrypt(&nonce, message.data.as_ref())
            .map_err(CryptoError::Aes)?;

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::crypto::ecdh::{get_public_info, get_shared_secret};

    #[test]
    fn with_generate() {
        const MESSAGE_FOR_CRYPTO: &[u8] = b"test message";

        let crypto = Aes::generate();
        let encrypted_message = crypto.encrypt(MESSAGE_FOR_CRYPTO).unwrap();
        let decrypted_message = crypto.decrypt(&encrypted_message).unwrap();

        println!(
            "MESSAGE_FOR_CRYPTO: {}",
            String::from_utf8(MESSAGE_FOR_CRYPTO.to_vec()).unwrap()
        );
        println!(
            "decrypted_message: {}",
            String::from_utf8(decrypted_message.clone()).unwrap()
        );
        assert_eq!(MESSAGE_FOR_CRYPTO, decrypted_message);
    }

    #[test]
    fn with_ecdh() {
        const MESSAGE_FOR_CRYPTO: &[u8] = b"test message";
        let (alice_secret, _) = get_public_info().unwrap();
        let (_, bob_public_key) = get_public_info().unwrap();

        let shared_secret = get_shared_secret(&alice_secret, &bob_public_key);

        let crypto = Aes::with_shared_key(shared_secret);
        let encrypted_message = crypto.encrypt(MESSAGE_FOR_CRYPTO).unwrap();
        let decrypted_message = crypto.decrypt(&encrypted_message).unwrap();

        println!(
            "MESSAGE_FOR_CRYPTO: {}",
            String::from_utf8(MESSAGE_FOR_CRYPTO.to_vec()).unwrap()
        );
        println!(
            "decrypted_message: {}",
            String::from_utf8(decrypted_message.clone()).unwrap()
        );
        assert_eq!(MESSAGE_FOR_CRYPTO, decrypted_message);
    }
}
