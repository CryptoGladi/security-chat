//! Module for `ONLY` testing

pub use crate::command::Command;
use crate::command::CommandError;
pub use crate::runner::builder::RunnerBuilder;
pub use async_trait::async_trait;
use fcore::test_utils::*;
pub use high_level::prelude::*;

#[derive(Debug, Default)]
pub struct TestCommand;

#[async_trait]
impl Command<CommandError> for TestCommand {
    fn get_id(&self) -> &'static str {
        "test_command"
    }

    async fn run(&self, _client: &mut Client, _command: &[&str]) -> Result<(), CommandError> {
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct SameTestCommand;

#[async_trait]
impl Command<CommandError> for SameTestCommand {
    fn get_id(&self) -> &'static str {
        "same_test_command"
    }

    async fn run(&self, _client: &mut Client, _command: &[&str]) -> Result<(), CommandError> {
        Ok(())
    }
}

/// Get client for unit test
///
/// This function does:
/// 1. Creating temporary folders
/// 2. Register an account
/// 3. Returning the ready client for testing
pub async fn get_client() -> (PathsForTest, ClientInitConfig, Client) {
    let paths = PathsForTest::get();
    let client_config = ClientInitConfig::new(
        paths.path_to_config_file.clone(),
        paths.path_to_cache.clone(),
        ADDRESS_SERVER,
    );
    let client = Client::registration(&get_rand_string(20), client_config.clone())
        .await
        .unwrap();

    (paths, client_config, client)
}
