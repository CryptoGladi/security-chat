use crate::global;
use high_level::{
    client::{
        impl_message::{Message, MessageInfo},
        storage_crypto::Nickname,
    },
    prelude::*,
};
use log::*;
use crate::check_version::smart_check_version;
use tauri::{Manager, Runtime, Size};

pub async fn load_client(app: tauri::AppHandle) {
    let mut client = Client::load(global::CLIENT_INIT_CONFIG.clone())
        .await
        .unwrap();
    client.update_cryptos().await.unwrap();
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
                NewMessage(message) => {
                    app.emit_all("new-message", message).unwrap();
                }
                NewSentAcceptAesKey(_) => {}
                NewAcceptAesKey(_key) => {
                    global::LOADED_CLIENT
                        .write()
                        .await
                        .as_mut()
                        .unwrap()
                        .update_cryptos()
                        .await
                        .unwrap();
                    // TODO
                    // app.emit_all("new-accept-aes-key", ()).unwrap();
                }
            }

            global::LOADED_CLIENT
                .read()
                .await
                .as_ref()
                .unwrap()
                .save()
                .unwrap();
        }
    });
}

#[tauri::command]
pub async fn open(path: String) {
    info!("run `open` command with path: {}", path);
    open::that(path).unwrap();
}

#[tauri::command]
pub async fn have_account(app: tauri::AppHandle) -> bool {
    let have_account = Client::have_account(&global::CLIENT_INIT_CONFIG).unwrap();
    info!("run `have account`: {}", have_account);

    if have_account {
        load_client(app).await;
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
pub async fn registration(app: tauri::AppHandle, nickname: String) {
    let nickname = nickname.trim().to_string();
    info!("run `registration` command with nickname: {}", nickname);

    let client = Client::registration(&nickname, global::CLIENT_INIT_CONFIG.clone())
        .await
        .unwrap();
    client.save().unwrap();

    drop(client);
    load_client(app).await;
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

    debug!("get_all_users: {:?}", users);
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
    trace!("run `fuzzy_search_vim_command`: {:?}", result);

    result
}

#[tauri::command]
pub async fn change_window_for_main_page<R: Runtime>(window: tauri::Window<R>) {
    info!("run `change_window_for_main_page`");
    window
        .set_size(Size::Physical(tauri::PhysicalSize::new(1000, 1000)))
        .unwrap();
    // TODO BUG
}

#[tauri::command]
pub async fn run_command(command: String) {
    let mut client = global::LOADED_CLIENT.write().await;

    if let Err(error) = global::VIM_RUNNER
        .lock()
        .await
        .run(client.as_mut().unwrap(), &command)
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
    global::LOADED_CLIENT
        .write()
        .await
        .as_mut()
        .unwrap()
        .get_messages_for_user(Nickname(nickname_from), 1_000)
        .await
        .unwrap()
}

#[tauri::command]
pub async fn get_nickname() -> String {
    global::LOADED_CLIENT
        .read()
        .await
        .as_ref()
        .unwrap()
        .get_nickname()
        .0
}

#[tauri::command]
pub async fn send_message(nickname: String, message: String) {
    global::LOADED_CLIENT
        .write()
        .await
        .as_mut()
        .unwrap()
        .send_message(
            Nickname(nickname),
            Message {
                text: message,
                reply: None,
            },
        )
        .await
        .unwrap();
}

#[tauri::command]
pub async fn get_cryptos_for_accept() -> Vec<String> {
    debug!("run get_cryptos_for_accept");
    global::LOADED_CLIENT
        .write()
        .await
        .as_mut()
        .unwrap()
        .get_cryptos_for_accept()
        .await
        .unwrap()
        .into_iter()
        .map(|x| x.0.nickname_to)
        .collect()
}

#[tauri::command]
pub async fn add_crypto(nickname: String) {
    error!("run `add_crypto` with nickname: {}", nickname);
    let mut locked_client = global::LOADED_CLIENT.write().await;

    for key in locked_client
        .as_mut()
        .unwrap()
        .get_cryptos_for_accept()
        .await
        .unwrap()
        .iter_mut()
        .filter(|x| x.0.nickname_to == nickname)
    {
        key.accept(locked_client.as_mut().unwrap()).await.unwrap();
    }
}

#[tauri::command]
pub async fn delete_crypto(nickname: String) {
    info!("run `delete_crypto` with nickname: {}", nickname);
    let mut locked_client = global::LOADED_CLIENT.write().await;

    for key in locked_client
        .as_mut()
        .unwrap()
        .get_cryptos_for_accept()
        .await
        .unwrap()
        .iter_mut()
        .filter(|x| x.0.nickname_from == nickname)
    {
        key.delete(locked_client.as_mut().unwrap()).await.unwrap();
    }
}

#[tauri::command]
pub async fn check_version() {
    if !smart_check_version().await {
        panic!("you have old version app. Please, update your app");
    }
}