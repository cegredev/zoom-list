use std::{collections::HashMap, fmt::Debug, fs, hash::Hash, io};

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize, Serializer};

use crate::clients::{get_clients, insert_client, Client};

const TIME_FORMAT: &'static str = "dd.MM.yyyy HH:mm:ss";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    start: String,
    duration_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientRecords {
    id: Option<i32>,
    name: String,
    records: Vec<Record>,
}

fn get_client_map(conn: Connection) -> Result<HashMap<String, Client>> {
    let clients = get_clients(conn)?;

    let mut map = HashMap::new();
    for client in clients {
        map.insert(client.name.clone(), client.clone());
    }

    Ok(map)
}

pub fn read_client_records(conn: Connection, path: String) -> Option<Vec<ClientRecords>> {
    let clients_map = get_client_map(conn).ok()?;
    let content = fs::read_to_string(path).ok()?;

    let mut records_map: HashMap<String, Vec<Record>> = HashMap::new();

    let mut first = true;
    for line in content.split("\n") {
        if first {
            first = false;
            continue;
        }

        let line = line.trim_end();

        let values: Vec<&str> = line.split(",").collect();
        if values.len() < 5 {
            continue;
        }

        let name = values[0].to_owned();
        let start = values[2].to_owned();
        let duration = values[4];

        let duration_minutes: u32 = duration.parse().expect("Duration could not be parsed");

        let vec = records_map.get(&name);
        let mut real_vec: Vec<Record>;
        if let None = vec {
            real_vec = vec![];
        } else {
            real_vec = vec.unwrap().to_vec();
        }

        real_vec.push(Record {
            start,
            duration_minutes,
        });

        records_map.insert(name, real_vec);
    }

    let mut result: Vec<ClientRecords> = vec![];

    for (name, records) in records_map {
        let client = clients_map.get(&name);

        if let Some(client) = client {
            result.push(ClientRecords {
                id: Some(client.id),
                name,
                records,
            });
        } else {
            result.push(ClientRecords {
                id: None,
                name,
                records,
            })
        }
    }

    Some(result)
}

pub fn submit_records(conn: Connection, records: Vec<ClientRecords>) -> Option<()> {
    for client_records in records {
        let client_id = client_records.id.unwrap_or(
            insert_client(&conn, client_records.name)
                .try_into()
                .unwrap(),
        );

        for record in client_records.records {
            let date = NaiveDateTime::parse_from_str(&record.start, "%d.%m.%Y %H:%M:%S")
                .expect("could not parse data");

            conn.execute(
                "INSERT INTO records (client_id, start_ms, duration_m) VALUES (?1, ?2, ?3)",
                params![client_id, date.timestamp_millis(), record.duration_minutes,],
            )
            .expect("Could not insert record");
        }
    }

    Some(())
}
