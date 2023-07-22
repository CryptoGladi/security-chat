use crate::path;
use impl_chat::prelude::{client_save, Client};
use log::{debug, error, info};

pub trait KillUnwrap<T> {
    fn kill_unwrap(self) -> T;
}

impl<T, E: std::fmt::Debug> KillUnwrap<T> for Result<T, E> {
    fn kill_unwrap(self) -> T {
        match self {
            Ok(o) => o,
            Err(e) => unwrap_failed("called `Result::unwrap()` on an `Err` value", &e),
        }
    }
}

pub fn unwrap_failed(msg: &str, error: &dyn std::fmt::Debug) -> ! {
    error!("{msg}: {error:?}");
    std::process::exit(1)
}

#[tauri::command]
pub async fn open(path: String) {
    info!("run `open` command with path: {}", path);
    open::that(path).kill_unwrap()
}

#[tauri::command]
pub async fn nickname_is_taken(nickname: String) -> bool {
    let nickname = nickname.trim();
    info!(
        "run `nickname_is_taken` command with nickname: {}",
        nickname
    );

    let nickname_is_taken = impl_chat::client::nickname_is_taken(nickname)
        .await
        .kill_unwrap();
    debug!("nickname_is_taken: {}", nickname_is_taken);

    nickname_is_taken
}

#[tauri::command]
pub async fn registration(nickname: String) {
    let nickname = nickname.trim().to_string();
    info!("run `registration` command with nickname: {}", nickname);

    let client = Client::registration(&nickname).await.kill_unwrap();
    client_save(&client.data, path::get_app_folder().join("config.json")).kill_unwrap();
}
