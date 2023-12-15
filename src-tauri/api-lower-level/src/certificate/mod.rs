//! This module is responsible for downloading the certificate

use connection_parameters::ConnectionParameters;
use error::Error;
use log::{debug, trace};
use reqwest::Client;
use sha2::{Digest, Sha512};

pub mod connection_parameters;
pub mod error;
pub mod getter;

#[derive(Debug, PartialEq, Default)]
pub struct Certificate {
    /// URL link to file
    pub link: String,

    /// [SHA-512](https://emn178.github.io/online-tools/sha512.html) hash
    pub valid_hash: String,
    pub connection_parameters: ConnectionParameters,
}

impl Certificate {
    #[must_use]
    pub fn new(link: String, hash: String, connection_parameters: ConnectionParameters) -> Self {
        Self {
            link,
            valid_hash: hash,
            connection_parameters,
        }
    }

    fn check_valid(certificate: &str, valid_hash: &str) -> bool {
        let mut hash = Sha512::new();
        hash.update(certificate.as_bytes());

        valid_hash == format!("{:x}", hash.finalize())
    }

    /// Download
    #[allow(clippy::unwrap_in_result)]
    pub async fn download(&self) -> Result<String, Error> {
        debug!("run `download` certificate");

        let client = Client::builder()
            .timeout(self.connection_parameters.timeout)
            .connect_timeout(self.connection_parameters.timeout)
            .build()?;

        trace!("run downloading certificate...");
        let certificate = client.get(&self.link).send().await?.text().await?;
        trace!("done downloading certificate!");

        if !Certificate::check_valid(&certificate, &self.valid_hash) {
            return Err(Error::InvalidHash);
        }

        Ok(certificate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_LINK: &str =
        "https://raw.githubusercontent.com/CryptoGladi/certificates/master/MyCertificate.crt";
    const TEST_HASH: &str = "ceb4a38626e8aecf316824ff5a83fc02458b33b6d024d30882b1a5e025b51337783e68fcbcfc0c2378dcf7a86413b3411e3c90398bcfe73c4a63c50dad3fa3cc";

    #[tokio::test]
    async fn download() {
        let certificate = Certificate::new(
            TEST_LINK.to_string(),
            TEST_HASH.to_string(),
            ConnectionParameters::default(),
        );

        certificate.download().await.unwrap();
    }
}
