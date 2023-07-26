use crate::client::crypto::{common::get_rand, CryptoError};
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, Nonce},
    Aes256Gcm, Key,
};
use log::info;
use serde::{Deserialize, Serialize};
use super::ecdh::SharedSecret;

pub const SIZE_KEY: usize = 256 / 8; // = 32

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AES {
    key: [u8; SIZE_KEY],
}

#[derive(Debug, Default)]
pub struct EncryptedMessage {
    data: Vec<u8>,
    nonce: Nonce<Aes256Gcm>,
}

impl AES {
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

        let data = cipher.encrypt(&nonce, message).map_err(CryptoError::AES)?;

        Ok(EncryptedMessage { data, nonce })
    }

    pub fn decrypt(&self, message: &EncryptedMessage) -> Result<Vec<u8>, CryptoError> {
        info!("decrypting message...");
        let key = Key::<Aes256Gcm>::from_slice(&self.key);
        let cipher = Aes256Gcm::new(key);

        let data = cipher
            .decrypt(&message.nonce, message.data.as_ref())
            .map_err(CryptoError::AES)?;

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

        let crypto = AES::generate();
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

        let crypto = AES::with_shared_key(shared_secret);
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
