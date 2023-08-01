use hashbrown::HashMap;
use lower_level::client::ClientData;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use lower_level::client::crypto::ecdh::EphemeralSecretDef;
use super::storage_crypto::{StorageCrypto, Nickname};

#[derive(Debug, Clone)]
pub struct ClientInitConfig {
    pub path_to_config_file: PathBuf,
    pub address_to_server: http::Uri,
}

impl ClientInitConfig {
    pub fn new(
        path_to_config_file: impl AsRef<Path>,
        address_to_server: impl TryInto<http::Uri>,
    ) -> Self {
        let Ok(address_to_server) = address_to_server.try_into() else {
            panic!("address_to_server.try_into() error");
        };

        Self {
            path_to_config_file: path_to_config_file.as_ref().to_path_buf(),
            address_to_server,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddCryptoInfo {
    db_id: i64,
    secret: EphemeralSecretDef
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ClientConfig {
    pub client_data: ClientData,
    pub storage_crypto: StorageCrypto,
    pub order_adding_crypto: HashMap<Nickname, AddCryptoInfo>
}
