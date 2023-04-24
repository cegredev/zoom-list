use std::{fs, path::PathBuf};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use tauri::PathResolver;

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub path: String,
    pub divide_by_year: bool,
    pub divide_by_month: bool,
}

fn resolve_config_path(path_resolver: &PathResolver) -> anyhow::Result<PathBuf> {
    let mut path = path_resolver
        .app_config_dir()
        .context("Could not get app config dir")?;

    path.push("config.json");

    Ok(path)
}

pub fn init_config(path_resolver: &PathResolver) -> anyhow::Result<()> {
    let path = resolve_config_path(path_resolver)?;

    if path.exists() {
        return Ok(());
    }

    write_config_internal(path, Config::default())
}

pub fn read_config(path_resolver: &PathResolver) -> anyhow::Result<Config> {
    let path = resolve_config_path(path_resolver)?;

    let content = fs::read_to_string(path).expect("could not read file");

    let config: Config = serde_json::from_str(content.as_str())?;

    Ok(config)
}

pub fn write_config(path_resolver: &PathResolver, config: Config) -> anyhow::Result<()> {
    let path = resolve_config_path(path_resolver)?;

    write_config_internal(path, config)
}

fn write_config_internal(path: PathBuf, config: Config) -> anyhow::Result<()> {
    let content = serde_json::to_string_pretty(&config).context("Could not JSON-ify config")?;

    fs::write(path, content).context("Could not write config!")?;

    Ok(())
}
