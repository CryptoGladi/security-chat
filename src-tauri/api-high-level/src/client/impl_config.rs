//! Module for implementing of config saving and loading

use self::client_init_config::ClientInitArgs;
use super::storage_crypto::StorageCrypto;
use super::Error;
use super::{Client, LowerLevelClient};
use api_lower_level::client::DataForAutification;
use crate_unsafe::safe_impl::crypto::ephemeral_secret_def::UnsafeEphemeralSecretDef;
use fcore::prelude::BincodeConfig;
use fcore::prelude::{simple_load, simple_save};
use hashbrown::HashMap;
use log::debug;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

pub mod client_init_config;
pub mod impl_serde;

pub type NicknameFrom = String;
pub type Secret = UnsafeEphemeralSecretDef;

/// All data for client
///
/// # Warning
///
/// After **changing** the fields, please change [this](impl_serde)
#[derive(Default, Clone, Debug)]
pub struct ClientConfig {
    pub data_for_autification: DataForAutification,
    pub storage_crypto: Arc<RwLock<StorageCrypto>>,
    pub order_adding_crypto: HashMap<NicknameFrom, Secret>,
}

impl PartialEq for ClientConfig {
    fn eq(&self, other: &Self) -> bool {
        self.data_for_autification == other.data_for_autification
            && *self.storage_crypto.read().unwrap() == *other.storage_crypto.read().unwrap()
            && self.order_adding_crypto == other.order_adding_crypto
    }
}

impl Client {
    pub async fn login_by_config(init_args: ClientInitArgs) -> Result<Self, Error> {
        debug!("run `load_config`");

        let bincode_config = BincodeConfig::new(init_args.path_to_config_file.clone());
        let config: ClientConfig = simple_load(&bincode_config)?;
        let lower_level_client = LowerLevelClient::login(
            init_args.address_to_server.clone(),
            config.data_for_autification.nickname.clone(),
            config.data_for_autification.refresh_token.clone(),
        )
        .await?;

        let cache = init_args.get_cache().await?;

        Ok(Self {
            lower_level_client,
            _cache: cache,
            config,
            bincode_config,
        })
    }

    pub fn save_config(&self) -> Result<(), Error> {
        debug!("run save_config");

        simple_save(&self.bincode_config, &self.config)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::get_client;
    use test_log::test;

    #[test(tokio::test)]
    async fn save_and_load() {
        let (_paths, client_config, client) = get_client().await;

        client.save_config().unwrap();
        let client_data = client.lower_level_client.data_for_autification.clone();
        drop(client); // for cache

        let loaded_client = Client::login_by_config(client_config).await.unwrap();

        log::info!(
            "loaded_client data: {:#?}",
            loaded_client.lower_level_client.data_for_autification
        );
        log::info!("client data: {:#?}", client_data);

        assert_eq!(
            loaded_client
                .lower_level_client
                .data_for_autification
                .nickname,
            client_data.nickname
        );
    }

    #[test(tokio::test)]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: Config(IO(Os { code: 2, kind: NotFound, message: \"No such file or directory\" }))"
    )]
    async fn not_found_file() {
        let (_, client_config, _) = get_client().await;

        let _loaded_client = Client::login_by_config(client_config).await.unwrap();
    }

    #[test]
    fn impl_partial_eq() {
        let mut client_config = ClientConfig {
            order_adding_crypto: HashMap::default(),
            data_for_autification: DataForAutification {
                nickname: "test_nickname".to_string(),
                refresh_token: "SUPER VALUE".to_string(),
            },
            storage_crypto: Arc::new(RwLock::new(StorageCrypto::default())),
        };

        assert_eq!(client_config, client_config);

        let client_config1 = client_config.clone();
        // *client_config1.storage_crypto.write().unwrap() = StorageCrypto::default(); std::arc::ARC!
        client_config.data_for_autification.nickname = "new_nickname".to_string();

        assert_ne!(client_config, client_config1);
    }
}