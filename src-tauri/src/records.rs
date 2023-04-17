use std::{collections::HashMap, fmt::Debug, fs};

use chrono::{Days, NaiveDate, NaiveDateTime};
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

use crate::clients::{get_clients, insert_client, Client};

const DATE_FORMAT: &'static str = "%d.%m.%Y %H:%M:%S";
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

pub fn get_record_counts_month(conn: Connection, mut year: i32, mut month: u32) -> Vec<u32> {
    let mut statement = conn
        .prepare("SELECT ((start_ms - ?1) / (1000 * 60 * 60 * 24)) AS day FROM records WHERE start_ms >= ?1 AND start_ms < ?2")
        .expect("Couldnt prepare statement");

    let start = NaiveDateTime::parse_from_str(
        format!("01.{}.{} 00:00:00", month, year).as_str(),
        DATE_FORMAT,
    )
    .expect("Could not parse date");

    if month == 12 {
        year += 1;
        month = 1;
    }

    let end = NaiveDateTime::parse_from_str(
        format!("01.{}.{} 00:00:00", month + 1, year).as_str(),
        DATE_FORMAT,
    )
    .expect("Could not parse date");

    let result = statement
        .query_map(
            params![start.timestamp_millis(), end.timestamp_millis()],
            |row| {
                let day: usize = row.get(0)?;
                Ok(day)
            },
        )
        .expect("Could not query");

    let mut output: Vec<u32> = vec![
        0;
        end.signed_duration_since(start)
            .num_days()
            .try_into()
            .unwrap()
    ];

    for x in result {
        if let Ok(day) = x {
            output[day] += 1;
        }
    }

    output
}

pub fn delete_records_on(conn: Connection, year: i32, month: u32, date: u32) {
    let start = NaiveDateTime::parse_from_str(
        format!("{}.{month}.{year} 00:00:00", date + 1).as_str(),
        DATE_FORMAT,
    )
    .expect("could not parse date");
    let end = start.checked_add_days(Days::new(1)).unwrap();

    println!("{:?} {:?}", start, end);

    let start_ms = start.timestamp_millis();
    let end_ms = end.timestamp_millis();

    conn.execute(
        "DELETE FROM records WHERE start_ms >= ?1 AND start_ms < ?2",
        params![start_ms, end_ms],
    )
    .expect("could not execute");
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
            let date = NaiveDateTime::parse_from_str(&record.start, DATE_FORMAT)
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
