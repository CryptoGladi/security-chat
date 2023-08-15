// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::warn;

pub mod command;
pub mod global;
pub mod logger;
pub mod path;

fn main() {
    let panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        panic_hook(info);
        std::process::exit(1);
    }));

    logger::init_logger();
    warn!("running chat...");

    if !crate::path::get_app_folder().is_dir() {
        std::fs::create_dir_all(crate::path::get_app_folder()).unwrap();
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::open,
            command::nickname_is_taken,
            command::registration,
            command::have_account,
            command::fuzzy_search_vim_command,
            command::run_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
