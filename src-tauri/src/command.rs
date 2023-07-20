use log::{info, debug};

#[tauri::command]
pub async fn open(path: String) {
    info!("run `open` command with path: {}", path);
    open::that(path).unwrap();
}

#[tauri::command]
pub async fn nickname_is_taken(nickname: String) -> bool {
    let nickname = nickname.trim();
    info!("run `nickname_is_taken` command with nickname: {}", nickname);
    let nickname_is_taken = impl_chat::client::nickname_is_taken(nickname.to_string()).await.unwrap();
    debug!("nickname_is_taken: {}", nickname_is_taken);

    nickname_is_taken
}