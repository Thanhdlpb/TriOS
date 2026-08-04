use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct Manifest {
    pub name: String,

    pub version: String,

    pub description: String,

    pub ha_min: Option<String>,

    pub ha_max: Option<String>,

    pub author: Option<String>,
}

impl Manifest {
    pub fn load(path: &Path) -> Result<Self, String> {
        let data = fs::read_to_string(path).map_err(|e| e.to_string())?;

        toml::from_str(&data).map_err(|e| e.to_string())
    }
}
