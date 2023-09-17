use log::warn;
use fcore::prelude::*;

pub fn init() {
    dotenv::dotenv().ok();

    warn!("env address server: {}", env_var("ADDRESS_SERVER"));
    warn!("env folder name: {}", env_var("FOLDER_NAME"));
}
