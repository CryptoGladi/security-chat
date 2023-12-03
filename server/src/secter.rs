use log::{info, warn};
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::PathBuf;

const LEN_SECRET: usize = 256;

pub struct Secret(pub Vec<u8>);

impl Secret {
    fn generate(len: usize) -> Self {
        warn!("generate with len: {len}");
        let secret: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect();

        Self(secret.into_bytes())
    }

    fn save(&self, path: PathBuf) -> Result<(), std::io::Error> {
        warn!("save with path: {}", path.display());
        let mut file = File::create(path)?;
        file.write_all(&self.0)?;

        Ok(())
    }

    fn load(path: PathBuf) -> Result<Self, std::io::Error> {
        info!("load with path: {}", path.display());
        Ok(Self(read_to_string(path)?.into_bytes()))
    }

    pub fn get(path: PathBuf) -> Result<Self, std::io::Error> {
        info!("get with path: {}", path.display());

        if !path.is_file() {
            let new_secret = Self::generate(LEN_SECRET);
            new_secret.save(path.clone())?;
        }

        Ok(Self(Self::load(path.clone())?.0))
    }
}
