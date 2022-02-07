use chrono::Utc;

use crate::{config, util::pathbuf_to_str, DeviceConfig};
use std::{error::Error, fs, path::Path};

pub fn run_backup(
    backup_path: &Path,
    device_config: &DeviceConfig,
    dry_run: bool,
) -> Result<(), Box<dyn Error>> {
    for sync in &device_config.files {
        let dest_dir = backup_path.join(&sync.dest);
        if dest_dir.exists() && !dest_dir.is_dir() {
            println!("dest path {} is not a directory", pathbuf_to_str(&dest_dir));
            continue;
        }
        if !dest_dir.is_dir() && !dry_run {
            fs::create_dir(&dest_dir)?;
        }
        for source_dir in &sync.sources {
            if !source_dir.exists() {
                println!("source dir {} not found", pathbuf_to_str(source_dir));
            } else if !source_dir.is_dir() {
                println!(
                    "source path {} is not a directory",
                    pathbuf_to_str(source_dir)
                );
            } else {
                copy_files(source_dir, &dest_dir, dry_run)?;
            }
        }
    }

    let mut config = config::load_config(backup_path)?;
    for d in &mut config.devices {
        if d.name == device_config.name {
            d.last_backup = Some(Utc::now());
            break;
        }
    }
    config::save_config(&config, backup_path)?;

    Ok(())
}

fn copy_files(source_dir: &Path, dest_dir: &Path, dry_run: bool) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(source_dir)? {
        let entry = entry.unwrap();
        let source_path = entry.path();
        let dest_path = dest_dir.join(entry.file_name());
        if source_path.is_dir() {
            if dest_path.exists() && !dest_path.is_dir() {
                println!(
                    "dest path {} is not a directory",
                    pathbuf_to_str(&dest_path)
                );
                continue;
            }
            if !dest_path.is_dir() && !dry_run {
                fs::create_dir(&dest_path)?;
            }
            copy_files(&source_path, &dest_path, dry_run)?;
        } else if dest_path.exists() && !dest_path.is_dir() {
            // already backed up
            continue;
        } else {
            println!(
                "copying {} to {}",
                pathbuf_to_str(&source_path),
                pathbuf_to_str(&dest_path)
            );
            if !dry_run {
                fs::copy(&source_path, &dest_path)?;
            }
        }
    }

    Ok(())
}
