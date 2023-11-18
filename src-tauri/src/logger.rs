//! Logger

use fern::colors::{Color, ColoredLevelConfig};
use hashbrown::HashSet;
use log::LevelFilter;

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
    /// Init [`fern`]
    pub fn init(self) -> Result<(), fern::InitError> {
        let colors = ColoredLevelConfig {
            info: Color::Blue,
            debug: Color::Cyan,
            ..Default::default()
        };

        let mut fern_config = fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "[{} {} {}:{}] {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    colors.color(record.level()),
                    record.target(),
                    record.line().unwrap_or(0), // ZERO IN ERROR!
                    message
                ));
            })
            .level(self.standard_level_for_crate);

        for target in self.level_for_targets.iter() {
            fern_config = fern_config.level_for(target.name.clone(), target.level);
        }

        let data = chrono::Local::now().format("%Y-%m-%d").to_string();
        let folder_for_logging = crate::path::get_app_folder().join("log");
        std::fs::create_dir_all(folder_for_logging.clone()).unwrap();

        fern_config
            .chain(std::io::stdout())
            .chain(fern::log_file(
                folder_for_logging.join(format!("{}.log", data)),
            )?)
            .apply()?;

        Ok(())
    }
}

/// Simple function for init [`Logger`]
pub fn init_logger() {
    let logger = Logger {
        standard_level_for_crate: LevelFilter::Error,
        level_for_targets: CrateLevelFilter::new(ALL_WORKSPACE_CRATE, LevelFilter::Trace),
    };

    logger.init().expect("run init function for logger");
}

#[cfg(test)]
mod tests {
    use super::{CrateLevelFilter, HashSet};
    use log::LevelFilter;

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

    #[test]
    fn test_all_workspace_crate() {
        let _crate_info_for_test =
            CrateLevelFilter::new(super::ALL_WORKSPACE_CRATE, LevelFilter::Warn);
    }
}
