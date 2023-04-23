#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod clients;
mod config;
mod db;
mod records;

use clients::Client;
use config::Config;
use db::{init_db, open_db_connection, DATABASE_FILE_NAME, DATABASE_FOLDER_NAME};
use records::ClientRecords;
use std::fs;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            db::init_db_files(&app.path_resolver())?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_clients,
            insert_client,
            delete_client,
            parse_csv,
            submit_records,
            get_record_counts_month,
            delete_records_on,
            read_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn read_config(app_handle: tauri::AppHandle) -> Config {
    let mut path = app_handle
        .path_resolver()
        .app_config_dir()
        .expect("could not get config dir");

    path.push("config.json");

    config::read_config(path).expect("Could not read config")
}

#[tauri::command]
fn get_clients(app_handle: tauri::AppHandle) -> Vec<Client> {
    let conn = open_db_connection(app_handle).expect("couldnt connect to db");

    clients::get_clients(conn).expect("couldnt get clients")
}

#[tauri::command]
fn insert_client(app_handle: tauri::AppHandle, name: String) -> i64 {
    let conn = open_db_connection(app_handle).expect("couldnt connect to db");

    clients::insert_client(&conn, name)
}

#[tauri::command]
fn delete_client(app_handle: tauri::AppHandle, id: i32) {
    let conn = open_db_connection(app_handle).expect("couldnt connect to db");

    clients::delete_client(conn, id)
}

#[tauri::command]
fn parse_csv(app_handle: tauri::AppHandle, path: String) -> Vec<ClientRecords> {
    let conn = open_db_connection(app_handle).expect("couldnt connect to db");

    records::read_client_records(conn, path).expect("could not parse")
}

#[tauri::command]
fn submit_records(app_handle: tauri::AppHandle, records: Vec<ClientRecords>) {
    let conn = open_db_connection(app_handle).expect("couldnt connect to db");

    records::submit_records(conn, records).expect("could not submit")
}

#[tauri::command]
fn get_record_counts_month(app_handle: tauri::AppHandle, year: i32, month: u32) -> Vec<u32> {
    let conn = open_db_connection(app_handle).expect("couldnt connect to db");

    records::get_record_counts_month(conn, year, month)
}

#[tauri::command]
fn delete_records_on(app_handle: tauri::AppHandle, year: i32, month: u32, day: u32) {
    let conn = open_db_connection(app_handle).expect("couldnt connect to db");

    records::delete_records_on(conn, year, month, day)
}
