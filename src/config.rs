use crate::Backup;
use serde::{Deserialize, Serialize};
use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

const CONFIG_FILENAME: &str = ".strongbox.toml";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub backups: Vec<Backup>,
}

impl Config {
    pub fn add_backup(&mut self, backup: Backup) {
        self.backups.push(backup);
    }

}

pub fn initialize_config() -> Result<(), Box<dyn Error>> {
    let config = load_config()?;
    save_config(&config)?;
    let contents = toml::to_string_pretty(&config)?;
    println!("config initialized at {:?}", config_path());
    println!("{}", contents);
    Ok(())
}

pub fn load_config() -> Result<Config, Box<dyn Error>> {
    let f = config_path();
    if f.exists() {
        let contents = fs::read_to_string(f)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    } else {
        Ok(Config::default())
    }
}

pub fn save_config(config: &Config) -> Result<(), Box<dyn Error>> {
    let f = config_path();
    let contents = toml::to_string_pretty(config)?;
    fs::write(f, contents)?;
    Ok(())
}

fn config_path() -> PathBuf {
    let home_dir = env::var("HOME").unwrap();
    Path::new(&home_dir).join(CONFIG_FILENAME)
}
