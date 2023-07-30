use std::path::{Path, PathBuf};

use lower_level::client::error::Error;
use lower_level::client::Client as RawClient;

use crate::config::Config;

pub struct ClientConfig {
    path_to_config_file: PathBuf
}

pub struct Client {
    raw_client: RawClient,
    config: Config
}

impl Client {
    pub async fn registration(nickname: &str, config: ClientConfig) -> Result<Self, Error> {
        Ok(Self {
            raw_client: RawClient::registration(nickname).await?,
            config: Config::new(config.path_to_config_file)
        })
    }
}
