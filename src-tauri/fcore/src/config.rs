//! Module for saving and loading data

use log::trace;
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Debug, path::PathBuf};
use thiserror::Error;

pub mod impl_bincode;

#[derive(Debug, Error)]
pub enum Error {
    #[error("config io problem: `{0}`")]
    IO(std::io::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Interface for config
pub trait Config {
    /// Item for saving and loading
    ///
    /// It's for the sake of type security.
    type Item: Serialize + DeserializeOwned;

    /// Save [`Config::Item`]
    ///
    /// The path for saving is **determined by implementation**!
    fn save(&self, data: &Self::Item) -> Result<(), Error>;

    /// Load [`Config::Item`]
    ///
    /// The path for loading is **determined by implementation**!
    fn load(&self) -> Result<Self::Item, Error>;

    /// Get path for loading and saving config
    fn get_path_for_config(&self) -> PathBuf;
}

/// Function for simple [`load`](crate::config::Config::load) data
pub fn simple_load<T>(impl_config: &T) -> Result<T::Item, Error>
where
    T: Config + Debug,
{
    trace!("run `simple_load` with impl_config: {:?}", impl_config);
    impl_config.load()
}

/// Function for simple [`save`](crate::config::Config::save) data
pub fn simple_save<T>(impl_config: &T, data: &T::Item) -> Result<(), Error>
where
    T: Config + Debug,
{
    trace!("run `simple_save` with impl_config: {:?}", impl_config);
    impl_config.save(data)
}

#[cfg(test)]
mod tests {
    use crate::prelude::BincodeConfig;
    use temp_dir::TempDir;

    #[test]
    fn config_simple_load() {
        let temp_dir = TempDir::new().unwrap();
        let temp_file = temp_dir.child("config.bin");

        super::simple_save(&BincodeConfig::new(temp_file.clone()), &1).unwrap();
        let value: i32 = super::simple_load(&BincodeConfig::new(temp_file)).unwrap();

        assert_eq!(value, 1);
    }

    #[test]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: IO(Os { code: 2, kind: NotFound, message: \"No such file or directory\" })"
    )]
    fn config_simple_load_with_error_file_not_found() {
        let value: i32 = super::simple_load(&BincodeConfig::new("not_file.txt")).unwrap();

        assert_eq!(value, 404);
    }

    #[test]
    fn config_simple_save() {
        let temp_dir = TempDir::new().unwrap();
        let temp_file = temp_dir.child("config.bin");

        super::simple_save(&BincodeConfig::new(temp_file), &1).unwrap();
    }
}
