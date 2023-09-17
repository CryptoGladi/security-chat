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
    pub path_to_config_file: PathBuf,
    pub path_to_cache: PathBuf,
}

impl PathsForTest {
    pub fn get() -> Self {
        let temp_dir = TempDir::new().unwrap();

        Self {
            path_to_config_file: temp_dir.child("config.bin"),
            path_to_cache: temp_dir.child("cache.db"),
            _temp_dir: temp_dir,
        }
    }
}

/* 
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
*/

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use super::*;

    #[test]
    fn paths_for_test() {
        let paths = PathsForTest::get();
        
        assert_ne!(paths.path_to_cache, paths.path_to_config_file);
        assert_eq!(paths.path_to_cache.file_name(), Some(OsStr::new("cache.db")));
        assert_eq!(paths.path_to_config_file.file_name(), Some(OsStr::new("config.bin")));
    }
}