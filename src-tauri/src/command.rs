use high_level::{prelude::*, client::{storage_crypto::Nickname, impl_message::MessageInfo}};
use log::*;
use tauri::{Runtime, Size};

use crate::global;

pub async fn load_client() {
    let client = Client::load(global::CLIENT_INIT_CONFIG.clone())
        .await
        .unwrap();
    *global::LOADED_CLIENT.write().await = Some(client);

    let recv = global::LOADED_CLIENT
        .write()
        .await
        .as_mut()
        .unwrap()
        .subscribe()
        .await
        .unwrap();

    tauri::async_runtime::spawn(async move {
        loop {
            let nofity = recv.recv().await.unwrap();
            info!("new nofity: {:?}", nofity);

            use Event::*;
            match nofity.event {
                NewMessage(_message) => {
                    // TODO
                }
                NewSentAcceptAesKey(mut key) => key
                    .accept(global::LOADED_CLIENT.write().await.as_mut().unwrap())
                    .await
                    .unwrap(),
                NewAcceptAesKey(_key) => {
                    global::LOADED_CLIENT
                        .write()
                        .await
                        .as_mut()
                        .unwrap()
                        .update_cryptos()
                        .await
                        .unwrap();
                }
            }

            global::LOADED_CLIENT.read().await.as_ref().unwrap().save().unwrap();
        }
    });
}

#[tauri::command]
pub async fn open(path: String) {
    info!("run `open` command with path: {}", path);
    open::that(path).unwrap()
}

#[tauri::command]
pub async fn have_account() -> bool {
    let have_account = Client::have_account(&global::CLIENT_INIT_CONFIG).unwrap();
    info!("run `have account`: {}", have_account);

    if have_account {
        load_client().await;
    }

    have_account
}

#[tauri::command]
pub async fn nickname_is_taken(nickname: String) -> bool {
    let nickname = nickname.trim();
    let nickname_is_taken = Client::nickname_is_taken(&global::CLIENT_INIT_CONFIG, nickname)
        .await
        .unwrap();
    debug!("run `nickname_is_taken`: {}", nickname_is_taken);

    nickname_is_taken
}

#[tauri::command]
pub async fn registration(nickname: String) {
    let nickname = nickname.trim().to_string();
    info!("run `registration` command with nickname: {}", nickname);

    let client = Client::registration(&nickname, global::CLIENT_INIT_CONFIG.clone())
        .await
        .unwrap();
    client.save().unwrap();

    drop(client);
    load_client().await;
}

#[tauri::command]
pub async fn get_all_users() -> Vec<String> {
    let users = global::LOADED_CLIENT
        .read()
        .await
        .as_ref()
        .unwrap()
        .get_all_users()
        .unwrap()
        .into_iter()
        .map(|x| x.0)
        .collect();

    debug!("get_all_users: {:?}", global::LOADED_CLIENT.read().await.as_ref());
    users
}

#[tauri::command]
pub async fn fuzzy_search_vim_command(command: String) -> Vec<String> {
    let result = global::VIM_RUNNER
        .lock()
        .await
        .get_fuzzy_array(&command)
        .into_iter()
        .map(|x| x.text)
        .collect();
    info!("run `fuzzy_search_vim_command`: {:?}", result);

    result
}

#[tauri::command]
pub async fn change_window_for_main_page<R: Runtime>(window: tauri::Window<R>) {
    info!("run `change_window_for_main_page`");
    window
        .set_size(Size::Physical(tauri::PhysicalSize::new(1000, 1000)))
        .unwrap();
    // TODO
}

#[tauri::command]
pub async fn run_command(command: String) {
    let mut client = Client::load(global::CLIENT_INIT_CONFIG.clone())
        .await
        .unwrap();
    if let Err(error) = global::VIM_RUNNER
        .lock()
        .await
        .run(&mut client, &command)
        .await
    {
        error!(
            "error in `run_command`: {:?}; with command: {}",
            error, command
        );
    }
}

#[tauri::command]
pub async fn get_messages_for_user(nickname_from: String) -> Vec<MessageInfo> {
    // TODO переделать сообщение!
    global::LOADED_CLIENT.write().await.as_mut().unwrap().get_messages_for_user(Nickname(nickname_from),  1_000).await.unwrap()
}

#[tauri::command]
pub async fn get_nickname() -> String {
    global::LOADED_CLIENT.read().await.as_ref().unwrap().get_nickname().0
}