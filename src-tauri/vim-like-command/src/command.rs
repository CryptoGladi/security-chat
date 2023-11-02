//! Module for storage commands
//!
//! Please, add them to the [`ALL_COMMANDS`] variable after adding a new command

use async_trait::async_trait;
use high_level::prelude::*;
use std::{error::Error, fmt::Debug};
use thiserror::Error;

pub mod send_crypto;

pub const ALL_COMMANDS: &[&dyn Command] = &[&send_crypto::SendCrypto];

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("problem in API client")]
    API(#[from] ClientError),

    #[error("problem in command: {0}")]
    Other(&'static str),
}

/// Interface for commands
#[async_trait]
pub trait Command<ErrorType = CommandError>: Debug + Send + Sync
where
    ErrorType: Error,
{
    /// Return a unique identifier for your command
    fn get_id(&self) -> &'static str;

    /// Run command
    ///
    /// * `client` - сlient API
    /// * `args` - arguments when your command is called
    async fn run(&self, client: &mut Client, args: &[&str]) -> Result<(), ErrorType>;
}
