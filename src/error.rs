use std::{fmt, io, path::PathBuf};

use crate::util::pathbuf_to_str;

pub enum BackupError {
    NotADirectory {
        path: PathBuf,
        path_type: PathType,
    },
    DirectoryNotFound {
        path: PathBuf,
        path_type: PathType,
    },
    BackupNotFound {
        backup_name: String,
    },
    DeviceNotFound {
        device_name: String,
        backup_name: String,
    },
    IO(io::Error),
    DeToml(toml::de::Error),
    SerToml(toml::ser::Error),
}

impl fmt::Debug for BackupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotADirectory { path, path_type } => {
                write!(f, "{path_type} {} is not a directory", pathbuf_to_str(path))
            }
            Self::DirectoryNotFound { path, path_type } => {
                write!(f, "{path_type} {} not found", pathbuf_to_str(path))
            }
            Self::BackupNotFound { backup_name } => {
                write!(f, "backup '{backup_name}' not found")
            }
            Self::DeviceNotFound {
                device_name,
                backup_name,
            } => {
                write!(
                    f,
                    "device '{device_name}' not found in backup '{backup_name}'"
                )
            }
            Self::IO(err) => {
                write!(f, "{:?}", err)
            }
            Self::DeToml(err) => {
                write!(f, "{:?}", err)
            }
            Self::SerToml(err) => {
                write!(f, "{:?}", err)
            }
        }
    }
}

pub enum PathType {
    Source,
    Dest,
}

impl fmt::Display for PathType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathType::Source => {
                write!(f, "source")
            }
            PathType::Dest => {
                write!(f, "dest")
            }
        }
    }
}

impl From<io::Error> for Box<BackupError> {
    fn from(value: io::Error) -> Self {
        Box::new(BackupError::IO(value))
    }
}

impl From<toml::de::Error> for Box<BackupError> {
    fn from(value: toml::de::Error) -> Self {
        Box::new(BackupError::DeToml(value))
    }
}

impl From<toml::ser::Error> for Box<BackupError> {
    fn from(value: toml::ser::Error) -> Self {
        Box::new(BackupError::SerToml(value))
    }
}
