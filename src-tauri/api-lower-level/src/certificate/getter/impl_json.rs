use super::Getter;
use crate::certificate::connection_parameters::ConnectionParameters;
use crate::certificate::Certificate;
use async_trait::async_trait;
use log::trace;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    /// URL link to file
    pub link: String,

    /// [SHA-512](https://emn178.github.io/online-tools/sha512.html) hash
    pub hash: String,
}

#[derive(Debug)]
pub struct GetterByJson {
    link: String,
}

impl GetterByJson {
    #[must_use]
    pub fn new(link: String) -> Self {
        Self { link }
    }
}

#[async_trait]
impl Getter for GetterByJson {
    async fn get(&self, connection_parameters: ConnectionParameters) -> Certificate {
        trace!(
            "run `get` with connection_parameters: `{:?}`",
            connection_parameters
        );

        let raw_json = reqwest::get(&self.link)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let json: Data = serde_json::from_str(&raw_json).unwrap();

        Certificate {
            link: json.link,
            valid_hash: json.hash,
            connection_parameters,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_dir::TempDir;

    const TEST_LINK: &str =
        "https://raw.githubusercontent.com/CryptoGladi/certificates/master/information.json";

    #[tokio::test]
    async fn impl_json() {
        let getter = GetterByJson::new(TEST_LINK.to_string());
        let temp_dir = TempDir::new().unwrap();
        let folder = temp_dir.child("temp");
        std::fs::create_dir_all(&folder).unwrap();

        let certificate = getter.get(ConnectionParameters::default()).await;
        certificate.download().await.unwrap();
    }
}
