use high_level::prelude::*;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::path::PathBuf;

pub use temp_dir::TempDir;
pub const ADDRESS_SERVER: &str = "http://[::1]:2052";

pub fn get_rand_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect::<String>()
}

pub struct PathsForTest {
    _temp_dir: TempDir, // for lifetime
    path_to_config_file: PathBuf,
    path_to_cache: PathBuf,
}

impl PathsForTest {
    fn get() -> Self {
        let temp_dir = TempDir::new().unwrap();

        Self {
            path_to_config_file: temp_dir.child("config.bin"),
            path_to_cache: temp_dir.child("cache.db"),
            _temp_dir: temp_dir,
        }
    }
}

pub async fn get_client() -> (PathsForTest, ClientInitConfig, Client) {
    let paths = PathsForTest::get();
    let client_config = ClientInitConfig::new(
        paths.path_to_config_file.clone(),
        paths.path_to_cache.clone(),
        ADDRESS_SERVER,
    );
    let client = Client::registration(&get_rand_string(), client_config.clone())
        .await
        .unwrap();

    (paths, client_config, client)
}
