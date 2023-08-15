use std::sync::{Mutex, Arc};

use high_level::prelude::*;
use once_cell::sync::Lazy;
use vim_like_command::prelude::*;

pub static CLIENT_INIT_CONFIG: Lazy<ClientInitConfig> = Lazy::new(get_client_init_config);
pub static VIM_RUNNER: Lazy<Arc<Mutex<Runner>>> = Lazy::new(|| Arc::new(Mutex::new(get_runner())));

pub const ADDRESS_SERVER: &str = "http://[::1]:2052";

fn get_client_init_config() -> ClientInitConfig {
    let dir = crate::path::get_app_folder();

    ClientInitConfig::new(dir.join("config.bin"), ADDRESS_SERVER)
}

fn get_runner<'a>() -> Runner<'a> {
    RunnerBuilder::default().build().unwrap()
}