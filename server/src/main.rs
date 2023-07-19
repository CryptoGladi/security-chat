use log::warn;
use tonic::transport::Server;
use crate::service::{SecurityChatService, security_chat::security_chat_server::SecurityChatServer};

pub mod logger;
pub mod service;

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    logger::init_logger().unwrap();

    warn!("done run server");
    
    let addr = "[::1]:2052".parse()?;
    let service = SecurityChatService::default();

    Server::builder()
        .add_service(SecurityChatServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
