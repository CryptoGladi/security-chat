use dotenv::dotenv;
use log::warn;
use service::prelude::get_service;
use std::env;
use std::time::Duration;
use tonic::transport::Server;

#[cfg(not(debug_assertions))]
use mimalloc::MiMalloc;

#[cfg(not(debug_assertions))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub mod logger;

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
        .layer(tonic::service::interceptor(authentication::intercept))
        .into_inner();

    Server::builder()
        .layer(layer)
        .add_service(get_service(100_000).await)
        .serve(addr)
        .await?;

    Ok(())
}
