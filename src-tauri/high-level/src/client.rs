use self::notification::Notification;
use crate::bincode_config;
use client_config::{ClientConfig, ClientConfigData, ClientInitConfig};
use error::Error;
use kanal::AsyncReceiver;
use log::*;
use lower_level::client::{
    crypto::{
        ecdh::{get_shared_secret, EphemeralSecretDef, PublicKey},
        Aes,
    },
    Client as RawClient,
};
use storage_crypto::Nickname;
use cache::prelude::*;

pub mod client_config;
pub mod error;
pub mod impl_crypto;
pub mod impl_message;
pub mod notification;
pub mod storage_crypto;

#[derive(Debug)]
pub struct Client {
    raw_client: RawClient,
    config: ClientConfig,
    cache: CacheSQLite,
    init_config: ClientInitConfig,
}

impl Client {
    pub async fn registration(
        nickname: &str,
        init_config: ClientInitConfig,
    ) -> Result<Client, Error> {
        debug!("run registration...");

        let raw_client =
            RawClient::registration(nickname, init_config.address_to_server.clone()).await?;
        let cache = Cache::new(init_config.path_to_cache.clone()).await?;

        warn!("new registration: {}", raw_client.data.nickname);

        Ok(Self {
            config: ClientConfigData {
                client_data: raw_client.data.clone(),
                ..Default::default()
            }
            .as_normal(),
            cache,
            init_config,
            raw_client,
        })
    }

    pub fn get_all_users(&self) -> Result<Vec<Nickname>, Error> {
        let storage_crypto = self.config.storage_crypto.read().unwrap();
        debug!("sfdssd: {:?}", storage_crypto);
        Ok(storage_crypto.0.keys().cloned().collect())
    }

    pub fn have_account(init_config: &ClientInitConfig) -> Result<bool, Error> {
        Ok(init_config.path_to_config_file.is_file())
    }

    pub async fn load(init_config: ClientInitConfig) -> Result<Client, Error> {
        info!("run load");
        let config: ClientConfigData =
            bincode_config::load(init_config.path_to_config_file.clone())?;
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

        let cache = Cache::new(init_config.path_to_cache.clone()).await?;

        Ok(Self {
            raw_client: RawClient {
                api,
                data: config.client_data.clone(),
            },
            cache,
            config: config.as_normal(),
            init_config,
        })
    }

    pub fn save(&self) -> Result<(), Error> {
        info!("run save");
        bincode_config::save(
            &self.config.as_data(),
            &self.init_config.path_to_config_file,
        )?;
        Ok(())
    }

    pub fn get_nickname(&self) -> Nickname {
        Nickname(self.config.client_data.nickname.clone())
    }

    pub async fn subscribe(&mut self) -> Result<AsyncReceiver<Notification>, Error> {
        info!("run subscribe");
        let mut subscribe = self.raw_client.subscribe().await?;
        let (send, recv) = kanal::unbounded_async();
        let storage_crypto = self.config.storage_crypto.clone();

        tokio::spawn(async move {
            loop {
                let notify = subscribe.get_mut().message().await.unwrap().unwrap();
                let notify = Client::nofity(&storage_crypto.read().unwrap(), notify).unwrap();
                info!("new notify: {:?}", notify);

                if send.send(notify).await.is_err() {
                    break;
                }
            }
        });

        Ok(recv)
    }

    pub async fn nickname_is_taken(
        init_config: &ClientInitConfig,
        nickname: &str,
    ) -> Result<bool, Error> {
        info!("run nickname_is_taken");

        Ok(
            lower_level::client::nickname_is_taken(nickname, init_config.address_to_server.clone())
                .await?,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{get_client, get_rand_string};
    use test_log::test;

    #[test(tokio::test)]
    async fn nickname_is_taken() {
        let (_paths, client_config, client) = get_client().await;

        assert!(
            Client::nickname_is_taken(&client_config, client.get_nickname().as_str())
                .await
                .unwrap()
        );
        assert!(
            !Client::nickname_is_taken(&client_config, &get_rand_string())
                .await
                .unwrap()
        );
    }

    #[test(tokio::test)]
    async fn save_and_load() {
        let (_paths, client_config, client) = get_client().await;

        client.save().unwrap();
        let client_data = client.raw_client.data.clone();
        drop(client); // for cache

        let loaded_client = Client::load(client_config).await.unwrap();
        println!("loaded_client data: {:#?}", loaded_client.raw_client.data);
        println!("client data: {:#?}", client_data);
        assert_eq!(loaded_client.raw_client.data.nickname, client_data.nickname)
    }

    #[test(tokio::test)]
    async fn have_account() {
        let (_paths, client_config, client) = get_client().await;

        client.save().unwrap();
        assert!(Client::have_account(&client_config).unwrap());
    }

    #[test(tokio::test)]
    async fn get_nickname() {
        let (_paths, _, client) = get_client().await;

        assert_eq!(
            client.get_nickname(),
            Nickname::from(client.raw_client.data.nickname)
        )
    }
}
