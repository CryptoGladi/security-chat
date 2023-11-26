use fcore::prelude::*;
use std::path::PathBuf;

/// Get folder for application
///
/// # Panics
///
/// The returned value depends on the operating system.
/// If the folder is inaccessible in the operating system, there will be a panic.
#[must_use]
pub fn get_app_folder() -> PathBuf {
    dirs::data_local_dir()
        .unwrap()
        .join(get_env_var("FOLDER_NAME"))
}
