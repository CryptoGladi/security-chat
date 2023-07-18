pub(crate) mod crypto;
pub mod error;

use std::collections::{HashMap};

use crypto::Crypto;

pub struct Client {
    pub cryptos: HashMap<String, Crypto>,
    pub username: String,
}

impl Client {
    async fn registration(username: String) {
        todo!()
    }
}
