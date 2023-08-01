use crate::json_config::JsonConfig;
use error::Error;
use lower_level::client::Client as RawClient;
use lower_level::client::ClientData;
use user::{Nicknames, Nickname};
use client_config::ClientInitConfig;

pub mod error;
pub mod user;
pub mod client_config;
pub mod order_adding_nicknames;

pub struct Client<'a> {
    raw_client: RawClient,
    config: JsonConfig<'a, ClientData>,
    nicknames: JsonConfig<'a, Nicknames>,
}

impl<'a> Client<'a> {
    pub async fn registration(nickname: &str, config: ClientInitConfig) -> Result<Client<'a>, Error> {
        Ok(Self {
            raw_client: RawClient::registration(nickname, config.address_to_server).await?,
            config: JsonConfig::new(config.path_to_config_file),
            nicknames: JsonConfig::new(config.path_to_usernames),
        })
    }

    pub async fn load(config: ClientInitConfig) -> Result<Client<'a>, Error> {
        let data = JsonConfig::<ClientData>::new(config.path_to_config_file.clone()).load()?;
        let api = RawClient::api_connect(config.address_to_server.clone()).await?;

        if !*RawClient::check_valid(&data.nickname, &data.auth_key, config.address_to_server)
            .await?
        {
            return Err(Error::AccoutIsInvalid);
        }

        Ok(Self {
            raw_client: RawClient { api, data },
            config: JsonConfig::new(config.path_to_config_file),
            nicknames: JsonConfig::new(config.path_to_usernames),
        })
    }

    pub fn save(&self) -> Result<(), Error> {
        Ok(self.config.save(&self.raw_client.data)?)
    }

    pub async fn send_crypto(&mut self, nickname_from: Nickname) -> Result<(), Error> {
        if self.raw_client.data.nickname == *nickname_from {
            return Err(Error::NicknameSame)
        }

        let secret = self.raw_client.send_aes_key(&nickname_from).await?;

        // TODO
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;
    use crate::test_utils::get_rand_string;
    use temp_dir::TempDir;

    pub const ADDRESS_SERVER: &str = "http://[::1]:2052";

    struct PathsForTest {
        _temp_dir: TempDir, // for lifetime
        path_to_config_file: PathBuf,
        path_to_usernames: PathBuf,
    }

    impl PathsForTest {
        fn get() -> Self {
            let temp_dir = TempDir::new().unwrap();

            Self {
                path_to_config_file: temp_dir.child("config.json"),
                path_to_usernames: temp_dir.child("usernames.json"),
                _temp_dir: temp_dir,
            }
        }
    }

    #[tokio::test]
    async fn save_and_load() {
        let paths = PathsForTest::get();
        let client_config = ClientInitConfig::new(paths.path_to_config_file, paths.path_to_usernames, ADDRESS_SERVER);
        let client = Client::registration(&get_rand_string(), client_config.clone())
            .await
            .unwrap();

        client.save().unwrap();

        let loaded_client = Client::load(client_config).await.unwrap();
        println!("loaded_client data: {:#?}", loaded_client.raw_client.data);
        println!("client data: {:#?}", client.raw_client.data);
        assert_eq!(loaded_client.raw_client.data.nickname, client.raw_client.data.nickname)
    }
}
