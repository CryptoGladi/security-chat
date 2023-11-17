use std::{sync::Mutex, collections::HashSet};
use super::SecurityChatService;
use log::info;
use rand::{distributions::Alphanumeric, Rng};
use tonic::Request;
use once_cell::sync::Lazy;

static STORAGE_AUTH_TOKEN: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::default()));
// TODO USE DB

pub fn intercept(
    request: Request<()>,
) -> Result<Request<()>, tonic::Status> {
    let token = match request.metadata().get("authorization") {
        Some(token) => token.to_str(),
        None => {
            info!(
                "Token not found for client for request: {:?}",
                request.get_ref()
            );

            return Err(tonic::Status::invalid_argument(
                "Token not found in metadata",
            ));
        }
    };

    let Ok(token) = token else {
        return Err(tonic::Status::cancelled("error to_str()"));
    };

    if !(service.storage_auth_tokens.lock().unwrap().contains(token)) {
        return Err(tonic::Status::unauthenticated("Your token is not valid"));
    }

    Ok(request)
}

impl SecurityChatService {
    fn get_token(&mut self) -> String {
        let new_token: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if !self
            .storage_auth_tokens
            .lock()
            .unwrap()
            .insert(new_token.clone())
        {
            return self.get_token();
        }

        new_token
    }
}
