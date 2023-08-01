use hashbrown::HashMap;
use lower_level::client::crypto::Aes;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct StorageCrypto {
    aes: HashMap<Nickname, Aes>,
}

impl Nickname {
    pub fn get_user() -> Self {
        todo!()
    }
}
