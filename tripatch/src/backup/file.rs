use std::fs;
use std::path::{Path, PathBuf};

pub fn backup_file(src: &Path, dst: &Path) -> Result<PathBuf, String> {
    if !src.exists() {
        return Err(format!("Không tìm thấy {}", src.display()));
    }

    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    fs::copy(src, dst).map_err(|e| e.to_string())?;

    Ok(dst.to_path_buf())
}
