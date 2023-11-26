use super::*;

pub const DEFAULT_COMMANDS: &[&dyn Command] = crate::command::ALL_COMMANDS;

/// Builder for [`Runner`]
#[derive(Debug)]
pub struct RunnerBuilder<'a> {
    commands: Vec<&'a dyn Command>,
    limit_for_fuzzy_search: usize,
}

impl<'a> Default for RunnerBuilder<'a> {
    fn default() -> Self {
        Self {
            commands: DEFAULT_COMMANDS.to_vec(),
            limit_for_fuzzy_search: 10,
        }
    }
}

macro_rules! impl_setter {
    ($name:ident, $x:ty) => {
        #[must_use]
        pub fn $name(mut self, $name: $x) -> Self {
            self.$name = $name;
            self
        }
    };

    ($name:ident, $x:ty, $cond:expr) => {
        #[must_use]
        pub fn $name(mut self, $name: $x) -> Self {
            assert!($cond);
            self.$name = $name;
            self
        }
    };
}

impl<'a> RunnerBuilder<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            commands: vec![],
            limit_for_fuzzy_search: 1,
        }
    }

    impl_setter!(commands, Vec<&'a dyn Command>);

    impl_setter!(limit_for_fuzzy_search, usize, limit_for_fuzzy_search >= 1);

    pub fn build(self) -> VimError<Runner<'a>> {
        trace!("run `build`");

        let mut parsed_commands = HashMap::new();

        for i in self.commands.iter() {
            if parsed_commands.insert(i.get_id(), *i).is_some() {
                return Err(error::Error::IdenticalId);
            }
        }

        Ok(Runner {
            commands: parsed_commands,
            limit_for_fuzzy_search: self.limit_for_fuzzy_search,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use test_log::test;

    #[test]
    fn build() {
        let runner = RunnerBuilder::new()
            .commands(DEFAULT_COMMANDS.to_vec())
            .build()
            .unwrap();

        let commands_from_runner = runner
            .commands
            .values()
            .map(|x| x.get_id())
            .collect::<Vec<&str>>();

        let commands_from_const = DEFAULT_COMMANDS
            .iter()
            .map(|x| x.get_id())
            .collect::<Vec<&str>>();

        assert_eq!(commands_from_runner, commands_from_const);
    }

    #[test]
    fn default() {
        let _ = RunnerBuilder::default().build().unwrap();
    }

    #[test]
    fn identical_id() {
        let runner = RunnerBuilder::new()
            .commands(vec![&TestCommand, &TestCommand])
            .build();

        assert!(matches!(runner, Err(error::Error::IdenticalId)));
    }

    #[test]
    #[should_panic]
    fn limit_fuzzy_panic() {
        let _runner = RunnerBuilder::new()
            .limit_for_fuzzy_search(0)
            .build()
            .unwrap();
    }
}
