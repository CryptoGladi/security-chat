use log::*;
use high_level::prelude::*;

use crate::global;

#[tauri::command]
pub async fn open(path: String) {
    info!("run `open` command with path: {}", path);
    open::that(path).unwrap()
}

#[tauri::command]
pub async fn have_account() -> bool {
    let have_account = Client::have_account(&global::CLIENT_INIT_CONFIG).unwrap();
    info!("run `have account`: {}", have_account);
  have_account
}

#[tauri::command]
pub async fn nickname_is_taken(nickname: String) -> bool {
    let nickname = nickname.trim();
    let nickname_is_taken = Client::nickname_is_taken(&global::CLIENT_INIT_CONFIG, nickname).await.unwrap();
    debug!("run `nickname_is_taken`: {}", nickname_is_taken);

    nickname_is_taken
}

#[tauri::command]
pub async fn registration(nickname: String) {
    let nickname = nickname.trim().to_string();
    info!("run `registration` command with nickname: {}", nickname);

    let client = Client::registration(&nickname, global::CLIENT_INIT_CONFIG.clone()).await.unwrap();
    client.save().unwrap();
}
