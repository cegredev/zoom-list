use std::path::PathBuf;

use rusqlite::{Connection, Result};

pub const DATABASE_FOLDER_NAME: &'static str = "databases";
pub const DATABASE_FILE_NAME: &'static str = "db.db3";

pub fn open_db_connection(app_handle: tauri::AppHandle) -> Option<Connection> {
    let mut path = app_handle.path_resolver().app_data_dir()?;

    path.push(DATABASE_FOLDER_NAME);
    path.push(DATABASE_FILE_NAME);

    Connection::open(path).ok()
}

pub fn init_db(path: PathBuf) -> Result<()> {
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
            duration_m  INTEGER UNSIGNED,
            CONSTRAINT fk_client_in_records
                FOREIGN KEY (client_id)
                REFERENCES clients(id)
                ON DELETE CASCADE
        )",
        (),
    )?;
    println!("Success!");

    Ok(())
}
