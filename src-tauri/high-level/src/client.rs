use crate::config::Config;
use error::Error;
use lower_level::client::Client as RawClient;
use lower_level::client::ClientData;
use user::Usernames;
use client_config::ClientConfig;

pub mod error;
pub mod user;
pub mod client_config;

pub struct Client<'a> {
    raw_client: RawClient,
    config: Config<'a, ClientData>,
    usernames: Config<'a, Usernames>,
}

impl<'a> Client<'a> {
    pub async fn registration(nickname: &str, config: ClientConfig) -> Result<Client<'a>, Error> {
        Ok(Self {
            raw_client: RawClient::registration(nickname, config.address_to_server).await?,
            config: Config::new(config.path_to_config_file),
            usernames: Config::new(config.path_to_usernames),
        })
    }

    pub async fn load(config: ClientConfig) -> Result<Client<'a>, Error> {
        let data = Config::<ClientData>::new(config.path_to_config_file.clone()).load()?;
        let api = RawClient::api_connect(config.address_to_server.clone()).await?;

        if !*RawClient::check_valid(&data.nickname, &data.auth_key, config.address_to_server)
            .await?
        {
            return Err(Error::AccoutIsInvalid);
        }

        Ok(Self {
            raw_client: RawClient { api, data },
            config: Config::new(config.path_to_config_file),
            usernames: Config::new(config.path_to_usernames),
        })
    }

    pub fn save(&self) -> Result<(), Error> {
        Ok(self.config.save(&self.raw_client.data)?)
    }

    pub async fn send_crypto() {
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn lll() {
        let client = Client::registration("llll", ClientConfig::new("sss", "ss", "ss"))
            .await
            .unwrap();
    }
}
