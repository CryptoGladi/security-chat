use crate::client::error::Error;
use cache::prelude::{Cache, CacheSQLite};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ClientInitConfig {
    pub path_to_config_file: PathBuf,
    pub path_to_cache: PathBuf,
    pub address_to_server: http::Uri,
    pub use_cache: bool,
}

impl ClientInitConfig {
    pub fn new(
        path_to_config_file: impl AsRef<Path>,
        path_to_cache: impl AsRef<Path>,
        address_to_server: impl TryInto<http::Uri>,
        use_cache: bool,
    ) -> Self {
        let Ok(address_to_server) = address_to_server.try_into() else {
            panic!("address_to_server.try_into() error");
        };

        Self {
            path_to_config_file: path_to_config_file.as_ref().into(),
            path_to_cache: path_to_cache.as_ref().into(),
            address_to_server,
            use_cache,
        }
    }
}

pub async fn get_cache(init_config: &ClientInitConfig) -> Result<Option<CacheSQLite>, Error> {
    Ok(if init_config.use_cache {
        Some(Cache::new(init_config.path_to_cache.clone()).await?)
    } else {
        None
    })
}
