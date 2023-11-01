use super::storage_crypto::StorageCrypto;
use hashbrown::HashMap;
use lower_level::client::crypto::ecdh::EphemeralSecretDef;
use lower_level::client::ClientData;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct ClientInitConfig {
    pub path_to_config_file: PathBuf,
    pub path_to_cache: PathBuf,
    pub address_to_server: http::Uri,
}

impl ClientInitConfig {
    pub fn new(
        path_to_config_file: impl AsRef<Path>,
        path_to_cache: impl AsRef<Path>,
        address_to_server: impl TryInto<http::Uri>,
    ) -> Self {
        let Ok(address_to_server) = address_to_server.try_into() else {
            panic!("address_to_server.try_into() error");
        };

        Self {
            path_to_config_file: path_to_config_file.as_ref().into(),
            path_to_cache: path_to_cache.as_ref().into(),
            address_to_server,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ClientConfigData {
    pub client_data: ClientData,
    pub storage_crypto: StorageCrypto,

    /// nickname_from - secter_to
    pub order_adding_crypto: HashMap<String, EphemeralSecretDef>,
}

impl ClientConfigData {
    pub fn as_normal(&self) -> ClientConfig {
        ClientConfig {
            client_data: self.client_data.clone(),
            storage_crypto: Arc::new(RwLock::new(self.storage_crypto.clone())),
            order_adding_crypto: self.order_adding_crypto.clone(),
        }
    }
}

impl Debug for ClientConfigData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ClientConfig")
            .field(&self.client_data)
            .field(&self.storage_crypto)
            .finish()
    }
}

#[derive(Default, Clone, Debug)]
pub struct ClientConfig {
    pub client_data: ClientData,
    pub storage_crypto: Arc<RwLock<StorageCrypto>>,

    /// nickname_from - secter_to
    pub order_adding_crypto: HashMap<String, EphemeralSecretDef>,
}

impl ClientConfig {
    pub fn as_data(&self) -> ClientConfigData {
        ClientConfigData {
            client_data: self.client_data.clone(),
            storage_crypto: self.storage_crypto.read().unwrap().clone(),
            order_adding_crypto: self.order_adding_crypto.clone(),
        }
    }
}
