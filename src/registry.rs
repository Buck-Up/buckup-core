use crate::Backup;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt, fs,
    path::{Path, PathBuf},
};

const REGISTRY_FILENAME: &str = ".smartsync.toml";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Registry {
    pub backups: Vec<Backup>,
}

impl Registry {
    /// add `backup` to registry
    ///
    /// ## Example
    ///
    /// ```
    /// use smartsync_core::{Backup, Registry};
    /// use std::path::Path;
    ///
    /// let mut registry = Registry::default();
    /// let backup = Backup::new("foo".to_owned(), Path::new("foo").to_path_buf());
    /// registry.add_backup(backup);
    /// ```
    pub fn add_backup(&mut self, backup: Backup) {
        self.backups.push(backup);
    }
}

impl fmt::Display for Registry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let backups = self
            .backups
            .iter()
            .map(|b| format!("{}", b))
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", backups)
    }
}

pub fn initialize_registry() -> Result<(), Box<dyn Error>> {
    let registry = load_registry()?;
    save_registry(&registry)?;
    let contents = toml::to_string_pretty(&registry)?;
    println!("registry initialized at {:?}", registry_path());
    println!("{}", contents);
    Ok(())
}

pub fn load_registry() -> Result<Registry, Box<dyn Error>> {
    let f = registry_path();
    if f.exists() {
        let contents = fs::read_to_string(f)?;
        let registry = toml::from_str(&contents)?;
        Ok(registry)
    } else {
        Ok(Registry::default())
    }
}

pub fn save_registry(registry: &Registry) -> Result<(), Box<dyn Error>> {
    let f = registry_path();
    let contents = toml::to_string_pretty(registry)?;
    fs::write(f, contents)?;
    Ok(())
}

fn registry_path() -> PathBuf {
    let home_dir = dirs::home_dir().expect("failed to determine home directory");
    Path::new(&home_dir).join(REGISTRY_FILENAME)
}
