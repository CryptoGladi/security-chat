use crate::bincode_config;
use client_config::{ClientConfig, ClientInitConfig};
use error::Error;
use lower_level::client::Client as RawClient;
use storage_crypto::Nickname;

pub mod client_config;
pub mod error;
pub mod storage_crypto;

pub struct Client {
    raw_client: RawClient,
    config: ClientConfig,
    init_config: ClientInitConfig,
}

impl Client {
    pub async fn registration(
        nickname: &str,
        init_config: ClientInitConfig,
    ) -> Result<Client, Error> {
        let raw_client =
            RawClient::registration(nickname, init_config.address_to_server.clone()).await?;

        Ok(Self {
            config: ClientConfig {
                client_data: raw_client.data.clone(),
                ..Default::default()
            },
            init_config,
            raw_client,
        })
    }

    pub async fn load(init_config: ClientInitConfig) -> Result<Client, Error> {
        let config: ClientConfig = bincode_config::load(init_config.path_to_config_file.clone())?;
        let api = RawClient::api_connect(init_config.address_to_server.clone()).await?;

        if !*RawClient::check_valid(
            &config.client_data.nickname,
            &config.client_data.auth_key,
            init_config.address_to_server.clone(),
        )
        .await?
        {
            return Err(Error::AccoutIsInvalid);
        }

        Ok(Self {
            raw_client: RawClient {
                api,
                data: config.client_data.clone(),
            },
            config,
            init_config,
        })
    }

    pub fn save(&self) -> Result<(), Error> {
        bincode_config::save(&self.config, &self.init_config.path_to_config_file)?;
        Ok(())
    }

    pub async fn send_crypto(&mut self, nickname_from: Nickname) -> Result<(), Error> {
        if self.raw_client.data.nickname == *nickname_from {
            return Err(Error::NicknameSame(nickname_from));
        }
        if self.config.order_adding_crypto.contains_key(&nickname_from) {
            return Err(Error::NicknameSame(nickname_from));
        }

        let secret = self.raw_client.send_aes_key(&nickname_from).await?;
        //self.config.order_adding_crypto.insert(nickname_from, ); // TODO

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::get_rand_string;
    use std::path::PathBuf;
    use temp_dir::TempDir;

    pub const ADDRESS_SERVER: &str = "http://[::1]:2052";

    struct PathsForTest {
        _temp_dir: TempDir, // for lifetime
        path_to_config_file: PathBuf,
    }

    impl PathsForTest {
        fn get() -> Self {
            let temp_dir = TempDir::new().unwrap();

            Self {
                path_to_config_file: temp_dir.child("config.bin"),
                _temp_dir: temp_dir,
            }
        }
    }

    #[tokio::test]
    async fn save_and_load() {
        let paths = PathsForTest::get();
        let client_config = ClientInitConfig::new(paths.path_to_config_file, ADDRESS_SERVER);
        let client = Client::registration(&get_rand_string(), client_config.clone())
            .await
            .unwrap();

        client.save().unwrap();

        let loaded_client = Client::load(client_config).await.unwrap();
        println!("loaded_client data: {:#?}", loaded_client.raw_client.data);
        println!("client data: {:#?}", client.raw_client.data);
        assert_eq!(
            loaded_client.raw_client.data.nickname,
            client.raw_client.data.nickname
        )
    }
}
