// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lock::Lock;
use log::warn;

pub mod command;
pub mod global;
pub mod lock;
pub mod logger;
pub mod path;

fn main() {
    let panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        panic_hook(info);

        #[allow(clippy::exit)]
        std::process::exit(1);
    }));

    logger::init_logger();
    warn!("running chat...");

    if !crate::path::get_app_folder().is_dir() {
        std::fs::create_dir_all(crate::path::get_app_folder()).unwrap();
    }

    let _lock = Lock::new(crate::path::get_app_folder().join("lockfile"));

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::open,
            command::nickname_is_taken,
            command::registration,
            command::have_account,
            command::fuzzy_search_vim_command,
            command::run_command,
            command::get_all_users,
            command::change_window_for_main_page,
            command::get_messages_for_user,
            command::get_nickname,
            command::send_message,
            command::get_cryptos_for_accept,
            command::add_crypto
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
