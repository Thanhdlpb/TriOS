use crate::patch::backup;
use crate::patch::common::*;
use std::path::Path;

pub struct BluetoothPatch;

impl Patch for BluetoothPatch {
    fn name(&self) -> &'static str {
        "bluetooth"
    }

    fn check(&self) -> Result<(), String> {
        print_header("Bluetooth");

        if is_android() {
            ok("Android detected");
        } else {
            warn("Android not detected");
        }

        if is_proot() {
            ok("PRoot detected");
        } else {
            warn("Not running inside PRoot");
        }

        if hass_installed() {
            ok("Home Assistant found");
        } else {
            return Err("Home Assistant not found".into());
        }

        if Path::new(hass_config()).exists() {
            ok("configuration.yaml found");
        } else {
            warn("configuration.yaml missing");
        }

        Ok(())
    }

    fn apply(&self) -> Result<(), String> {
        self.check()?;

        if Path::new(hass_config()).exists() {
            backup::backup(hass_config());
            ok("Backup completed");
        }

        println!("Bluetooth compatibility patch prepared.");
        println!("(Giai đoạn sau sẽ tự điều chỉnh cấu hình Bluetooth cho Android/PRoot)");

        Ok(())
    }

    fn rollback(&self) -> Result<(), String> {
        backup::restore(hass_config());

        ok("Bluetooth patch rollback completed");

        Ok(())
    }

    fn status(&self) -> String {
        if Path::new(hass_config()).exists() {
            "READY".into()
        } else {
            "NO CONFIG".into()
        }
    }
}

pub fn apply() {
    let patch = BluetoothPatch;

    if let Err(e) = patch.apply() {
        fail(&e);
    }
}
