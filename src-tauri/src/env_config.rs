use fcore::prelude::*;
use log::warn;

pub fn init() {
    dotenv::dotenv().ok();

    warn!("env address server: {}", get_env_var("ADDRESS_SERVER"));
    warn!("env folder name: {}", get_env_var("FOLDER_NAME"));
}
