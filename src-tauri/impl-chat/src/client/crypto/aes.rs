use crate::client::error::Error;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, Nonce, OsRng},
    Aes256Gcm, Key,
};
use log::info;
use serde::{Deserialize, Serialize};

pub const SIZE_KEY: usize = 256 / 8; // = 32

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AES {
    pub key: [u8; SIZE_KEY],
}

pub struct EncryptedMessage {
    data: Vec<u8>,
    nonce: Nonce<Aes256Gcm>,
}

impl AES {
    pub fn generate() -> Self {
        info!("generating key...");
        let key_array = Aes256Gcm::generate_key(OsRng);
        let key: [u8; SIZE_KEY] = key_array.try_into().unwrap();

        Self { key }
    }

    pub fn encrypt(&self, message: &[u8]) -> Result<EncryptedMessage, Error> {
        info!("encypting message...");
        let key = Key::<Aes256Gcm>::from_slice(&self.key);
        let cipher = Aes256Gcm::new(key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let data = cipher.encrypt(&nonce, message).map_err(Error::Crypto)?;

        Ok(EncryptedMessage { data, nonce })
    }

    pub fn decrypt(&self, message: &EncryptedMessage) -> Result<Vec<u8>, Error> {
        info!("decrypting message...");
        let key = Key::<Aes256Gcm>::from_slice(&self.key);
        let cipher = Aes256Gcm::new(key);

        let data = cipher
            .decrypt(&message.nonce, message.data.as_ref())
            .map_err(Error::Crypto)?;

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crypto() {
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
}
