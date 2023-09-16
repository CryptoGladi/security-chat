use std::path::PathBuf;

use crate::env_config::env_var;

pub fn get_app_folder() -> PathBuf {
    // TODO https://tauri.app/v1/api/js/path/
    dirs::config_local_dir().unwrap().join(env_var("FOLDER_NAME"))
}
