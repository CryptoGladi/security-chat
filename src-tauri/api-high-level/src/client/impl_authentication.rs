use super::{Client, LowerLevelClient};
use crate::client::error::Error;
use crate::client::impl_config::ClientConfig;
use crate::prelude::ClientInitArgs;
use api_lower_level::authentication::tokens::RefreshToken;
use fcore::prelude::BincodeConfig;
use log::{debug, info};

impl Client {
    pub async fn registration(nickname: &str, init_args: ClientInitArgs) -> Result<Client, Error> {
        debug!("run registration...");

        let raw_client =
            LowerLevelClient::registration(nickname, init_args.address_to_server.clone()).await?;

        let cache = init_args.get_cache().await?;

        info!(
            "new registration: {}",
            raw_client.data_for_autification.nickname
        );

        Ok(Self {
            config: ClientConfig {
                data_for_autification: raw_client.data_for_autification.clone(),
                ..Default::default()
            },
            _cache: cache,
            lower_level_client: raw_client,
            bincode_config: BincodeConfig::new(init_args.path_to_config_file),
        })
    }

    #[deprecated(note = "it is function load without config. Please use `Client::login_by_config`")]
    pub async fn login(
        init_args: ClientInitArgs,
        nickname: String,
        refresh_token: RefreshToken,
    ) -> Result<Self, Error> {
        debug!("run login...");

        let raw_client =
            LowerLevelClient::login(init_args.address_to_server.clone(), nickname, refresh_token)
                .await?;

        let cache = init_args.get_cache().await?;

        Ok(Self {
            config: ClientConfig {
                data_for_autification: raw_client.data_for_autification.clone(),
                ..Default::default()
            },
            _cache: cache,
            lower_level_client: raw_client,
            bincode_config: BincodeConfig::new(init_args.path_to_config_file),
        })
    }

    pub async fn nickname_is_taken(
        init_config: &ClientInitArgs,
        nickname: &str,
    ) -> Result<bool, Error> {
        debug!("run nickname_is_taken");

        Ok(api_lower_level::client::Client::nickname_is_taken(
            nickname,
            init_config.address_to_server.clone(),
        )
        .await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::get_client;
    use fcore::test_utils::{get_rand_string, PathsForTest, ADDRESS_SERVER};
    use test_log::test;

    #[test(tokio::test)]
    async fn nickname_is_taken() {
        let (_paths, client_config, client) = get_client().await;

        assert!(
            Client::nickname_is_taken(&client_config, client.get_nickname().as_str())
                .await
                .unwrap()
        );
        assert!(
            !Client::nickname_is_taken(&client_config, &get_rand_string(20))
                .await
                .unwrap()
        );
    }

    #[test(tokio::test)]
    #[allow(deprecated)]
    async fn login() {
        let (_paths, init_args, client) = get_client().await;

        let nickname = client.config.data_for_autification.nickname.clone();
        let refresh_token = client.config.data_for_autification.refresh_token.clone();

        Client::login(init_args, nickname, refresh_token)
            .await
            .unwrap();
    }

    #[test(tokio::test)]
    async fn registration() {
        let paths = PathsForTest::get();
        let client_config =
            ClientInitArgs::new(paths.path_to_config_file.clone(), ADDRESS_SERVER, None).unwrap();

        let _client = Client::registration(&get_rand_string(20), client_config.clone())
            .await
            .unwrap();
    }
}
