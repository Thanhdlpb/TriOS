use std::fs;
use std::path::Path;

use crate::doctor::{DoctorCheck, DoctorResult};

pub struct ConfigurationCheck;

impl DoctorCheck for ConfigurationCheck {
    fn name(&self) -> &'static str {
        "configuration"
    }

    fn run(&self) -> DoctorResult {
        let configs = [
            "/root/.homeassistant/configuration.yaml",
            "/config/configuration.yaml",
            "/homeassistant/configuration.yaml",
        ];

        for file in configs {
            let path = Path::new(file);

            if path.exists() {
                match fs::metadata(path) {
                    Ok(meta) => {
                        return DoctorResult::pass(
                            "Configuration",
                            &format!("configuration.yaml found ({} bytes)", meta.len()),
                        );
                    }

                    Err(_) => {
                        return DoctorResult::warn("Configuration", "Found but cannot read");
                    }
                }
            }
        }

        DoctorResult::warn("Configuration", "configuration.yaml not found")
    }
}
