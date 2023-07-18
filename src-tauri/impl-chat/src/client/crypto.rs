use ring::aead::{CHACHA20_POLY1305, OpeningKey, SealingKey, BoundKey};
use ring::pbkdf2::{derive, PBKDF2_HMAC_SHA256};
use ring::rand::{SystemRandom, SecureRandom};
use std::num::NonZeroU32;
use crate::client::error::Error;

#[derive(Debug, Default)]
pub struct Crypto {
    key: [u8; 32],
    
}

impl Crypto {
    fn generate_key(salt: &[u8]) -> Result<[u8; 32], Error> {
        let mut key = [0; 32];
        let secret = {
            let rand = SystemRandom::new();
            let mut vec = vec![0; 2048];
            rand.fill(&mut vec).map_err(|x| Error::GenerateClient(x))?;
    
            vec
        };
    
        derive(PBKDF2_HMAC_SHA256, NonZeroU32::new(100).unwrap(), salt, &secret[..], &mut key);
    
        Ok(key)
    }

    pub fn generate(salt: &[u8]) -> Result<Self, Error> {
        let key = Crypto::generate_key(salt)?;
        let opening_key = OpeningKey::new(CHACHA20_POLY1305, &key).unwrap();
        let sealing_key = SealingKey::new(CHACHA20_POLY1305, &key).unwrap();

        Ok(Self {
            key
        })
    }

    pub fn encrypt(&self, data: &Vec<u8>) {
        let additional_data: [u8; 0] = [];
        let mut in_out = data.clone();

        for _ in 0..CHACHA20_POLY1305.tag_len() {
            in_out.push(0);
        }


    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn generate_aes() {
        let key = super::Crypto::generate_key(b"test_salt").unwrap();

        println!("key:")
    }
}