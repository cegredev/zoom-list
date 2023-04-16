#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod clients;
mod db;
mod records;

use clients::Client;
use db::{init_db, open_db_connection, DATABASE_FILE_NAME, DATABASE_FOLDER_NAME};
use std::fs;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let mut dir_path = app
                .path_resolver()
                .app_data_dir()
                .ok_or("AppDataDir not found")?;
            dir_path.push(DATABASE_FOLDER_NAME);

            let mut db_path = dir_path.clone();
            db_path.push(DATABASE_FILE_NAME);

            if !db_path.exists() {
                fs::create_dir_all(dir_path.clone())?;

                init_db(db_path)?;
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_clients, insert_client])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_clients(app_handle: tauri::AppHandle) -> Vec<Client> {
    let conn = open_db_connection(app_handle).expect("couldnt connect to db");

    clients::get_clients(conn).expect("couldnt get clients")
}

#[tauri::command]
fn insert_client(app_handle: tauri::AppHandle, name: String) -> i64 {
    let conn = open_db_connection(app_handle).expect("couldnt connect to db");

    clients::insert_client(conn, name)
}
