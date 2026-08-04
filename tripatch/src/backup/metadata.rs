use std::fs;
use std::path::Path;

pub fn write_metadata(
    dir: &Path,
    id: &str,
    plugin: &str,
    source: &str,
    backup: &str,
) -> Result<(), String> {
    let data = format!(
        r#"id = "{}"
plugin = "{}"

[[files]]
source = "{}"
backup = "{}"
"#,
        id, plugin, source, backup
    );

    fs::write(dir.join("backup.toml"), data).map_err(|e| e.to_string())
}
