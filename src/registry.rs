use crate::util::pathbuf_to_str;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt, fs,
    path::{Path, PathBuf},
};

const REGISTRY_FILENAME: &str = ".smartsync.toml";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Registry {
    pub backups: Vec<PathBuf>,
}

impl Registry {
    /// add `backup` to registry
    ///
    /// ## Example
    ///
    /// ```
    /// use smartsync_core::Registry;
    /// use std::path::Path;
    ///
    /// let mut registry = Registry::default();
    /// registry.add_backup(Path::new("foo").to_path_buf());
    /// ```
    pub fn add_backup(&mut self, backup: PathBuf) {
        self.backups.push(backup);
    }

    pub fn backups_str(&self) -> Vec<String> {
        self.backups.iter().map(|s| pathbuf_to_str(s)).collect()
    }
}

impl fmt::Display for Registry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let backups = self
            .backups
            .iter()
            .map(|b| {
                let offline = if b.exists() { "" } else { " (offline)" };
                format!("{}{}", pathbuf_to_str(b), offline)
            })
            .collect::<Vec<String>>()
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
