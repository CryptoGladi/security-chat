use crate::service::{
    security_chat::security_chat_server::SecurityChatServer, SecurityChatService,
};
use dotenv::dotenv;
use log::warn;
use mimalloc::MiMalloc;
use tonic::codec::CompressionEncoding;
use tonic::transport::Server;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub mod database;
pub mod logger;
pub mod models;
pub mod schema;
pub mod service;

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    dotenv().ok();
    logger::init_logger().unwrap();

    warn!("running server...");

    let addr = "[::1]:2052".parse()?;
    let db_pool = database::establish_pooled_connection();
    let service = SecurityChatService { db_pool };

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
