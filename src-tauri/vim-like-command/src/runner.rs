use crate::command::HighLevelCommand;
use error::VimError;
use high_level::prelude::Client;
use log::*;
use std::collections::HashMap;
use std::fmt::Debug;

pub mod builder;
pub mod error;
pub mod impl_fuzzy_search;

type NameCommand<'a> = &'a str;

/// Struct for run commands
///
/// For initialization use [`crate::runner::builder::RunnerBuilder`]
#[derive(Debug, Default)]
pub struct Runner<'a> {
    /// All commands
    pub(crate) commands: HashMap<NameCommand<'a>, &'a HighLevelCommand>,

    /// Maximum number of items that will be displayed during a [fuzzy search](https://en.wikipedia.org/wiki/Approximate_string_matching)
    pub(crate) limit_for_fuzzy_search: usize,
}

impl<'a> Runner<'a> {
    /// Run command
    ///
    /// # Format
    ///
    /// {0} {1..2..3}
    ///
    /// * {0} - id command
    /// * {1..2..3} - args command
    ///
    /// # Example
    ///
    /// `test_command` `test_argument`
    ///
    /// `send_crypto` `nickname_my_friend`
    pub async fn run(&mut self, client: &mut Client, command: &str) -> VimError<()> {
        info!("run `command`: {}", command);
        let args: Vec<&str> = command.split_whitespace().collect();

        self.commands
            .get(&args[0])
            .ok_or(error::Error::NotFoundCommand)?
            .run(client, &args)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use test_log::test;

    #[test(tokio::test)]
    async fn run() {
        let (_path, _, mut client) = get_client().await;
        let mut runner = RunnerBuilder::new()
            .commands(vec![&TestCommand])
            .build()
            .unwrap();
        runner.run(&mut client, "test_command").await.unwrap();
    }

    #[test(tokio::test)]
    async fn not_found_command() {
        let (_path, _, mut client) = get_client().await;
        let mut runner = RunnerBuilder::new()
            .commands(vec![&TestCommand])
            .build()
            .unwrap();

        assert!(matches!(
            runner.run(&mut client, "not_command").await,
            Err(error::Error::NotFoundCommand)
        ));
    }
}
