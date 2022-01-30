use crate::DeviceConfig;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

const CONFIG_FILENAME: &str = "smartsync.toml";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub devices: Vec<DeviceConfig>,
}

impl Config {
    /// add `device` to config
    ///
    /// ## Example
    ///
    /// ```
    /// use smartsync_core::{Config, DeviceConfig};
    ///
    /// let mut config = Config::default();
    /// let device = DeviceConfig::default();
    /// config.add_device(device);
    /// ```
    pub fn add_device(&mut self, device: DeviceConfig) {
        self.devices.push(device);
    }
}

pub fn initialize_config(directory: &Path) -> Result<(), Box<dyn Error>> {
    let config = load_config(directory)?;
    save_config(&config, directory)?;
    let contents = toml::to_string_pretty(&config)?;
    println!("config initialized at {:?}", config_path(directory));
    println!("{}", contents);
    Ok(())
}

pub fn load_config(directory: &Path) -> Result<Config, Box<dyn Error>> {
    let f = config_path(directory);
    if f.exists() {
        let contents = fs::read_to_string(f)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    } else {
        Ok(Config::default())
    }
}

pub fn save_config(config: &Config, directory: &Path) -> Result<(), Box<dyn Error>> {
    let f = config_path(directory);
    let contents = toml::to_string_pretty(config)?;
    fs::write(f, contents)?;
    Ok(())
}

fn config_path(directory: &Path) -> PathBuf {
    directory.join(CONFIG_FILENAME)
}
