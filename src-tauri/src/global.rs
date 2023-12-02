use api_high_level::prelude::*;
use fcore::prelude::*;
use once_cell::sync::Lazy;
use tauri::async_runtime::{Mutex, RwLock};
use vim_like_command::prelude::*;

pub static CLIENT_INIT_CONFIG: Lazy<ClientInitArgs> = Lazy::new(|| {
    let dir = crate::path::get_app_folder();

    ClientInitArgs::new(
        dir.join("config.bin"),
        get_env_var("ADDRESS_SERVER"),
        Some(dir.join("cache.db")),
    )
    .unwrap()
});

pub static VIM_RUNNER: Lazy<Mutex<Runner<'_>>> =
    Lazy::new(|| Mutex::new(RunnerBuilder::default().build().unwrap()));

pub static LOADED_CLIENT: Lazy<RwLock<Option<Client>>> = Lazy::new(|| RwLock::new(None));
