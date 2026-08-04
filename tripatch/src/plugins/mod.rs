use std::fs;
use std::path::PathBuf;

use crate::manifest::Manifest;

#[derive(Debug)]
pub struct Plugin {
    pub path: PathBuf,
    pub manifest: Manifest,
}

pub fn scan_plugins(root: &str) -> Vec<Plugin> {
    let mut result = Vec::new();

    let dir = PathBuf::from(root);
    if !dir.exists() {
        return result;
    }

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                let mf = path.join("manifest.toml");

                if mf.exists() {
                    if let Ok(manifest) = Manifest::load(&mf) {
                        result.push(Plugin { path, manifest });
                    }
                }
            }
        }
    }

    result
}
