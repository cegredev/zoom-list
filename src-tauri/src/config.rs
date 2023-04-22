use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct Config {
    path: String,
    divide_by_year: bool,
    divide_by_month: bool,
}

pub fn read_config(path: PathBuf) -> Result<Config> {
    println!("{:?}", path);

    let content = fs::read_to_string(path).expect("could not read file");

    let config: Config = serde_json::from_str(content.as_str())?;

    Ok(config)
}
