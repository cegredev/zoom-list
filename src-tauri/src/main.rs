#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{fs, path::PathBuf};

use rusqlite::{Connection, Result};
use tauri::PathResolver;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

const DATABASE_FOLDER_NAME: &'static str = "databases";

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let mut path = app
                .path_resolver()
                .app_data_dir()
                .ok_or("AppDataDir not found")?;
            path.push(DATABASE_FOLDER_NAME);

            if !path.exists() {
                fs::create_dir(path.clone())?;
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![init_db])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn init_db(app_handle: tauri::AppHandle) {
    let mut path = app_handle.path_resolver().app_data_dir().unwrap();
    path.push(DATABASE_FOLDER_NAME);
    path.push("db.db3");

    println!("path: {:?}", path);

    init_db_logic(path).expect("Init failed");
}

fn init_db_logic(path: PathBuf) -> Result<()> {
    println!("initing");

    let conn = Connection::open(path)?;

    conn.execute(
        "CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
        )",
        (), // empty list of parameters.
    )?;
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;

    println!("inserted");

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }

    println!("done");

    Ok(())
}
