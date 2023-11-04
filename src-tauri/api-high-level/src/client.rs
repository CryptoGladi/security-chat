//! Main module

use self::notification::Notification;
use api_lower_level::client::{
    impl_crypto::{
        aes::Aes,
        ecdh::{get_shared_secret, EphemeralSecretDef, PublicKey},
    },
    Client as LowerLevelClient,
};
use cache::prelude::*;
use error::Error;
use fcore::prelude::BincodeConfig;
use impl_config::client_init_config::ClientInitConfig;
use impl_config::ClientConfig;
use kanal::AsyncReceiver;
use log::*;

pub mod error;
pub mod impl_config;
pub mod impl_crypto;
pub mod impl_message;
pub mod notification;
pub mod storage_crypto;

#[derive(Debug)]
pub struct Client {
    lower_level_client: LowerLevelClient,
    config: ClientConfig,
    bincode_config: BincodeConfig<ClientConfig>,
    _cache: Option<CacheSQLite>, // TODO
}

impl Client {
    pub async fn registration(
        nickname: &str,
        init_config: ClientInitConfig,
    ) -> Result<Client, Error> {
        debug!("run registration...");

        let raw_client =
            LowerLevelClient::registration(nickname, init_config.address_to_server.clone()).await?;

        let cache = impl_config::client_init_config::get_cache(&init_config)
            .await
            .unwrap();

        info!(
            "new registration: {}",
            raw_client.data_for_autification.nickname
        );

        Ok(Self {
            config: ClientConfig {
                client_data: raw_client.data_for_autification.clone(),
                ..Default::default()
            },
            _cache: cache,
            lower_level_client: raw_client,
            bincode_config: BincodeConfig::new(init_config.path_to_config_file),
        })
    }

    pub fn get_all_users(&self) -> Result<Vec<String>, Error> {
        let storage_crypto = self.config.storage_crypto.read().unwrap();
        Ok(storage_crypto.0.keys().cloned().collect())
    }

    pub fn have_account(init_config: &ClientInitConfig) -> Result<bool, Error> {
        Ok(init_config.path_to_config_file.is_file())
    }

    pub fn get_nickname(&self) -> String {
        self.config.client_data.nickname.clone()
    }

    pub async fn subscribe(&mut self) -> Result<AsyncReceiver<Notification>, Error> {
        debug!("run subscribe");
        let mut subscribe = self.lower_level_client.subscribe_to_notifications().await?;
        let (send, recv) = kanal::unbounded_async();
        let storage_crypto = self.config.storage_crypto.clone();

        tokio::spawn(async move {
            loop {
                let notify = subscribe.get_mut().message().await.unwrap().unwrap();
                let notify = Client::nofity(&storage_crypto.read().unwrap(), notify).unwrap();
                debug!("new notify: {:?}", notify);

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
        debug!("run nickname_is_taken");

        Ok(api_lower_level::client::Client::nickname_is_taken(
            nickname,
            init_config.address_to_server.clone(),
        )
        .await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::get_client;
    use fcore::test_utils::get_rand_string;
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
            !Client::nickname_is_taken(&client_config, &get_rand_string(20))
                .await
                .unwrap()
        );
    }

    #[test(tokio::test)]
    async fn have_account() {
        let (_paths, client_config, client) = get_client().await;

        client.save_config().unwrap();
        assert!(Client::have_account(&client_config).unwrap());
    }

    #[test(tokio::test)]
    async fn get_nickname() {
        let (_paths, _, client) = get_client().await;

        assert_eq!(
            client.get_nickname(),
            client.lower_level_client.data_for_autification.nickname
        );
    }
}
