pub use crate::service::grpc_intercept;
use crate::service::AuthenticationService;
pub use crate_proto::AuthenticationServer;

pub async fn get_service(secret: Vec<u8>) -> AuthenticationService {
    let db_pool = database::establish_pooled_connection().await;
    AuthenticationService { db_pool, secret }
}
