use high_level::prelude::*;
use once_cell::sync::Lazy;
use tauri::async_runtime::{Mutex, RwLock};
use vim_like_command::prelude::*;

use crate::env_config::env_var;

pub static CLIENT_INIT_CONFIG: Lazy<ClientInitConfig> = Lazy::new(get_client_init_config);
pub static VIM_RUNNER: Lazy<Mutex<Runner<'_>>> = Lazy::new(|| Mutex::new(get_runner()));
pub static LOADED_CLIENT: Lazy<RwLock<Option<Client>>> = Lazy::new(|| RwLock::new(None));

fn get_client_init_config() -> ClientInitConfig {
    let dir = crate::path::get_app_folder();

    ClientInitConfig::new(
        dir.join("config.bin"),
        dir.join("cache.db"),
        env_var("ADDRESS_SERVER"),
    )
}

fn get_runner<'a>() -> Runner<'a> {
    RunnerBuilder::default().build().unwrap()
}
