use std::{fs, io};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    client_id: u32,
    name: String,
    duration_minutes: u32,
}

pub fn records_from_csv(path: String) -> io::Result<Vec<Record>> {
    let content = fs::read_to_string(path)?;

    for line in content.split("\n") {
        let line = line.trim_end();

        let values: Vec<&str> = line.split(",").collect();
        let name = values[0];
    }

    Ok(vec![])
}
