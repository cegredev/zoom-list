use std::{fs, path::PathBuf};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use tauri::PathResolver;

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    path: String,
    divide_by_year: bool,
    divide_by_month: bool,
}

pub fn init_config(path_resolver: &PathResolver) -> anyhow::Result<()> {
    let mut path = path_resolver
        .app_config_dir()
        .context("Could not get app config dir")?;

    path.push("config.json");

    if path.exists() {
        return Ok(());
    }

    let content =
        serde_json::to_string_pretty(&Config::default()).context("Could not JSON-ify config")?;

    fs::write(path, content).context("Could not write config!")?;

    Ok(())
}

pub fn read_config(path: PathBuf) -> Result<Config> {
    let content = fs::read_to_string(path).expect("could not read file");

    let config: Config = serde_json::from_str(content.as_str())?;

    Ok(config)
}
