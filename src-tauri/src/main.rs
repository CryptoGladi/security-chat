// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::check_version::smart_check_version;
use fcore::prelude::*;
use log::{debug, warn};

pub mod check_version;
pub mod command;
pub mod global;
pub mod logger;
pub mod path;

fn main() {
    color_backtrace::install();
    let panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        panic_hook(info);

        #[allow(clippy::exit)]
        std::process::exit(1);
    }));

    dotenv::dotenv().ok();
    logger::init().expect("logger init");
    warn!("running chat...");
    debug!("env server address: {}", get_env_var("ADDRESS_SERVER"));

    tauri::async_runtime::spawn(async {
        assert!(
            (smart_check_version().await),
            "you have old version app. Please, update your app"
        );
    });

    if !crate::path::get_app_folder().is_dir() {
        std::fs::create_dir_all(crate::path::get_app_folder()).unwrap();
    }

    let _lock = Lock::new(crate::path::get_app_folder().join("lockfile"));

    let tauri_builder = tauri::Builder::default().invoke_handler(tauri::generate_handler![
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
        command::send_crypto,
        command::get_cryptos_for_accept,
        command::add_crypto,
        command::delete_crypto,
        command::get_random_nickname,
        command::get_version_app,
        command::get_order_adding_crypto
    ]);

    #[allow(clippy::disallowed_types)]
    {
        tauri_builder
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}
