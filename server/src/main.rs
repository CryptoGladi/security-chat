use dotenv::dotenv;
use log::warn;
use service::prelude::get_service;
use std::env;
use std::time::Duration;
use tonic::transport::Server;
use tonic_async_interceptor::async_interceptor;
use log::info;

#[cfg(not(debug_assertions))]
use mimalloc::MiMalloc;

#[cfg(not(debug_assertions))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub mod logger;

fn get_string_by_metadata<'a>(request: &'a tonic::Request<()>, name: &'a str) -> Result<&'a str, tonic::Status> {
    let metadata = match request.metadata().get(name) {
        Some(metadata) => metadata.to_str(),
        None => {
            info!(
                "Token not found for client for request: {:?}",
                request.get_ref()
            );

            return Err(tonic::Status::invalid_argument(
                "Token not found in metadata",
            ));
        }
    };

    metadata.map_err(|_| tonic::Status::cancelled("error to_str()"))
}

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    dotenv().ok();
    logger::init_logger().unwrap();

    let addr = env::var("ADDRESS_BIND")
        .expect("ADDRESS_BIND must be set")
        .parse()?;

    warn!("running server by addr: {}", addr);

    let layer = tower::ServiceBuilder::new()
        .timeout(Duration::from_secs(30))
        .layer(async_interceptor(|request| async {
            let pool_db = service::database::establish_pooled_connection().await;
            let db = pool_db.get().await.unwrap();

            let nickname = get_string_by_metadata(&request, "nickname")?;
           // let authkey = get_string_by_metadata(&request, "authkey")?;

            Ok(request)
        }))
        .into_inner();

    Server::builder()
        .layer(layer)
        .add_service(get_service(100_000).await)
        .serve(addr)
        .await?;

    Ok(())
}
