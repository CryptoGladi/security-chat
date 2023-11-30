//! Module for `ONLY` testing

pub use crate::command::Command;
use crate::command::Error;
pub use crate::runner::builder::RunnerBuilder;
pub use api_high_level::prelude::*;
pub use async_trait::async_trait;
use fcore::test_utils::*;

#[derive(Debug, Default)]
pub struct TestCommand;

#[async_trait]
impl Command<Error> for TestCommand {
    fn get_id(&self) -> &'static str {
        "test_command"
    }

    async fn run(&self, _client: &mut Client, _command: &[&str]) -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct SameTestCommand;

#[async_trait]
impl Command<Error> for SameTestCommand {
    fn get_id(&self) -> &'static str {
        "same_test_command"
    }

    async fn run(&self, _client: &mut Client, _command: &[&str]) -> Result<(), Error> {
        Ok(())
    }
}

/// Get client for unit test
///
/// This function does:
/// 1. Creating temporary folders
/// 2. Register an account
/// 3. Returning the ready client for testing
pub async fn get_client() -> (PathsForTest, ClientInitArgs, Client) {
    let paths = PathsForTest::get();

    let client_config =
        ClientInitArgs::new(paths.path_to_config_file.clone(), ADDRESS_SERVER, None).unwrap();

    let client = Client::registration(&get_rand_string(20), client_config.clone())
        .await
        .unwrap();

    (paths, client_config, client)
}
