use crate::Backup;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub backups: Vec<Backup>,
}

impl Config {
    pub fn add_backup(&mut self, backup: Backup) {
        self.backups.push(backup);
    }
}
