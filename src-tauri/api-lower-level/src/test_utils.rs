//! Module for `ONLY` testing

use fcore::test_utils::*;
use crate::client::Client;
use crate::client::error::Error;

/// Get client for `testing`
pub async fn get_client() -> Result<Client, Error> {
    let nickname = get_rand_string(20);
    Client::registration(&nickname, ADDRESS_SERVER.parse().unwrap())
        .await
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn get_client() {
        let _new_client = super::get_client().await.unwrap();
    }
}