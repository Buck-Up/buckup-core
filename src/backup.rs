use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Backup {
    pub name: String,
    pub sources: Vec<PathBuf>,
    pub dest: PathBuf,
}

impl Backup {
    pub fn new(name: String, dest: PathBuf) -> Backup {
        Backup {
            name,
            sources: Vec::new(),
            dest,
        }
    }

    pub fn add_source(&mut self, source: PathBuf) {
        self.sources.push(source);
    }
}
