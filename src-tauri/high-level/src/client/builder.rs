//! Builder for [client](crate::client::Client)

pub(crate) mod info_struct;

use std::path::PathBuf;
use info_struct::*;
use super::Client;

#[derive(Debug)]
pub struct ClientBuilder {
    pub cache: Option<CacheInfo>
}

impl ClientBuilder {
    fn new() -> Self {
        ClientBuilder { 
            cache: None,
        }
    }

    fn cache(mut self, path: PathBuf) -> Self {
        self.cache = Some(CacheInfo { path: path });
        self
    } 

    fn build(self) -> Client {
        // Client { raw_client: (), config: (), _cache: (), init_config: () }
        todo!()
    }
}