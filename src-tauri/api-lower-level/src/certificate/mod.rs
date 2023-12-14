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
pub mod getter;

#[derive(Debug, PartialEq, Default)]
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

    const TEST_LINK: &str =
        "https://raw.githubusercontent.com/CryptoGladi/certificates/master/MyCertificate.crt";
    const TEST_HASH: &str = "ceb4a38626e8aecf316824ff5a83fc02458b33b6d024d30882b1a5e025b51337783e68fcbcfc0c2378dcf7a86413b3411e3c90398bcfe73c4a63c50dad3fa3cc";

    #[test]
    fn download() {
        let temp_dir = TempDir::new().unwrap();
        let folder = temp_dir.child("temp");
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
