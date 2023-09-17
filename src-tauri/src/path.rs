use std::path::PathBuf;
use fcore::prelude::*;

pub fn get_app_folder() -> PathBuf {
    // TODO https://tauri.app/v1/api/js/path/
    dirs::config_local_dir()
        .unwrap()
        .join(get_env_var("FOLDER_NAME"))
}
