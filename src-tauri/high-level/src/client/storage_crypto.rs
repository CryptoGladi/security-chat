use error::Error;
use hashbrown::HashMap;
use lower_level::client::crypto::Aes;
use serde::{Deserialize, Serialize};

pub mod error;

#[derive(Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Nickname(pub String);

impl From<String> for Nickname {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Nickname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Deref for Nickname {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Nickname {
    pub fn get_user() -> Self {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct StorageCrypto(pub HashMap<Nickname, Aes>);

impl StorageCrypto {
    pub fn add(&mut self, nickname: Nickname, aes: Aes) -> Result<(), Error> {
        if self.0.contains_key(&nickname) {
            return Err(Error::AlreadyExists);
        }

        self.0.insert_unique_unchecked(nickname, aes);
        Ok(())
    }

    pub fn get(&self, nickname: &Nickname) -> Result<&Aes, Error> {
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
        let nickname = Nickname::from("da".to_string());

        storage_crypto.add(nickname.clone(), aes).unwrap();
        assert_eq!(*storage_crypto.get(&nickname).unwrap(), aes);
    }

    #[test]
    fn get_not_found() {
        let storage_crypto = StorageCrypto::default();

        assert_eq!(
            storage_crypto
                .get(&Nickname("ss".to_string()))
                .err()
                .unwrap(),
            Error::NotFound
        );
    }

    #[test]
    fn add_already_exists() {
        let mut storage_crypto = StorageCrypto::default();
        let nickname = Nickname::from("irlik".to_string());

        storage_crypto
            .add(nickname.clone(), Aes::generate())
            .unwrap();

        assert_eq!(
            storage_crypto.add(nickname, Aes::generate()).err().unwrap(),
            Error::AlreadyExists
        );
    }
}
