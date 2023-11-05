//! Module for `ONLY` testing

use crate::prelude::*;
use fcore::test_utils::*;

pub async fn get_client() -> (PathsForTest, ClientInitArgs, Client) {
    let paths = PathsForTest::get();

    let client_config =
        ClientInitArgs::new(paths.path_to_config_file.clone(), ADDRESS_SERVER, None);

    let client = Client::registration(&get_rand_string(20), client_config.clone())
        .await
        .unwrap();

    (paths, client_config, client)
}
