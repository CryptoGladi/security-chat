use crate::service::{
    security_chat::security_chat_server::SecurityChatServer, SecurityChatService,
};
use dotenv::dotenv;
use log::warn;
use tokio::sync::broadcast;
use tonic::codec::CompressionEncoding;
use tonic::transport::Server;
use schema::database;

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

    warn!("running server...");

    let addr = "[::1]:2052".parse()?;
    let db_pool = database::establish_pooled_connection().await;

    let (producer, consumer) = broadcast::channel(100_000);
    let service = SecurityChatService {
        db_pool,
        producer,
        consumer,
    };

    Server::builder()
        .add_service(
            SecurityChatServer::new(service)
                .send_compressed(CompressionEncoding::Gzip)
                .accept_compressed(CompressionEncoding::Gzip),
        )
        .serve(addr)
        .await?;

    Ok(())
}
