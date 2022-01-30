use crate::util::pathbuf_to_str;
use serde::{Deserialize, Serialize};
use std::{fmt, path::PathBuf};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FileSync {
    pub name: String,
    pub sources: Vec<PathBuf>,
    pub dest: PathBuf,
}

impl FileSync {
    pub fn new(name: String, dest: PathBuf) -> FileSync {
        FileSync {
            name,
            sources: Vec::new(),
            dest,
        }
    }

    /// add `source` path to sync
    ///
    /// ## Example
    ///
    /// ```
    /// use smartsync_core::FileSync;
    /// use std::path::Path;
    ///
    /// let mut sync = FileSync::new("foo".to_string(), Path::new("foo").to_path_buf());
    /// sync.add_source(Path::new("bar").to_path_buf());
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

impl fmt::Display for FileSync {
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
