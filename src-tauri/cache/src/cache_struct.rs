use error::{CacheResult, Error};
use log::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub mod error;

#[derive(Debug)]
pub struct CacheStruct {
    db: rocksdb::DB,
}

impl CacheStruct {
    pub fn new<P>(path: P) -> CacheResult<Self>
    where
        P: AsRef<std::path::Path>,
    {
        info!("new with path: {}", path.as_ref().display());
        let mut options = rocksdb::Options::default();
        options.create_if_missing(true);
        options.create_missing_column_families(true);
        options.increase_parallelism(1);
        options.set_compression_type(rocksdb::DBCompressionType::Zstd);

        let db = rocksdb::DB::open(&options, path)?;

        Ok(Self { db })
    }

    pub fn put<V>(&mut self, key: &str, value: &V) -> CacheResult<()>
    where
        V: Serialize + Debug,
    {
        info!("put key={} value{:?}", key, value);
        let value = bincode::serialize(&value)?;
        Ok(self.db.put(key, value)?)
    }

    pub fn get<V>(&self, key: &str) -> CacheResult<V>
    where
        V: DeserializeOwned,
    {
        info!("get key={}", key);
        let Some(value) = self.db.get(key)? else {
            return Err(Error::NotFound);
        };

        Ok(bincode::deserialize(&value)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use temp_dir::TempDir;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestValue {
        name: String,
        number: i32,
        love: bool,
    }

    #[test]
    fn put() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = CacheStruct::new(temp_dir.path()).unwrap();

        let value = TestValue {
            name: "CryptoGladi".to_string(),
            number: 641,
            love: false,
        };

        cache.put("new-key", &value).unwrap();
    }

    #[test]
    fn get() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = CacheStruct::new(temp_dir.path()).unwrap();

        let value = TestValue {
            name: "CryptoGladi".to_string(),
            number: 641,
            love: false,
        };

        cache.put("new-key", &value).unwrap();

        let got_value: TestValue = cache.get("new-key").unwrap();
        assert_eq!(value, got_value);
    }

    #[test]
    fn get_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let cache = CacheStruct::new(temp_dir.path()).unwrap();

        assert!(matches!(
            cache.get::<TestValue>("not_key"),
            Err(Error::NotFound)
        ));
    }

    #[test]
    fn new() {
        let temp_dir = TempDir::new().unwrap();
        let _cache = CacheStruct::new(temp_dir.path()).unwrap();
    }
}
