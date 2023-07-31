use std::path::PathBuf;
use lower_level::client::Client as RawClient;
use crate::config::Config;
use error::Error;

pub mod error;

pub struct ClientConfig {
    path_to_config_file: PathBuf,
    address_to_server: http::Uri
}

pub struct Client {
    raw_client: RawClient,
    config: Config
}

impl Client {
    pub async fn registration(nickname: &str, config: ClientConfig) -> Result<Self, Error> {
        Ok(Self {
            raw_client: RawClient::registration(nickname, config.address_to_server).await?,
            config: Config::new(config.path_to_config_file)
        })
    }

    pub async fn load(config: ClientConfig) -> Result<Self, Error> {
        let data = Config::new(config.path_to_config_file.clone()).load()?;
        let api = RawClient::api_connect(config.address_to_server.clone()).await?;
        
        if !*RawClient::check_valid(&data.nickname, &data.auth_key, config.address_to_server).await? {
            return Err(Error::AccoutIsInvalid);
        }

        Ok(Self {
            raw_client: RawClient {
                api,
                data
            },
            config: Config::new(config.path_to_config_file)
        })
    }

    pub fn save(&self) -> Result<(), Error> {
        Ok(self.config.save(&self.raw_client.data)?)
    }
}
