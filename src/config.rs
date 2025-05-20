use anyhow::anyhow;
use anyhow::{Context, Result};
use ini::Ini;
use std::path::PathBuf;

// Find path to home directory for reading config file
pub fn fetch_config_path() -> Result<PathBuf> {
    Ok(home::home_dir()
        .ok_or_else(|| anyhow!("Could not determine user's home directory"))?
        .join(".config")
        .join("mini-weather-report")
        .join("config.ini"))
}

// Read config file and load configuration
pub fn fetch_config() -> Result<(String, String, String), anyhow::Error> {
    let config_path = fetch_config_path()?;
    let config = Ini::load_from_file(&config_path)
        .with_context(|| format!("Failed to load config file: {}", config_path.display()))?;
    let key = config.get_from(Some("main"), "key").unwrap().to_string();
    let city = config
        .get_from(Some("main"), "city")
        .unwrap_or_default()
        .to_string();
    let country = config
        .get_from(Some("main"), "country")
        .unwrap_or_default()
        .to_string();
    Ok((key, city, country))
}
