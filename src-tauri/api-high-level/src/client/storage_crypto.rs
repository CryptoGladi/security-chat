use api_lower_level::client::impl_crypto::aes::Aes;
use error::Error;
use hashbrown::HashMap;
use log::info;
use serde::{Deserialize, Serialize};

pub mod error;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct StorageCrypto(pub HashMap<String, Aes>);

impl StorageCrypto {
    pub fn add(&mut self, nickname: String, aes: Aes) -> Result<(), Error> {
        info!("adding new key for {}", nickname);

        if self.0.contains_key(&nickname) {
            return Err(Error::AlreadyExists);
        }

        self.0.insert_unique_unchecked(nickname, aes);
        Ok(())
    }

    pub fn get(&self, nickname: &str) -> Result<&Aes, Error> {
        match self.0.get(nickname) {
            Some(aes) => Ok(aes),
            None => Err(Error::NotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn add_and_get() {
        let mut storage_crypto = StorageCrypto::default();
        let aes = Aes::generate();
        let nickname = "da".to_string();

        storage_crypto.add(nickname.clone(), aes).unwrap();
        assert_eq!(*storage_crypto.get(&nickname).unwrap(), aes);
    }

    #[test]
    fn get_not_found() {
        let storage_crypto = StorageCrypto::default();

        assert_eq!(storage_crypto.get("ss").err().unwrap(), Error::NotFound);
    }

    #[test]
    fn add_already_exists() {
        let mut storage_crypto = StorageCrypto::default();
        let nickname = "irlik".to_string();

        storage_crypto
            .add(nickname.clone(), Aes::generate())
            .unwrap();

        assert_eq!(
            storage_crypto.add(nickname, Aes::generate()).err().unwrap(),
            Error::AlreadyExists
        );
    }
}
