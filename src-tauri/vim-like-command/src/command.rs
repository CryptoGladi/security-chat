pub mod send_crypto;

pub const ALL_COMMANDS: &[&HighLevelCommand] = &[&send_crypto::SendCrypto];

use high_level::prelude::*;
use std::{error::Error, fmt::Debug};
use async_trait::async_trait;

#[async_trait]
pub trait Command<E>: Debug + Send + Sync
where
    E: Error
{
    fn get_id(&self) -> &'static str;

    async fn run(&self, client: &mut Client, args: &[&str]) -> Result<(), E>;
}

pub type HighLevelCommand = dyn Command<ClientError>;
