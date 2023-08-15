use crate::command::HighLevelCommand;
use error::VimError;
use high_level::prelude::Client;
use log::*;
use std::collections::HashMap;
use std::fmt::Debug;

pub mod builder;
pub mod error;
pub mod impl_fuzzy_search;

#[derive(Debug, Default)]
pub struct Runner<'a> {
    pub(crate) commands: HashMap<&'a str, &'a HighLevelCommand>,
    pub(crate) limit_fuzzy: usize,
}

impl<'a> Runner<'a> {
    pub async fn run(&mut self, client: &mut Client, str: &str) -> VimError<()> {
        info!("run `command`: {}", str);
        let args: Vec<&str> = str.split_whitespace().collect();

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
