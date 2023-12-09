//! This module is responsible for downloading the certificate

use connection_parameters::ConnectionParameters;
use downloader::{Download, Downloader, Verification};
use error::Error;
use log::trace;
use sha2::Sha512;
use std::path::PathBuf;
use std::sync::Arc;
use temp_dir::TempDir;

pub mod connection_parameters;
pub mod error;

#[derive(Debug)]
pub struct Certificate {
    /// URL link to file
    pub link: String,

    /// [SHA-512](https://emn178.github.io/online-tools/sha512.html) hash
    pub hash: String,
    pub connection_parameters: ConnectionParameters,
}

impl Certificate {
    #[must_use]
    pub fn new(link: String, hash: String, connection_parameters: ConnectionParameters) -> Self {
        Self {
            link,
            hash,
            connection_parameters,
        }
    }

    /// Download
    ///
    /// # Panics
    ///
    /// Occurs when there is an error in accessing a file,
    /// but it is theoretically impossible because we
    /// are using [temporary directories](temp_dir)
    #[allow(clippy::unreachable)]
    pub fn download(&self) -> Result<PathBuf, Error> {
        trace!("run `download` certificate");
        let temp = TempDir::new().map_err(Error::TempDir)?;

        let valid_hash = self.hash.clone();
        let download = Download::new(&self.link).verify(Arc::new(move |path, _simple_progress| {
            let mut sha512 = Sha512::default();
            let hash_for_valid = file_hashing::get_hash_file(path, &mut sha512).unwrap();

            if valid_hash == hash_for_valid {
                Verification::Ok
            } else {
                Verification::Failed
            }
        }));

        let mut downloader = Downloader::builder()
            .connect_timeout(self.connection_parameters.timeout)
            .timeout(self.connection_parameters.timeout)
            .download_folder(temp.path())
            .build()?;

        let result = downloader.download(&[download])?;
        debug_assert_eq!(result.len(), 1, "we only need to download ONE file");

        if let Some(for_check_error) = result.into_iter().next() {
            return Ok(for_check_error?.file_name);
        }

        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn download() {
        let certificate = Certificate::new(
            "e".to_string(),
            "g".to_string(),
            ConnectionParameters::default(),
        );

        certificate.download().unwrap();
    }
}
