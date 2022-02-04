use crate::util::pathbuf_to_str;
use serde::{Deserialize, Serialize};
use std::{fmt, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Backup {
    pub name: String,
    pub path: PathBuf,
}

impl Backup {
    pub fn new(name: String, path: PathBuf) -> Backup {
        Backup { name, path }
    }

    pub fn path_str(&self) -> String {
        pathbuf_to_str(&self.path)
    }
}

impl fmt::Display for Backup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let offline = if self.path.exists() { "" } else { " (offline)" };
        let path = format!("{}{}", self.path_str(), offline);
        write!(f, "{}\n{}", format!("Backup: {}", self.name), path)
    }
}
