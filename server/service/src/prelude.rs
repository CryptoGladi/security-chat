use crate::database;
use crate::service::authentication::{intercept, self};
use crate::service::SecurityChatService;
use crate_proto::SecurityChatServer;
use std::collections::HashSet;
use std::sync::Mutex;
use tokio::sync::broadcast;
use tonic::codec::CompressionEncoding;
use tonic::service::interceptor::InterceptorLayer;
use tonic::Request;
use tonic::transport::Server;
use std::time::Duration;
use std::env;

pub async fn get_service<'a>(broadcast_capacity: usize) -> SecurityChatServer<SecurityChatService> {
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