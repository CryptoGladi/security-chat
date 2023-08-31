use crate::cache_struct::db_trait::SQLite;
use crate::cache_struct::Cache;

pub type CacheSQLite = Cache<SQLite>;
pub use crate::cache_struct::db_trait::DBOptions;
