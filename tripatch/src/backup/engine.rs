use std::path::{Path, PathBuf};

use super::{backup_file, backup_id, write_metadata};

pub struct BackupEngine;

impl BackupEngine {
    pub fn backup_configuration() -> Result<PathBuf, String> {
        let src = Path::new("/root/.homeassistant/configuration.yaml");

        let id = backup_id();

        let backup_dir = PathBuf::from(format!("/root/.trios/backup/{}", id));

        let dst = backup_dir.join("configuration.yaml");

        backup_file(src, &dst)?;

        write_metadata(
            &backup_dir,
            &id,
            "usb",
            src.to_string_lossy().as_ref(),
            "configuration.yaml",
        )?;

        Ok(dst)
    }
}
