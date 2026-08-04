use crate::patch::backup;
use crate::patch::common::*;
use std::path::Path;

pub struct UsbPatch;

impl Patch for UsbPatch {
    fn name(&self) -> &'static str {
        "usb"
    }

    fn check(&self) -> Result<(), String> {
        print_header("USB");

        if !is_proot() {
            warn("Không chạy trong PRoot");
        } else {
            ok("PRoot detected");
        }

        if is_android() {
            ok("Android detected");
        } else {
            warn("Android not detected");
        }

        if hass_installed() {
            ok("Home Assistant found");
        } else {
            return Err("Home Assistant chưa được cài".into());
        }

        if Path::new(hass_config()).exists() {
            ok("configuration.yaml found");
        } else {
            warn("configuration.yaml not found");
        }

        Ok(())
    }

    fn apply(&self) -> Result<(), String> {
        self.check()?;

        if Path::new(hass_config()).exists() {
            backup::backup(hass_config());
            ok("Backup completed");
        }

        println!("USB patch prepared.");
        println!("(Giai đoạn sau sẽ tự sửa configuration.yaml)");

        Ok(())
    }

    fn rollback(&self) -> Result<(), String> {
        backup::restore(hass_config());

        ok("USB patch rollback completed");

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
    let patch = UsbPatch;

    if let Err(e) = patch.apply() {
        fail(&e);
    }
}
