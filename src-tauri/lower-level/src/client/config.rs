use super::ClientData;
use std::{
    fs::OpenOptions,
    io::{BufReader, Write},
    path::Path,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("config io problem: `{0}`")]
    IO(std::io::Error),

    #[error("config serde problem: `{0}`")]
    Serde(serde_json::Error),
}

pub fn save(data: &ClientData, path: impl AsRef<Path>) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .map_err(Error::IO)?;

    let json = serde_json::to_string_pretty(data).map_err(Error::Serde)?;
    file.write_all(json.as_bytes()).map_err(Error::IO)?;

    Ok(())
}

/// Загрузить [`ClientData`]
///
/// **Загрузить можно только если файл существует!**
pub fn load(path: impl AsRef<Path>) -> Result<ClientData, Error> {
    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(Error::IO)?;

    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(Error::Serde)
}
