use std::{
    fs,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::BackupError;

const CONFIG_FILENAME: &str = ".smartsync.toml";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub backups: Vec<Backup>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Backup {
    pub name: String,
    pub source_paths: Vec<PathBuf>,
    pub destinations: Vec<Destination>,
    pub last_run: Option<DateTime<Utc>>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Destination {
    pub device_name: String,
    pub path: PathBuf,
}

pub fn initialize_config() -> Result<(), Box<BackupError>> {
    let config = load_config()?;
    save_config(&config)?;
    let contents = toml::to_string_pretty(&config)?;
    println!("config initialized at {:?}", config_path());
    println!("{}", contents);
    Ok(())
}

pub fn load_config() -> Result<Config, Box<BackupError>> {
    let f = config_path();
    if f.exists() {
        let contents = fs::read_to_string(f)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    } else {
        Ok(Config::default())
    }
}

pub fn save_config(config: &Config) -> Result<(), Box<BackupError>> {
    let f = config_path();
    let contents = toml::to_string_pretty(config)?;
    fs::write(f, contents)?;
    Ok(())
}

fn config_path() -> PathBuf {
    let home_dir = dirs::home_dir().expect("failed to determine home directory");
    Path::new(&home_dir).join(CONFIG_FILENAME)
}
