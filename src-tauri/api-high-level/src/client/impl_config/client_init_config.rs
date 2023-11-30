use crate::client::error::Error;
use cache::prelude::{Cache, CacheSQLite};
use std::path::{Path, PathBuf};

/// Struct for storage args for init client
#[derive(Debug, Clone)]
pub struct ClientInitArgs {
    pub path_to_config_file: PathBuf,
    pub address_to_server: http::Uri,
    pub path_to_cache: Option<PathBuf>,
}

impl ClientInitArgs {
    pub fn new<P>(
        path_to_config_file: P,
        address_to_server: impl TryInto<http::Uri>,
        path_to_cache: Option<PathBuf>,
    ) -> Option<Self>
    where
        P: AsRef<Path>,
    {
        let Ok(address_to_server) = address_to_server.try_into() else {
            return None;
        };

        Some(Self {
            path_to_config_file: path_to_config_file.as_ref().into(),
            address_to_server,
            path_to_cache,
        })
    }

    pub async fn get_cache(&self) -> Result<Option<CacheSQLite>, Error> {
        match &self.path_to_cache {
            Some(path_to_cache) => Ok(Some(Cache::new(path_to_cache.clone()).await?)),
            None => Ok(None),
        }
    }
}
