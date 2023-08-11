//use crate::path;
use log::*;
//use high_level::prelude::*;

#[tauri::command]
pub async fn open(path: String) {
    info!("run `open` command with path: {}", path);
    open::that(path).unwrap()
}

#[tauri::command]
pub async fn nickname_is_taken(nickname: String) -> bool {
    let nickname = nickname.trim();
    info!(
        "run `nickname_is_taken` command with nickname: {}",
        nickname
    );

    //let nickname_is_taken = lower_level::client::nickname_is_taken(nickname)
    //    .await
    //    .kill_unwrap();
    //debug!("nickname_is_taken: {}", nickname_is_taken);

    //nickname_is_taken
    false
}

#[tauri::command]
pub async fn registration(nickname: String) {
    let nickname = nickname.trim().to_string();
    info!("run `registration` command with nickname: {}", nickname);

    //let client = Client::registration(&nickname).await.kill_unwrap();
    //client_save(&client.data, path::get_app_folder().join("config.json")).kill_unwrap();
}
