use serde::{de::DeserializeOwned, Serialize};
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
    Serde(bincode::Error),
}

pub fn save<T: Serialize + DeserializeOwned>(
    data: &T,
    path: impl AsRef<Path>,
) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .map_err(Error::IO)?;

    let bincode = bincode::serialize(&data).map_err(Error::Serde)?;
    file.write_all(&bincode).map_err(Error::IO)?;
    file.flush().map_err(Error::IO)?;

    Ok(())
}

/// Загрузить [`ClientData`]
///
/// **Загрузить можно только если файл существует!**
pub fn load<T: Serialize + DeserializeOwned>(path: impl AsRef<Path>) -> Result<T, Error> {
    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(Error::IO)?;

    let reader = BufReader::new(file);
    bincode::deserialize_from(reader).map_err(Error::Serde)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use temp_dir::TempDir;
    use test_log::test;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Data {
        text: String,
        h: bool,
        num: i32,
    }

    #[test]
    fn save_and_load() {
        let data = Data {
            text: "dd".to_string(),
            h: true,
            num: 532,
        };
        let temp_dir = TempDir::new().unwrap();
        let file = temp_dir.child("config");

        super::save(&data, file.clone()).unwrap();
        let loaded_data = super::load(file).unwrap();

        assert_eq!(data, loaded_data);
    }
}
