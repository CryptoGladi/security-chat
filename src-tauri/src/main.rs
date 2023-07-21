// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::warn;

pub mod command;
pub mod logger;
pub mod path;

fn main() {
    logger::init_logger();
    warn!("running chat...");

    if !crate::path::get_app_folder().is_dir() {
        std::fs::create_dir_all(crate::path::get_app_folder()).unwrap();
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::open,
            command::nickname_is_taken,
            command::registration
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
