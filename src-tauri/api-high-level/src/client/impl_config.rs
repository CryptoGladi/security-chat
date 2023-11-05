//! Module for implementing of config saving and loading

use self::client_init_config::ClientInitArgs;
use super::storage_crypto::StorageCrypto;
use super::Error;
use super::{Client, LowerLevelClient};
use api_lower_level::client::impl_crypto::ecdh::EphemeralSecretDef;
use api_lower_level::client::DataForAutification;
use fcore::prelude::BincodeConfig;
use fcore::prelude::{config_simple_load, config_simple_save};
use hashbrown::HashMap;
use log::debug;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

pub mod client_init_config;
pub mod impl_serde;

pub type NicknameFrom = String;
pub type Secret = EphemeralSecretDef;

/// All data for client
///
/// # Warning
///
/// After **changing** the fields, please change [this](crate::client::impl_config::impl_serde)
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
    pub async fn load_config(init_args: ClientInitArgs) -> Result<Client, Error> {
        debug!("run `load_config`");

        let bincode_config = BincodeConfig::new(init_args.path_to_config_file.clone());
        let config: ClientConfig = config_simple_load(&bincode_config)?;
        let api = LowerLevelClient::grpc_connect(init_args.address_to_server.clone()).await?;

        #[cfg(debug_assertions)]
        {
            if !LowerLevelClient::check_account_valid(
                &config.data_for_autification.nickname,
                &config.data_for_autification.auth_key,
                init_args.address_to_server.clone(),
            )
            .await?
            {
                return Err(Error::AccoutIsInvalid);
            }
        }

        let cache = init_args.get_cache().await.unwrap();

        Ok(Self {
            lower_level_client: LowerLevelClient {
                grpc: api,
                data_for_autification: config.data_for_autification.clone(),
            },
            _cache: cache,
            config,
            bincode_config,
        })
    }

    pub fn save_config(&self) -> Result<(), Error> {
        debug!("run save_config");

        config_simple_save(&self.bincode_config, &self.config).unwrap();
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

        let loaded_client = Client::load_config(client_config).await.unwrap();

        println!(
            "loaded_client data: {:#?}",
            loaded_client.lower_level_client.data_for_autification
        );
        println!("client data: {:#?}", client_data);

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

        let _loaded_client = Client::load_config(client_config).await.unwrap();
    }

    #[test]
    fn impl_partial_eq() {
        let mut client_config = ClientConfig {
            order_adding_crypto: HashMap::default(),
            data_for_autification: DataForAutification {
                nickname: "test_nickname".to_string(),
                auth_key: "SUPER_KEY".to_string(),
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
