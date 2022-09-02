use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::Deserialize;
use xdg;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub redmine_api_key: String,
    pub redmine_url: String,
}

fn get_config_path() -> Result<PathBuf> {
    if let Ok(config_path) = std::env::var("REMISS_CONFIG_PATH") {
        Ok(config_path.into())
    } else {
        xdg::BaseDirectories::with_prefix("remiss")?
            .place_config_file("remiss.config")
            .map_err(anyhow::Error::from)
    }
}

pub fn get_config() -> Result<Config> {
    let config_path = get_config_path()?;
    dotenv::from_path(&config_path)
        .with_context(|| format!("Please place a config file at {}", config_path.display()))?;

    envy::from_env::<Config>().map_err(anyhow::Error::from)
}
