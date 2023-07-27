use crate::service::{
    security_chat::security_chat_server::SecurityChatServer, SecurityChatService,
};
use dotenv::dotenv;
use log::warn;
use tonic::codec::CompressionEncoding;
use tonic::transport::Server;

#[cfg(not(debug_assertions))]
use mimalloc::MiMalloc;

#[cfg(not(debug_assertions))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub mod database;
pub mod logger;
pub mod models;
pub mod schema;
pub mod service;

fn main() -> Result<(), color_eyre::Report> {
    tokio_uring::start(async {
        color_eyre::install()?;
        dotenv().ok();
        logger::init_logger().unwrap();

        warn!("running server...");

        let addr = "[::1]:2052".parse()?;
        let db_pool = database::establish_pooled_connection().await;
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
    })
}
