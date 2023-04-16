#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
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
        .invoke_handler(tauri::generate_handler![get_clients, insert_client])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn open_db_connection(app_handle: tauri::AppHandle) -> Option<Connection> {
    let mut path = app_handle.path_resolver().app_data_dir()?;

    path.push(DATABASE_FOLDER_NAME);
    path.push(DATABASE_FILE_NAME);

    Connection::open(path).ok()
}

#[tauri::command]
fn get_clients(app_handle: tauri::AppHandle) -> Vec<Client> {
    let conn = open_db_connection(app_handle).expect("couldnt connect to db");

    get_clients_logic(conn).expect("couldnt get clients")
}

fn get_clients_logic(conn: Connection) -> Result<Vec<Client>> {
    let mut statement = conn.prepare("SELECT id, name FROM clients")?;

    let client_iter = statement.query_map([], |row| {
        Ok(Client {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    let mapped_clients = client_iter.filter_map(|client| match client {
        Ok(client) => Some(client),
        Err(_) => None,
    });

    Ok(mapped_clients.collect())
}

#[tauri::command]
fn insert_client(app_handle: tauri::AppHandle, name: String) -> i64 {
    let conn = open_db_connection(app_handle).expect("couldnt connect to db");

    conn.execute("INSERT INTO clients (name) VALUES (?1)", [name])
        .expect("Couldn't insert client");

    conn.last_insert_rowid()
}

fn init_db(path: PathBuf) -> Result<()> {
    println!("Creating database:");

    let conn = Connection::open(path)?;

    println!("Creating clients table...");
    conn.execute(
        "CREATE TABLE clients (
            id    INTEGER PRIMARY KEY,
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
