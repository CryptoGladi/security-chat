use high_level::prelude::*;
use once_cell::sync::Lazy;

pub static CLIENT_INIT_CONFIG: Lazy<ClientInitConfig> = Lazy::new(|| get_client_init_config());

pub const ADDRESS_SERVER: &str = "http://[::1]:2052";

pub fn get_client_init_config() -> ClientInitConfig {
    let dir = crate::path::get_app_folder();

    ClientInitConfig::new(dir.join("config.bin"), ADDRESS_SERVER)
}