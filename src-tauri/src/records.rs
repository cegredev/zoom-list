use std::{collections::HashMap, fmt::Debug, fs, hash::Hash, io};

use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize, Serializer};

use crate::clients::get_clients;

const TIME_FORMAT: &'static str = "dd.MM.yyyy HH:mm:ss";

// struct SDateTime(DateTime<Utc>);

// impl Serialize for SDateTime {
//     fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
//         serializer.serialize_u64(self.0.timestamp_millis().try_into().unwrap())
//     }
// }

// impl Debug for SDateTime {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str(self.0.format("%d%m%Y %H:%M").to_string().as_str())
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    client_id: i32,
    start: String,
    duration_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatingRecord {
    start: String,
    duration_minutes: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CSVData {
    records: Vec<Record>,
    floating_records: HashMap<String, Vec<FloatingRecord>>,
}

fn get_client_map(conn: Connection) -> Result<HashMap<String, i32>> {
    let clients = get_clients(conn)?;

    let mut map = HashMap::new();
    for client in clients {
        map.insert(client.name, client.id);
    }

    Ok(map)
}

pub fn records_from_csv(conn: Connection, path: String) -> Option<CSVData> {
    let content = fs::read_to_string(path).ok()?;

    let name_map = get_client_map(conn).ok()?;
    let mut records: Vec<Record> = vec![];
    let mut floating_records: HashMap<String, Vec<FloatingRecord>> = HashMap::new();

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

        let client_id = name_map.get(&name);
        // let start_date = DateTime::parse_from_str(start, TIME_FORMAT);
        let duration_minutes: u32 = duration.parse().expect("Duration could not be parsed");

        if let Some(client_id) = client_id {
            records.push(Record {
                client_id: client_id.clone(),
                start,
                duration_minutes,
            });
        } else {
            let vec = floating_records.get(&name);
            let mut real_vec: Vec<FloatingRecord>;
            if let None = vec {
                real_vec = vec![];
            } else {
                real_vec = vec.unwrap().to_vec();
            }

            real_vec.push(FloatingRecord {
                start,
                duration_minutes,
            });

            floating_records.insert(name, real_vec);
        }
    }

    Some(CSVData {
        records,
        floating_records,
    })
}
