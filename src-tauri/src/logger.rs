//! Logger

use crate_level_filter::CratesForLevelFilter;
use error::LoggerResult;
use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;

pub mod crate_level_filter;
pub mod error;

const ALL_WORKSPACE_CRATE: &[&str] = &[
    "vim_like_command",
    "api_lower_level",
    "api_high_level",
    "fcore",
    "crate_proto",
    "cache",
    "security_chat",
];

/// Raw logger
///
/// **Better use: [`init`]**
pub struct Logger {
    /// standard level [`log::LevelFilter`] for **all** crate
    pub standard_level_for_crate: LevelFilter,

    /// [`log::LevelFilter`] for **specific** crate
    pub level_for_targets: CratesForLevelFilter
}

impl Logger {
    /// Init [`fern`]
    pub fn init(self) -> LoggerResult<()> {
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

        for target in &self.level_for_targets.crates {
            fern_config = fern_config.level_for(target.name.clone(), target.level);
        }

        let data = chrono::Local::now().format("%Y-%m-%d").to_string();
        let folder_for_logging = crate::path::get_app_folder().join("log");
        std::fs::create_dir_all(folder_for_logging.clone())?;

        fern_config
            .chain(std::io::stdout())
            .chain(fern::log_file(
                folder_for_logging.join(format!("{data}.log")),
            )?)
            .apply()?;

        Ok(())
    }
}

/// Simple function for init [`Logger`]
pub fn init() -> LoggerResult<()> {
    let logger = Logger {
        standard_level_for_crate: LevelFilter::Error,
        level_for_targets: CratesForLevelFilter::new(ALL_WORKSPACE_CRATE, LevelFilter::Trace).unwrap(),
    };

    logger.init()
}

#[cfg(test)]
mod tests {
    use super::CratesForLevelFilter;
    use log::LevelFilter;

    #[test]
    fn test_all_workspace_crate() {
        CratesForLevelFilter::new(super::ALL_WORKSPACE_CRATE, LevelFilter::Warn).unwrap();
    }
}
