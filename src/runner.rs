use chrono::Utc;

use crate::{
    config,
    error::{BackupError, PathType},
    util::pathbuf_to_str,
};
use std::{fs, path::Path};

pub fn run_backup(
    backup_name: &str,
    device_name: &str,
    dry_run: bool,
) -> Result<(), Box<BackupError>> {
    let mut config = config::load_config()?;

    let mut backup = None;
    for b in &config.backups {
        if b.name == backup_name {
            backup = Some(b);
        }
    }
    if backup.is_none() {
        return Err(Box::new(BackupError::BackupNotFound {
            backup_name: backup_name.into(),
        }));
    }
    let backup = backup.unwrap();

    let mut dest = None;
    for d in &backup.destinations {
        if d.device_name == device_name {
            dest = Some(&d.path);
        }
    }
    if dest.is_none() {
        return Err(Box::new(BackupError::DeviceNotFound {
            device_name: device_name.into(),
            backup_name: backup_name.into(),
        }));
    }
    let dest = dest.unwrap();

    if dest.exists() && !dest.is_dir() {
        return Err(Box::new(BackupError::NotADirectory {
            path: dest.clone(),
            path_type: PathType::Dest,
        }));
    }
    if !dest.is_dir() && !dry_run {
        fs::create_dir(dest)?;
    }

    for source in &backup.source_paths {
        if !source.exists() {
            return Err(Box::new(BackupError::DirectoryNotFound {
                path: source.clone(),
                path_type: PathType::Source,
            }));
        }
        if !source.is_dir() {
            return Err(Box::new(BackupError::NotADirectory {
                path: source.clone(),
                path_type: PathType::Source,
            }));
        }

        copy_files(source, dest, dry_run)?;
    }

    if !dry_run {
        for b in &mut config.backups {
            if b.name == backup_name {
                b.last_run = Some(Utc::now());
            }
        }
        config::save_config(&config)?;
    }

    Ok(())
}

fn copy_files(source_dir: &Path, dest_dir: &Path, dry_run: bool) -> Result<(), Box<BackupError>> {
    for entry in fs::read_dir(source_dir)? {
        let entry = entry.unwrap();
        let source_path = entry.path();
        let dest_path = dest_dir.join(entry.file_name());
        if source_path.is_dir() {
            if dest_path.exists() && !dest_path.is_dir() {
                return Err(Box::new(BackupError::NotADirectory {
                    path: dest_path,
                    path_type: PathType::Dest,
                }));
            }
            if !dest_path.is_dir() && !dry_run {
                fs::create_dir(&dest_path)?;
            }
            copy_files(&source_path, &dest_path, dry_run)?;
        } else if dest_path.exists() && !dest_path.is_dir() {
            // already backed up
            continue;
        } else if dry_run {
            println!(
                "would copy {} to {}",
                pathbuf_to_str(&source_path),
                pathbuf_to_str(&dest_path)
            );
        } else {
            println!(
                "copying {} to {}",
                pathbuf_to_str(&source_path),
                pathbuf_to_str(&dest_path)
            );
            fs::copy(&source_path, &dest_path)?;
        }
    }

    Ok(())
}
