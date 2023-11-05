use fcore::prelude::*;
use std::path::PathBuf;

pub fn get_app_folder() -> PathBuf {
    dirs::data_local_dir()
        .unwrap()
        .join(get_env_var("FOLDER_NAME"))
}
