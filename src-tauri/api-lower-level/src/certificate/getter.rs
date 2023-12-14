//! Module for obtaining information for a [certificate](Certificate)

use crate::certificate::connection_parameters::ConnectionParameters;
use crate::certificate::Certificate;
use async_trait::async_trait;
use std::path::PathBuf;

pub mod impl_json;

#[async_trait]
pub trait Getter {
    async fn get(&self, path: PathBuf, connection_parameters: ConnectionParameters) -> Certificate;
}
