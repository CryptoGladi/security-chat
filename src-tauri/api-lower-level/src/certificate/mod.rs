//! This module is responsible for downloading the certificate

use connection_parameters::ConnectionParameters;
use downloader::{Download, Downloader, Verification};
use error::Error;
use log::trace;
use sha2::Sha512;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub mod connection_parameters;
pub mod error;

#[derive(Debug)]
pub struct Certificate {
    /// URL link to file
    pub link: String,

    /// [SHA-512](https://emn178.github.io/online-tools/sha512.html) hash
    pub hash: String,
    pub path: PathBuf,
    pub connection_parameters: ConnectionParameters,
}

impl Certificate {
    #[must_use]
    pub fn new(
        link: String,
        hash: String,
        path: impl AsRef<Path>,
        connection_parameters: ConnectionParameters,
    ) -> Self {
        Self {
            link,
            hash,
            path: path.as_ref().to_path_buf(),
            connection_parameters,
        }
    }

    fn raw_check_valid(path: &PathBuf, valid_hash: &str) -> Result<bool, Error> {
        let mut sha512 = Sha512::default();
        let hash_for_valid = file_hashing::get_hash_file(path, &mut sha512)?;

        println!("332: {:?}", valid_hash == hash_for_valid);
        println!("valid_hash: {}", valid_hash);
        println!("hash_for_valid: {}", hash_for_valid);
        Ok(valid_hash == hash_for_valid)
    }

    /// Check valid
    pub fn check_valid(&self) -> Result<bool, Error> {
        Certificate::raw_check_valid(&self.path, &self.hash)
    }

    /// Download
    ///
    /// # Panics
    ///
    /// Occurs when there is an error in accessing a file,
    /// but it is theoretically impossible because we
    /// are using [temporary directories](temp_dir)
    #[allow(clippy::unwrap_in_result)]
    pub fn download(&self) -> Result<PathBuf, Error> {
        trace!("run `download` certificate");

        let valid_hash = self.hash.clone();
        let download = Download::new(&self.link).verify(Arc::new(move |path, _simple_progress| {
            println!("21");
            match Certificate::raw_check_valid(&path, &valid_hash) {
                Ok(is_valid) if is_valid => Verification::Ok,
                Ok(_) => Verification::Failed, // bool == false
                Err(_) => Verification::NotVerified,
            }
        }));

        let mut downloader = Downloader::builder()
            .connect_timeout(self.connection_parameters.timeout)
            .timeout(self.connection_parameters.timeout)
            .download_folder(&self.path)
            .build()?;

        let result = downloader.download(&[download])?;
        debug_assert_eq!(result.len(), 1, "we only need to download ONE file");

        Ok(result.into_iter().next().unwrap()?.file_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_dir::TempDir;

    const TEST_LINK: &str = "https://doc-14-58-docs.googleusercontent.com/docs/securesc/m72m5pht4gc4e7mr583n9bg50qbtfe5k/p3rrojp8k02u173llfo0cfhvji316s5u/1702317900000/03371813539766287582/03371813539766287582/1onV7LDhrYvcEWuhp_9BBKOzYV-3uD1Do?e=download&ax=AEqgLxmt_BmJLI2vJVAFj-wUAL2RKSQYzZ85oCD22_zCAukYiOiRaw8L5pghZmqW82fOEGz29K15Sz176YYHJTp66ucmjq1L9mW7ByTxdMFEVLF0QoQ5A64lKFvGAANvHJY0mcVIXNWcx587rAsrgEkanCMvlIXjCcL9BG0iFVz6PwigE0wIPHg5EOzaPiZ9SlgQj4bEiWZEg5Dnyj4kkaBmqs7c7x9nHNUtUOHLoMPeoiuBsB06O01ZhpYA37iGjjDF06z8QIvhW1MLcpKn7j2i_N3l0A3XvspdHPTyuIB-Cq-6-ieqwfSAiDzFIueYxuk89rV7OvCrrByeHQWMFSxu4VnkCZkHTcc3rKZS2iP-AyLQo8us5VyL2XjzIyLak8sUwJS9_Vh06BT6aO_UwliEEg_5_ZVx9nf46AQCbu-sf3zNjqK2SUGYNi3EqE1OAtoGqPplz7EuBcipDXUm8j78SqV33PCAq1VDxPThNdwoIRRf6kCm0shdY8Yz8Sq1fho2rTiSGm9GMjcfgsTKMG4m_Qs-fkaFOgIG0Bj4wKTVFOKCQ_s5C8ihgo291A2VRiwr6yYIgT3lPcGmHITmipFmAU_JOiGaIZP3oJeFQ4J-6qxymbdAN_GxtJ7fxGUlhBx4WP_vdKY5MxQSpoTx8or0HoUZo1Rm9VSAj165La4GlBm1qxm8baDPcp9asrAdJiJHwf9iRFvQV7q8HS-an4mJHj1GXB7zge2fGqZat2ZyvirQGzfufe67nqDMTo-muMD7GkIZ6lbQubGmTXDHo27Fl85OKg5QY46uFHBER3WElQQ1303QabJIlVIELgPcvmjy4zI7P9kRuFg81wq0VoPV1Tdwpj7bOjgT4NCKzWSDAoVYqw&uuid=9231e6a0-564a-4266-b980-ccd2aabd371b&authuser=0&nonce=3veumt6o6mq6e&user=03371813539766287582&hash=of1altfu4qf1j79jlbb9bnl4s9is17ue";
    const TEST_HASH: &str = "857cc0b96bb6bee09f4e03c01c8890585fb6a374d8aaadaceb18facb384602582c2bb9be778c10f5365ce6d6791399913ec278e387c8d2d7735c3f9f444018f3";

    #[test]
    fn download() {
        let temp_dir = TempDir::new().unwrap();
        let folder = temp_dir.child("folder_for_downloading");
        std::fs::create_dir_all(&folder).unwrap();

        let certificate = Certificate::new(
            TEST_LINK.to_string(),
            TEST_HASH.to_string(),
            &folder,
            ConnectionParameters::default(),
        );

        certificate.download().unwrap();
    }
}
