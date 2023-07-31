use std::path::{PathBuf, Path};

pub struct ClientConfig {
    pub path_to_config_file: PathBuf,
    pub path_to_usernames: PathBuf,
    pub address_to_server: http::Uri,
}

impl ClientConfig {
    pub fn new(
        path_to_config_file: impl AsRef<Path>,
        path_to_usernames: impl AsRef<Path>,
        address_to_server: impl TryInto<http::Uri>,
    ) -> Self {
        let Ok(address_to_server) = address_to_server.try_into() else {
            panic!("address_to_server.try_into() error");
        };

        Self {
            path_to_config_file: path_to_config_file.as_ref().to_path_buf(),
            path_to_usernames: path_to_usernames.as_ref().to_path_buf(),
            address_to_server,
        }
    }
}