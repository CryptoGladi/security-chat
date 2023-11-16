use super::SecurityChatService;
use log::info;
use rand::{distributions::Alphanumeric, Rng};
use tonic::Request;

pub fn intercept(request: Request<()>) -> Result<Request<()>, tonic::Status> {
    let token = match request.metadata().get("authorization") {
        Some(token) => token.to_str(),
        None => {
            info!(
                "Token not found for client for request: {:?}",
                request.get_ref()
            );
            return Err(tonic::Status::unauthenticated("Token not found"));
        }
    };

    Ok(request)
}

impl SecurityChatService {
    fn get_token(&mut self) -> String {
        let new_token = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if !self.storage_auth_tokens.lock().await.insert(new_token) {
            return self.get_token();
        }

        new_token
    }
}
