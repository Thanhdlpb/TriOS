use std::fs;
use std::path::Path;

pub fn backup(file: &str) {
    let src = Path::new(file);

    if !src.exists() {
        return;
    }

    let bak = format!("{}.tripatch.bak", file);

    if !Path::new(&bak).exists() {
        if let Err(e) = fs::copy(src, &bak) {
            eprintln!("Backup failed: {}", e);
        }
    }
}

pub fn restore(file: &str) {
    let bak = format!("{}.tripatch.bak", file);

    if Path::new(&bak).exists() {
        if let Err(e) = fs::copy(&bak, file) {
            eprintln!("Restore failed: {}", e);
        }
    }
}

pub fn exists(file: &str) -> bool {
    Path::new(file).exists()
}
