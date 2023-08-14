pub use crate::command::Command;
pub use crate::runner::builder::RunnerBuilder;
pub use high_level::prelude::*;
pub use async_trait::async_trait;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::path::PathBuf;
use temp_dir::TempDir;

pub const ADDRESS_SERVER: &str = "http://[::1]:2052";

pub fn get_rand_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect::<String>()
}

pub struct PathsForTest {
    _temp_dir: TempDir, // for lifetime
    path_to_config_file: PathBuf,
}

impl PathsForTest {
    fn get() -> Self {
        let temp_dir = TempDir::new().unwrap();

        Self {
            path_to_config_file: temp_dir.child("config.bin"),
            _temp_dir: temp_dir,
        }
    }
}

pub async fn get_client() -> (PathsForTest, ClientInitConfig, Client) {
    let paths = PathsForTest::get();
    let client_config = ClientInitConfig::new(paths.path_to_config_file.clone(), ADDRESS_SERVER);
    let client = Client::registration(&get_rand_string(), client_config.clone())
        .await
        .unwrap();

    (paths, client_config, client)
}

#[derive(Debug, Default)]
pub struct TestCommand;

#[async_trait]
impl Command<ClientError> for TestCommand {
    fn get_id(&self) -> &'static str {
        "test_command"
    }

    async fn run(
        &self,
        _client: &mut Client,
        _command: &[&str],
    ) -> Result<(), ClientError> {
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct SameTestCommand;

#[async_trait]
impl Command<ClientError> for SameTestCommand {
    fn get_id(&self) -> &'static str {
        "same_test_command"
    }

    async fn run(
        &self,
        _client: &mut Client,
        _command: &[&str],
    ) -> Result<(), ClientError> {
        Ok(())
    }
}
