use crate::client::{impl_message::Message, storage_crypto::Nickname};
use cache::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CachedMessage {
    message: Message,
    chat: String, // TODO
}

#[derive(Debug)]
pub struct CacheMessage {
    pub impl_cache: CacheStruct,
}

impl CacheMessage {
    pub fn new<P>(path: P) -> Result<Self, CacheStructError>
    where
        P: AsRef<std::path::Path>,
    {
        Ok(Self {
            impl_cache: CacheStruct::new(path)?,
        })
    }

    pub fn put(&mut self, nickname: Nickname, message: &Message) -> Result<(), CacheStructError> {
        self.impl_cache.put(&nickname, message)?;
        Ok(())
    }

    pub fn get(&self, nickname: Nickname) -> Result<Message, CacheStructError> {
        self.impl_cache.get(&nickname)
    }
}
