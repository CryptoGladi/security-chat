use crate::service::SecurityChatService;
pub use crate_proto::SecurityChatServer;
use tokio::sync::broadcast;

pub async fn get_service<'a>(broadcast_capacity: usize, secret: Vec<u8>) -> SecurityChatService {
    let db_pool = database::establish_pooled_connection().await;

    let (producer, consumer) = broadcast::channel(broadcast_capacity);
    SecurityChatService {
        db_pool,
        producer,
        consumer,
        secret,
    }
}
