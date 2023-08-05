use hashbrown::HashMap;
use lower_level::client::crypto::Aes;
use serde::{Deserialize, Serialize};
use error::Error;

pub mod error;

#[derive(Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Nickname(pub String);

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

        self.0.insert(nickname, aes);
        Ok(())
    }

    pub fn get(&self, nickname: &Nickname) -> Result<&Aes, Error> {
        match self.0.get(nickname) {
            Some(aes) => Ok(aes),
            None => Err(Error::NotFound)
        } 
    }
}