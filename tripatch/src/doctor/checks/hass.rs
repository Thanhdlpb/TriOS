use std::path::Path;

use crate::doctor::{DoctorCheck, DoctorResult};

pub struct HassDiscoveryCheck;

impl DoctorCheck for HassDiscoveryCheck {
    fn name(&self) -> &'static str {
        "home_assistant"
    }

    fn run(&self) -> DoctorResult {
        let paths = [
            "/root/.homeassistant",
            "/config",
            "/homeassistant",
            "/usr/share/hassio/homeassistant",
        ];

        for path in paths {
            if Path::new(path).exists() {
                return DoctorResult::pass("Home Assistant", &format!("Found: {}", path));
            }
        }

        DoctorResult::warn("Home Assistant", "Không tìm thấy thư mục Home Assistant")
    }
}
