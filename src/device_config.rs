use crate::FileSync;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeviceConfig {
    pub name: String,
    pub files: Vec<FileSync>,
    pub last_backup: Option<DateTime<Utc>>,
}

impl DeviceConfig {
    pub fn new(name: String) -> DeviceConfig {
        DeviceConfig {
            name,
            ..Default::default()
        }
    }

    /// add `backup` to config
    ///
    /// ## Example
    ///
    /// ```
    /// use smartsync_core::{DeviceConfig, FileSync};
    /// use std::path::Path;
    ///
    /// let mut device_config = DeviceConfig::default();
    /// let sync = FileSync::new("foo".to_string(), Path::new("foo").to_path_buf());
    /// device_config.add_sync(sync);
    /// ```
    pub fn add_sync(&mut self, sync: FileSync) {
        self.files.push(sync);
    }
}

impl fmt::Display for DeviceConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let files = self
            .files
            .iter()
            .map(|f| format!("{}", f))
            .collect::<Vec<String>>()
            .join("\n----\n");
        let last_backup = if let Some(timestamp) = self.last_backup {
            format!("{}", timestamp)
        } else {
            "N/A".to_string()
        };
        let descriptions = vec![
            format!("Device: {}", self.name),
            String::new(),
            files,
            String::new(),
            format!("Last backup: {}", last_backup),
        ]
        .join("\n");
        write!(f, "{}", descriptions)
    }
}
