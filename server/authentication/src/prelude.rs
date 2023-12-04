pub use crate::service::grpc_intercept;
use crate::service::AuthenticationService;
use chrono::Duration;
pub use crate_proto::AuthenticationServer;

pub async fn get_service(secret: Vec<u8>) -> AuthenticationService {
    log::error!("secter: {}", String::from_utf8(secret.clone()).unwrap());
    let db_pool = database::establish_pooled_connection().await;
    AuthenticationService {
        db_pool,
        secret: String::from_utf8(secret.clone()).unwrap().into_bytes(),
        lifetime_for_tokens: Duration::days(1),
        len_for_access_token: 40,
    }
}
