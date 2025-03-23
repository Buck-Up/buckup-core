# Smart Sync Design

## per-device registry

default location: `~/.smartsync.toml`

```rs
// contents of per-device backup registry
// location of backups (e.g. folder on external drive)
struct Registry {
    backups: Vec<Backup>,
}

struct Backup {
    pub name: String,
    pub path: PathBuf,
}
```

a backup is "connected" if the path resolves to a directory

if a backup is connected, it can be "opened" to view/edit its config or perform
a backup, etc.

## backup location configuration

default location: `/path/to/location/smartsync.toml`

(e.g. on an external drive)

```rs
// contents of backup location config
// new devices are added as different devices run Smart Sync
// and utilize the same backup location
struct Config {
    devices: Vec<DeviceConfig>,
}

// information about a backup device (e.g. Linux Laptop, Macbook)
struct DeviceConfig {
    name: String,
    last_backup: Option<DateTime<Utc>>,
    files: Vec<FileSync>,
}

// an individual entry for backing up file(s)/folder(s) (e.g. Pictures)
struct FileSync {
    name: String,
    sources: Vec<PathBuf>,
    dest: PathBuf,
}
```
