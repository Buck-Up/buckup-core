use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Backup {
    name: String,
    sources: Vec<PathBuf>,
    dest: PathBuf,
}
