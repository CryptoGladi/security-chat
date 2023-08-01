use hashbrown::HashMap;
use lower_level::client::crypto::Aes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Nickname(pub String);

impl std::ops::Deref for Nickname {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Nicknames {
    aes: HashMap<Nickname, Aes>,
}

impl Nickname {
    pub fn get_user() -> Self {
        todo!()
    }
}
