use hashbrown::HashSet;
use log::LevelFilter;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("already have: {0}")]
    AlreadyHave(String),
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CrateInfo {
    /// Name crate
    pub name: String,

    /// Level for crate
    pub level: LevelFilter,
}

/// [`log::LevelFilter`] but for crates
#[derive(Debug, PartialEq)]
pub struct CratesForLevelFilter {
    pub crates: HashSet<CrateInfo>,
}

impl CratesForLevelFilter {
    /// # Panics
    ///
    /// If already have crate level filter
    pub fn new(names: &[&str], level_standart: LevelFilter) -> Result<Self, Error> {
        let mut result = HashSet::with_capacity(names.len());

        for name in names {
            let did_not_previously_contain = result.insert(CrateInfo {
                name: (*name).to_string(),
                level: level_standart,
            });

            if !did_not_previously_contain {
                return Err(Error::AlreadyHave((*name).to_string()));
            }
        }

        Ok(Self { crates: result })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_for_crate_level_filter() {
        let crate_info_for_test =
            CratesForLevelFilter::new(&["test1", "test2"], LevelFilter::Warn).unwrap();

        let crate_info_eq = {
            let mut result = HashSet::new();

            result.insert(CrateInfo {
                name: "test1".to_string(),
                level: LevelFilter::Warn,
            });

            result.insert(CrateInfo {
                name: "test2".to_string(),
                level: LevelFilter::Warn,
            });

            CratesForLevelFilter { crates: result }
        };

        assert_eq!(crate_info_for_test, crate_info_eq);
    }

    #[test]
    fn panic_already_have_crate_level_filter() {
        let result = CratesForLevelFilter::new(&["test1", "test1"], LevelFilter::Warn);
        assert_eq!(result, Err(Error::AlreadyHave("test1".to_string())));
    }
}
