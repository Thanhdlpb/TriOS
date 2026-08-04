use std::path::Path;

use crate::doctor::{DoctorCheck, DoctorResult};

pub struct FilesystemCheck;

impl DoctorCheck for FilesystemCheck {
    fn name(&self) -> &'static str {
        "filesystem"
    }

    fn run(&self) -> DoctorResult {
        let dirs = ["/root/.homeassistant", "/config", "/homeassistant"];

        for dir in dirs {
            if Path::new(dir).exists() {
                return DoctorResult::pass("Filesystem", &format!("Đã tìm thấy: {}", dir));
            }
        }

        DoctorResult::warn("Filesystem", "Không tìm thấy thư mục Home Assistant")
    }
}
