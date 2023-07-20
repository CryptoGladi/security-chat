use log::info;

#[tauri::command]
pub async fn open(path: String) {
    info!("run `open` command with path: {}", path);
    open::that(path).unwrap();
}

#[tauri::command]
pub async fn nickname_is_taken(nickname: String) {
    info!("run `nickname_is_taken` command with nickname: {}", nickname);
    impl_chat::client::nickname_is_taken(nickname).await.unwrap();
}