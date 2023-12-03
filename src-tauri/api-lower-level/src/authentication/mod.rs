use crate::client::error::Error;
use crate_proto::authentication::RegistrationRequest;
use crate_proto::AuthenticationClient as GRPCAuthenticationClient;
use educe::Educe;
use http::Uri;
use log::trace;
use tonic::codec::CompressionEncoding;
use tonic::transport::Channel;

const COMPRESSION_ENCODING: CompressionEncoding = CompressionEncoding::Gzip;

#[derive(Hash)]
#[Educe(Debug)]
pub struct DataForLogin {
    #[educe(Debug(ignore))]
    pub refresh_string: String,
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

    pub async fn registration(&mut self, nickname: String) -> Result<String, tonic::Status> {
        trace!("run `registration` for nickname: {nickname}");

        let request = RegistrationRequest { nickname };
        let response = self.grpc_api.registration(request).await?;

        Ok(response.get_ref().access_token.clone())
    }

    pub async fn login(&mut self) {
        todo!()
    }
}
