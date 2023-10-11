//! Logger

use log::{LevelFilter, SetLoggerError};
use map_macro::hash_set;
use std::collections::HashSet;

/// [`log::LevelFilter`] but for crate
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CrateLevelFilter {
    /// Name crate
    pub name: String,

    /// Level for crate
    pub level: LevelFilter,
}

impl CrateLevelFilter {
    pub fn new(names: HashSet<&str>, level: LevelFilter) -> HashSet<Self> {
        let mut result = HashSet::with_capacity(names.len());

        for name in names {
            result.insert(Self {
                name: name.to_string(),
                level,
            });
        }

        result
    }
}

/// Raw logger
///
/// **Better use: [`init_logger`]**
pub struct Logger {
    /// standard level [`log::LevelFilter`] for **all** crate
    pub standard_level_for_crate: LevelFilter,

    /// [`log::LevelFilter`] for **specific** crate
    pub level_for_targets: HashSet<CrateLevelFilter>,
}

impl Logger {
    /// Init [`simple_logger::SimpleLogger`]
    pub fn init(self) -> Result<(), SetLoggerError> {
        let mut logger = simple_logger::SimpleLogger::default();
        logger = logger.with_level(self.standard_level_for_crate);

        for target in self.level_for_targets.iter() {
            logger = logger.with_module_level(&target.name, target.level);
        }

        logger.init()
    }
}

/// Smart init [`Logger`]
pub fn init_logger() {
    let logger = Logger {
        standard_level_for_crate: LevelFilter::Error,
        level_for_targets: CrateLevelFilter::new(
            hash_set! {
                "vim-like-command", "lower-level", "high-level", "fcore", "crate-proto", "cache"
            },
            LevelFilter::Trace,
        ),
    };

    logger.init().expect("run init function for logger");
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{CrateLevelFilter, Logger};
    use log::LevelFilter;
    use map_macro::hash_set;

    #[test]
    fn raw_init() {
        let logger = Logger {
            standard_level_for_crate: LevelFilter::Off,
            level_for_targets: CrateLevelFilter::new(hash_set! {}, LevelFilter::Error),
        };

        logger.init().unwrap();
    }

    #[test]
    fn new_for_crate_level_filter() {
        let crate_info_for_test =
            CrateLevelFilter::new(hash_set! {"test1", "test2"}, LevelFilter::Warn);

        let crate_info_eq = {
            let mut result = HashSet::new();

            result.insert(CrateLevelFilter {
                name: "test1".to_string(),
                level: LevelFilter::Warn,
            });

            result.insert(CrateLevelFilter {
                name: "test2".to_string(),
                level: LevelFilter::Warn,
            });

            result
        };

        assert_eq!(crate_info_for_test, crate_info_eq);
    }
}
