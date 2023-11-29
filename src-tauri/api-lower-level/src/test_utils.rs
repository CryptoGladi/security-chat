//! Module for `ONLY` testing

use crate::client::error::Error;
use crate::client::Client;
use fcore::test_utils::*;

/// Get client for `testing`
///
/// # Panics
///
/// TODO
pub async fn get_client() -> Result<Client, Error> {
    let nickname = get_rand_string(20);
    Client::registration(&nickname, ADDRESS_SERVER.parse().unwrap()).await
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn get_client() {
        let _new_client = super::get_client().await.unwrap();
    }
}
