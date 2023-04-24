use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    pub id: i32,
    pub name: String,
}

pub fn get_client(conn: &Connection, client_id: i32) -> Result<Client> {
    let arr: Vec<Client> = get_clients(conn)?
        .iter()
        .filter(|client| client.id == client_id)
        .map(|client| client.clone())
        .collect();

    Ok(arr[0].clone())
}

pub fn get_clients(conn: &Connection) -> Result<Vec<Client>> {
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

pub fn insert_client(conn: &Connection, name: String) -> i64 {
    conn.execute("INSERT INTO clients (name) VALUES (?1)", [name])
        .expect("Couldn't insert client");

    conn.last_insert_rowid()
}

pub fn delete_client(conn: Connection, id: i32) {
    conn.execute("DELETE FROM clients WHERE id=?1", [id])
        .expect("Coudn't delete!");
}
