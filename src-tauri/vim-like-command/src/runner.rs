use crate::command::HighLevelCommand;
use error::VimError;
use std::collections::HashMap;
use std::fmt::Debug;
use log::*;

pub mod builder;
pub mod error;
pub mod impl_fuzzy_search;

#[derive(Debug, Default)]
pub struct Runner<'a> {
    pub(crate) commands: HashMap<&'a str, &'a HighLevelCommand>,
    pub(crate) limit_fuzzy: usize,
}

unsafe impl<'a> Sync for Runner<'a> {}

impl<'a> Runner<'a> {
    pub async fn run(&mut self, str: &str) -> VimError<()> {
        info!("run `command`: {}", str);
        let args: Vec<&str> = str.split_whitespace().collect();

        self.commands
            .get(&args[0])
            .ok_or(error::Error::NotFoundCommand)?
            .get_id(); // TODO

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
        let mut runner = RunnerBuilder::new()
            .commands(vec![&TestCommand])
            .build()
            .unwrap();
        runner.run("test_command").await.unwrap();
    }

    #[test(tokio::test)]
    async fn not_found_command() {
        let mut runner = RunnerBuilder::new()
            .commands(vec![&TestCommand])
            .build()
            .unwrap();

        assert!(matches!(
            runner.run("not_command").await,
            Err(error::Error::NotFoundCommand)
        ));
    }
}
