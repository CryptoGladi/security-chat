//! Module for implementing of config saving and loading

use self::client_init_config::ClientInitConfig;
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

#[derive(Default, Clone, Debug)]
pub struct ClientConfig {
    pub client_data: DataForAutification,
    pub storage_crypto: Arc<RwLock<StorageCrypto>>,
    pub order_adding_crypto: HashMap<NicknameFrom, Secret>,
}

impl Client {
    pub async fn load_config(init_config: ClientInitConfig) -> Result<Client, Error> {
        debug!("run `load_config`");

        let bincode_config = BincodeConfig::new(init_config.path_to_config_file.clone());
        let config: ClientConfig = config_simple_load(&bincode_config)?;
        let api = LowerLevelClient::grpc_connect(init_config.address_to_server.clone()).await?;

        if !LowerLevelClient::check_account_valid(
            &config.client_data.nickname,
            &config.client_data.auth_key,
            init_config.address_to_server.clone(),
        )
        .await?
        {
            return Err(Error::AccoutIsInvalid);
        }

        let cache = client_init_config::get_cache(&init_config).await.unwrap();

        Ok(Self {
            lower_level_client: LowerLevelClient {
                grpc: api,
                data_for_autification: config.client_data.clone(),
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
}
