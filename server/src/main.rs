use crate::service::{
    security_chat::security_chat_server::SecurityChatServer, SecurityChatService,
};
use dotenv::dotenv;
use log::warn;
use tonic::transport::Server;

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
    let service = SecurityChatService::default();
    
    Server::builder()
        .add_service(SecurityChatServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}