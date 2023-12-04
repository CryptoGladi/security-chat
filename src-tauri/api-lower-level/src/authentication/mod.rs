use crate::client::error::Error;
use crate_proto::authentication::{CheckTokenRequest, LoginRequest, RegistrationRequest};
use crate_proto::AuthenticationClient as GRPCAuthenticationClient;
use derivative::Derivative;
use http::Uri;
use log::trace;
use tonic::codec::CompressionEncoding;
use tonic::transport::Channel;

const COMPRESSION_ENCODING: CompressionEncoding = CompressionEncoding::Gzip;
pub type AccessToken = String;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct JWTTokens {
    /// Token for get [`JWTTokens::access_token`]
    ///
    /// Unlimited lifetime
    ///
    /// **IT IS VERY SECRET VALUE!**
    #[derivative(Debug = "ignore")]
    pub refresh_token: String,

    /// Token for execution on your account.
    ///
    /// There is a lifetime limit! Determined by server
    ///
    /// **IT IS SECRET VALUE!**
    #[derivative(Debug = "ignore")]
    pub access_token: String,
}

pub struct AuthenticationClient {
    grpc_api: GRPCAuthenticationClient<Channel>,
}

impl AuthenticationClient {
    /// Init [gRPC](https://grpc.io/) connect and enable compression
    pub async fn connect(address: Uri) -> Result<Self, Error> {
        trace!("run `grpc_connect` to address: {address}");

        let channel = Channel::builder(address).connect().await?;
        let grpc_api = GRPCAuthenticationClient::new(channel)
            .send_compressed(COMPRESSION_ENCODING)
            .accept_compressed(COMPRESSION_ENCODING);

        Ok(Self { grpc_api })
    }

    pub async fn registration(&mut self, nickname: String) -> Result<JWTTokens, tonic::Status> {
        trace!("run `registration` for nickname: {nickname}");

        let request = RegistrationRequest { nickname };
        let response = self.grpc_api.registration(request).await?;

        Ok(JWTTokens {
            refresh_token: response.get_ref().refresh_token.clone(),
            access_token: response.get_ref().access_token.clone(),
        })
    }

    pub async fn login(
        &mut self,
        nickname: String,
        refresh_token: String,
    ) -> Result<AccessToken, tonic::Status> {
        trace!("run `login` for nickname: {nickname}");

        let request = LoginRequest {
            nickname,
            refresh_token,
        };

        let response = self.grpc_api.login(request).await?;

        Ok(response.get_ref().access_token.clone())
    }

    pub async fn check_valid(&mut self, access_token: AccessToken) -> Result<bool, tonic::Status> {
        trace!("run `check_valid`");

        let request = CheckTokenRequest { access_token };
        let response = self.grpc_api.check_token(request).await?;

        Ok(response.get_ref().is_valid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fcore::test_utils::*;

    #[tokio::test]
    async fn connect() {
        AuthenticationClient::connect(ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn registration() {
        let mut client = AuthenticationClient::connect(ADDRESS_SERVER.parse().unwrap())
            .await
            .unwrap();

        let random_nickname = get_rand_string(20);
        let tokens = client.registration(random_nickname).await.unwrap();

        println!("access_token: {}", tokens.access_token);
        println!("refresh: {}", tokens.refresh_token);

        assert!(client
            .check_valid(tokens.access_token.clone())
            .await
            .unwrap());

        assert!(!client
            .check_valid(tokens.refresh_token.clone())
            .await
            .unwrap());
    }
}
