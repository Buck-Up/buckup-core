use serde::{Deserialize, Serialize};
use std::{fmt, path::PathBuf};

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

impl fmt::Display for Backup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sources = self
            .sources
            .iter()
            .map(|s| format!("- {:?}", s))
            .collect::<Vec<String>>()
            .join("\n");
        let descriptions = vec![
            format!("Name: {}", self.name),
            format!("Sources:\n{}", sources),
            format!("Dest: {:?}", self.dest),
        ]
        .join("\n");
        write!(f, "{}", descriptions)
    }
}
