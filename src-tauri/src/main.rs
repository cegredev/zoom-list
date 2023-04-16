#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rusqlite::{Connection, Result};
use std::{fs, path::PathBuf};

#[derive(Debug)]
struct Client {
    id: u32,
    name: String,
}

const DATABASE_FOLDER_NAME: &'static str = "databases";
const DATABASE_FILE_NAME: &'static str = "db.db3";

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
        // .invoke_handler(tauri::generate_handler![init_db])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// #[tauri::command]
// fn init_db(app_handle: tauri::AppHandle) {
//     let mut path = app_handle.path_resolver().app_data_dir().unwrap();
//     path.push(DATABASE_FOLDER_NAME);
//     path.push("db.db3");

//     println!("path: {:?}", path);

//     init_db_logic(path).expect("Init failed");
// }

fn init_db(path: PathBuf) -> Result<()> {
    println!("Creating database:");

    let conn = Connection::open(path)?;

    println!("Creating clients table...");
    conn.execute(
        "CREATE TABLE clients (
            id    INTEGER UNSIGNED PRIMARY KEY,
            name  TEXT NOT NULL
            )",
        (), // empty list of pa rameters.
    )?;
    println!("Success!");

    println!("Creating records table...");
    conn.execute(
        "CREATE TABLE records (
            client_id   INTEGER UNSIGNED,
            start_ms    INTEGER UNSIGNED,
            duration_s  INTEGER UNSIGNED,
            FOREIGN KEY(client_id) REFERENCES clients(id)
        )",
        (),
    )?;
    println!("Success!");

    Ok(())
}
