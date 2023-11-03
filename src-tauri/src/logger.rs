//! Logger

use hashbrown::HashSet;
use log::{LevelFilter, SetLoggerError};

const ALL_WORKSPACE_CRATE: &[&str] = &[
    "vim_like_command",
    "api_lower_level",
    "api_high_level",
    "fcore",
    "crate_proto",
    "cache",
    "security_chat",
];

/// [`log::LevelFilter`] but for crate
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CrateLevelFilter {
    /// Name crate
    pub name: String,

    /// Level for crate
    pub level: LevelFilter,
}

impl CrateLevelFilter {
    pub fn new(names: &[&str], level: LevelFilter) -> HashSet<Self> {
        let mut result = HashSet::with_capacity(names.len());

        for name in names {
            let did_not_previously_contain = result.insert(Self {
                name: (*name).to_string(),
                level,
            });

            assert!(
                did_not_previously_contain,
                "already have crate level filter: {}",
                name
            );
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
        level_for_targets: CrateLevelFilter::new(ALL_WORKSPACE_CRATE, LevelFilter::Trace),
    };

    logger.init().expect("run init function for logger");
}

#[cfg(test)]
mod tests {
    use super::{CrateLevelFilter, HashSet, Logger};
    use log::LevelFilter;

    #[test]
    fn raw_init() {
        let logger = Logger {
            standard_level_for_crate: LevelFilter::Off,
            level_for_targets: CrateLevelFilter::new(&[], LevelFilter::Error),
        };

        logger.init().unwrap();
    }

    #[test]
    fn new_for_crate_level_filter() {
        let crate_info_for_test = CrateLevelFilter::new(&["test1", "test2"], LevelFilter::Warn);

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

    #[test]
    #[should_panic(expected = "already have crate level filter: test1")]
    fn panic_already_have_crate_level_filter() {
        let _crate_info_for_test = CrateLevelFilter::new(&["test1", "test1"], LevelFilter::Warn);
    }
}
