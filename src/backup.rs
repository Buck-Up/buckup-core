use serde::{Deserialize, Serialize};
use std::{
    fmt,
    path::{Path, PathBuf},
};

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

    /// add `source` path to backup
    ///
    /// ## Example
    ///
    /// ```
    /// use smartsync_core::Backup;
    /// use std::path::Path;
    ///
    /// let mut backup = Backup::new("foo".to_string(), Path::new("foo").to_path_buf());
    /// backup.add_source(Path::new("bar").to_path_buf());
    /// ```
    pub fn add_source(&mut self, source: PathBuf) {
        self.sources.push(source);
    }

    pub fn sources_str(&self) -> Vec<String> {
        self.sources.iter().map(|s| pathbuf_to_str(s)).collect()
    }

    pub fn dest_str(&self) -> String {
        pathbuf_to_str(&self.dest)
    }
}

fn pathbuf_to_str(p: &Path) -> String {
    match p.to_str() {
        Some(s) => s.to_string(),
        None => format!("{:?}", p),
    }
}

impl fmt::Display for Backup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sources = self
            .sources_str()
            .iter()
            .map(|s| format!("- {}", s))
            .collect::<Vec<String>>()
            .join("\n");
        let descriptions = vec![
            format!("Name: {}", self.name),
            format!("Sources:\n{}", sources),
            format!("Dest: {}", self.dest_str()),
        ]
        .join("\n");
        write!(f, "{}", descriptions)
    }
}
