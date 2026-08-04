use std::time::{SystemTime, UNIX_EPOCH};

pub fn backup_id() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    format!("backup-{}", now)
}
