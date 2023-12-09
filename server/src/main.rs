use crate::secter::Secret;
use authentication::prelude::{grpc_intercept as authentication_intercept, AuthenticationServer};
use dotenv::dotenv;
use log::warn;
use service::prelude::{get_service as get_main_service, SecurityChatServer};
use std::env;
use std::path::PathBuf;
use tonic::codec::CompressionEncoding;
use tonic::transport::{Identity, Server, ServerTlsConfig};

#[cfg(not(debug_assertions))]
use mimalloc::MiMalloc;

#[cfg(not(debug_assertions))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub mod logger;
pub mod secter;

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    dotenv().ok();
    logger::init_logger().unwrap();

    let data_dir = PathBuf::from_iter([std::env!("CARGO_MANIFEST_DIR"), "testing_data"]);
    let cert = std::fs::read_to_string(data_dir.join("MyCertificate.crt"))?;
    let key = std::fs::read_to_string(data_dir.join("MyKey.key"))?;

    let identity = Identity::from_pem(cert, key);

    let addr = env::var("ADDRESS_BIND")
        .expect("ADDRESS_BIND must be set")
        .parse()?;

    let secret = Secret::get(PathBuf::from("SECRET.txt")).unwrap();
    let authentication_service = authentication::prelude::get_service(secret.0.clone()).await;
    let authentication_server = AuthenticationServer::new(authentication_service)
        .send_compressed(CompressionEncoding::Gzip)
        .accept_compressed(CompressionEncoding::Gzip);

    let app_service = get_main_service(100_000, secret.0.clone()).await;
    let app_server = SecurityChatServer::with_interceptor(app_service, move |request| {
        authentication_intercept(request, &secret.0)
    });

    warn!("running server by addr: {}", addr);

    Server::builder()
        .tls_config(ServerTlsConfig::new().identity(identity))
        .unwrap()
        .add_service(authentication_server)
        .add_service(app_server)
        .serve(addr)
        .await?;

    Ok(())
}
