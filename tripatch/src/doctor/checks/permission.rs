use std::fs;
use std::path::Path;

use crate::doctor::{DoctorCheck, DoctorResult};

pub struct PermissionCheck;

impl DoctorCheck for PermissionCheck {
    fn name(&self) -> &'static str {
        "permission"
    }

    fn run(&self) -> DoctorResult {
        let dirs = ["/root/.homeassistant", "/config", "/homeassistant"];

        for dir in dirs {
            let path = Path::new(dir);

            if path.exists() {
                let test = path.join(".trios_permission_test");

                match fs::write(&test, "test") {
                    Ok(_) => {
                        let _ = fs::remove_file(&test);

                        return DoctorResult::pass(
                            "Permission",
                            &format!("Read/Write OK: {}", dir),
                        );
                    }

                    Err(e) => {
                        return DoctorResult::warn(
                            "Permission",
                            &format!("Cannot write {}: {}", dir, e),
                        );
                    }
                }
            }
        }

        DoctorResult::warn("Permission", "No Home Assistant directory")
    }
}
