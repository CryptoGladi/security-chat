//! Implementation [bincode](https://github.com/bincode-org/bincode) for [`config interface`](crate::config::Config)
//!
//! # Warning
//!
//! If your file is **broken**, [bincode](https://github.com/bincode-org/bincode) will still read it.
//! So do a validation **every time**!

use super::{Config, Error};
use log::*;
use serde::{de::DeserializeOwned, Serialize};
use std::io::Write;
use std::marker::PhantomData;
use std::path::Path;
use std::{fs::OpenOptions, io::BufReader, path::PathBuf};

#[derive(Debug)]
pub struct BincodeConfig<Item>
where
    Item: Serialize + DeserializeOwned,
{
    path: PathBuf,
    phantom: PhantomData<Item>,
}

impl<Item> BincodeConfig<Item>
where
    Item: Serialize + DeserializeOwned,
{
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            phantom: PhantomData,
        }
    }
}

impl<Item> Config for BincodeConfig<Item>
where
    Item: Serialize + DeserializeOwned,
{
    type Item = Item;

    fn load(&self) -> Result<Self::Item, Error> {
        trace!("run `load` with path: {}", self.path.display());

        let file = OpenOptions::new()
            .read(true)
            .open(self.get_path_for_config())
            .map_err(Error::IO)?;

        let reader = BufReader::new(file);
        bincode::deserialize_from(reader).map_err(|x| Error::Other(x.into()))
    }

    fn save(&self, data: &Self::Item) -> Result<(), Error> {
        trace!("run `save` with path: {}", self.path.display());

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.get_path_for_config())
            .map_err(Error::IO)?;

        let bincode = bincode::serialize(&data).map_err(|x| Error::Other(x.into()))?;
        file.write_all(&bincode).map_err(Error::IO)?;
        file.flush().map_err(Error::IO)?;

        Ok(())
    }

    fn get_path_for_config(&self) -> PathBuf {
        self.path.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_dir::TempDir;

    fn create_bincode_config() -> (BincodeConfig<i32>, TempDir, PathBuf) {
        let temp = TempDir::new().unwrap();
        let file = temp.child("config.bin");
        let bincode_config = BincodeConfig::new(file.clone());

        (bincode_config, temp, file)
    }

    #[test]
    fn save_and_load() {
        let testing_data = 873;
        let (bincode_config, _temp, _) = create_bincode_config();

        bincode_config.save(&testing_data).unwrap();
        assert_eq!(bincode_config.load().unwrap(), testing_data);
    }

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn file_not_found() {
        let testing_data: i32 = 123;

        let bincode_config_without_file = BincodeConfig::new("file_not_found.txt");

        let data: i32 = bincode_config_without_file.load().unwrap();
        assert_eq!(data, testing_data);
    }

    #[test]
    #[allow(unused_variables)]
    fn break_file() {
        let testing_data: i32 = 404;
        let (bincode_config, _temp, path_to_file) = create_bincode_config();

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path_to_file)
            .unwrap();

        file.write_all(b"BREAK DATA").unwrap();
        drop(file);

        // assert_eq!(bincode_config.load().unwrap(), testing_data);
        // There should be no panic because of the bincode spec.
        // Therefore, you should keep an eye on the integrity of your files!
    }

    #[test]
    fn get_path_for_config() {
        let (bincode_config, _temp, path_to_file) = create_bincode_config();

        assert_eq!(bincode_config.get_path_for_config(), path_to_file);
    }
}
