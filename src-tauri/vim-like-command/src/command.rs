pub mod add_crypto;

use high_level::prelude::*;
use std::{error::Error, fmt::Debug};

pub trait Command<E>: Debug
where
    E: Error,
{
    fn get_id(&self) -> &'static str;

    fn run(&mut self, client: &Client, command: &str) -> Result<(), E>;
}

pub type HighLevelCommand = dyn Command<ClientError>;
