use hashbrown::HashMap;
use lower_level::client::crypto::Aes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Username(pub String);

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Usernames {
    aes: HashMap<Username, Aes>,
}

impl Username {
    pub fn get_user() -> Self {
        todo!()
    }
}
