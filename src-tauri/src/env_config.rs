use log::warn;

pub fn env_var(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{} must be set", name))
}

pub fn init() {
    dotenv::dotenv().ok();

    warn!("env address server: {}", env_var("ADDRESS_SERVER"));
    warn!("env folder name: {}", env_var("FOLDER_NAME"));
}
