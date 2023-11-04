use super::storage_crypto::StorageCrypto;
use api_lower_level::client::impl_crypto::ecdh::EphemeralSecretDef;
use api_lower_level::client::DataForAutification;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use serde::ser::{Serializer, SerializeStruct};
use serde::de::{Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};

pub mod client_init_config;

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

#[derive(Default, Clone, Debug)]
pub struct ClientConfig {
    pub client_data: DataForAutification,
    pub storage_crypto: Arc<RwLock<StorageCrypto>>,

    /// nickname_from - secter_to
    pub order_adding_crypto: HashMap<String, EphemeralSecretDef>,
}

impl Serialize for ClientConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        let mut state = serializer.serialize_struct("ClientConfig", 3)?;

        state.serialize_field("client_data", &self.client_data)?;
        state.serialize_field("storage_crypto", &self.storage_crypto.read().unwrap().0);
        state.serialize_field("order_adding_crypto", &self.order_adding_crypto);

        state.end()
    }
}

impl<'de> Deserialize<'de> for ClientConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de> {
        deserializer.
    }
}