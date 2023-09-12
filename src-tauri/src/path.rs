use std::path::PathBuf;

pub fn get_app_folder() -> PathBuf {
    // TODO https://tauri.app/v1/api/js/path/
    dirs::config_local_dir().unwrap().join("security-chat02")
}
