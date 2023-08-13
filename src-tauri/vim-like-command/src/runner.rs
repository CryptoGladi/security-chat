use crate::command::add_crypto::AddCrypto;
use crate::command::HighLevelCommand;
use error::VimError;
use log::*;
use std::collections::HashMap;
use std::fmt::Debug;

pub mod error;

pub const DEFAULT_COMMANDS: &[&HighLevelCommand] = &[&AddCrypto];

#[derive(Debug)]
pub struct Runner<'a> {
    pub(crate) commands: HashMap<&'a str, &'a HighLevelCommand>,
}

impl<'a> Default for Runner<'a> {
    fn default() -> Self {
        Self::new(DEFAULT_COMMANDS).unwrap()
    }
}

impl<'a> Runner<'a> {
    pub fn new(commands: &'a [&'a HighLevelCommand]) -> VimError<Self> {
        info!("run `new`");
        let mut parsed_commands = HashMap::new();

        for i in commands.iter() {
            if parsed_commands.insert(i.get_id(), *i).is_some() {
                return Err(error::Error::IdenticalId);
            }
        }

        Ok(Self {
            commands: parsed_commands,
        })
    }

    pub fn run(&mut self, str: &str) -> VimError<()> {
        info!("run command: {}", str);
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
    use crate::command::Command;
    use high_level::prelude::ClientError;
    use test_log::test;

    #[test]
    fn new() {
        let runner = Runner::new(DEFAULT_COMMANDS).unwrap();

        println!("{:?}", runner);
        assert_eq!(runner.commands.len(), DEFAULT_COMMANDS.len());
    }

    #[test]
    fn default() {
        let _ = Runner::default();
    }

    #[derive(Debug)]
    struct TestCommand;

    impl Command<ClientError> for TestCommand {
        fn get_id(&self) -> &'static str {
            "test_command"
        }

        fn run(
            &mut self,
            _client: &high_level::prelude::Client,
            _command: &str,
        ) -> Result<(), ClientError> {
            Ok(())
        }
    }

    #[test]
    fn run() {
        let mut runner = Runner::new(&[&TestCommand]).unwrap();
        runner.run("test_command").unwrap();
    }

    #[test]
    fn identical_id() {
        let runner = Runner::new(&[&AddCrypto, &AddCrypto]);
        assert!(matches!(runner, Err(error::Error::IdenticalId)));
    }

    #[test]
    fn not_found_command() {
        let mut runner = Runner::new(&[&TestCommand]).unwrap();
        assert!(matches!(runner.run("not_command"), Err(error::Error::NotFoundCommand)));
    }
}
