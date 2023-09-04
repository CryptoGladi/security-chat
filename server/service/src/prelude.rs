use crate::database;
use crate::service::SecurityChatService;
use crate_proto::SecurityChatServer;
use tokio::sync::broadcast;
use tonic::codec::CompressionEncoding;

pub async fn get_service(broadcast_capacity: usize) -> SecurityChatServer<SecurityChatService> {
    let db_pool = database::establish_pooled_connection().await;

    let (producer, consumer) = broadcast::channel(broadcast_capacity);
    let service = SecurityChatService {
        db_pool,
        producer,
        consumer,
    };

    SecurityChatServer::new(service)
        .send_compressed(CompressionEncoding::Gzip)
        .accept_compressed(CompressionEncoding::Gzip)
}
