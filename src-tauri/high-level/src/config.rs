use lower_level::client::ClientData;
use std::{
    fs::OpenOptions,
    io::{BufReader, Write},
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("config io problem: `{0}`")]
    IO(std::io::Error),

    #[error("config serde problem: `{0}`")]
    Serde(serde_json::Error),
}

pub struct Config
{
    path: PathBuf,
}

impl Config
{
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf()
        }
    }

    pub fn save(&self, data: &ClientData) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.path.clone())
            .map_err(Error::IO)?;

        let json = serde_json::to_string_pretty(data).map_err(Error::Serde)?;
        file.write_all(json.as_bytes()).map_err(Error::IO)?;

        Ok(())
    }

    /// Загрузить [`ClientData`]
    ///
    /// **Загрузить можно только если файл существует!**
    pub fn load(&self) -> Result<ClientData, Error> {
        let file = OpenOptions::new()
            .read(true)
            .open(self.path.clone())
            .map_err(Error::IO)?;

        let reader = BufReader::new(file);
        serde_json::from_reader(reader).map_err(Error::Serde)
    }
}
