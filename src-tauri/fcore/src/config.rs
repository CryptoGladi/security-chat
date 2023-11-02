//! Module for saving and loading data

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
