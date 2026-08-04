pub mod config;
pub mod aiodns;
pub mod backup;
pub mod bluetooth;
pub mod common;
pub mod engine;
pub mod libpcap;
pub mod network;
pub mod registry;
pub mod rollback;
pub mod systemd;
pub mod usb;

use crate::util;
use std::fs;
use std::path::Path;

const PATCH_DIR: &str = "/root/.trios/patches";
const CONFIG: &str = "/root/.homeassistant/configuration.yaml";

pub fn init() {
    fs::create_dir_all(PATCH_DIR).ok();
}

fn marker(name: &str) -> String {
    format!("{}/{}", PATCH_DIR, name)
}

fn is_android() -> bool {
    Path::new("/system/build.prop").exists()
}

fn is_proot() -> bool {
    std::process::Command::new("sh")
        .arg("-c")
        .arg("uname -r | grep -qi proot")
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn list() {
    init();

    println!("== Installed patches ==");

    if let Ok(entries) = fs::read_dir(PATCH_DIR) {
        let mut found = false;

        for e in entries.flatten() {
            found = true;
            println!("{}", e.file_name().to_string_lossy());
        }

        if !found {
            println!("(none)");
        }
    }
}

pub fn apply(name: &str) {
    init();

    match name {
        "usb" => {
            usb::apply();
            apply_usb();
        }

        "bluetooth" => {
            bluetooth::apply();
            fs::write(marker("bluetooth"), "installed").ok();
        }

        "aiodns" => {
            aiodns::apply();
            fs::write(marker("aiodns"), "installed").ok();
        }

        "libpcap" => {
            libpcap::apply();
            fs::write(marker("libpcap"), "installed").ok();
        }

        "network" => {
            network::apply();
            fs::write(marker("network"), "installed").ok();
        }

        "systemd" => {
            systemd::apply();
            fs::write(marker("systemd"), "installed").ok();
        }

        _ => {
            fs::write(marker(name), "installed").ok();
            println!("Applied patch '{}'", name);
        }
    }
}

pub fn remove(name: &str) {
    match name {
        "usb" => {
            rollback::rollback("usb");
            rollback_usb();
        }

        "bluetooth" => rollback::rollback("bluetooth"),
        "aiodns" => rollback::rollback("aiodns"),
        "libpcap" => rollback::rollback("libpcap"),
        "network" => rollback::rollback("network"),
        "systemd" => rollback::rollback("systemd"),

        _ => {
            let p = marker(name);

            if Path::new(&p).exists() {
                fs::remove_file(p).ok();
            }

            println!("Removed patch '{}'", name);
        }
    }
}

fn apply_usb() {
    println!("== TriPatch USB ==");

    if !is_android() {
        println!("Not Android.");
        return;
    }

    if !is_proot() {
        println!("Not PRoot.");
        return;
    }

    if !Path::new(CONFIG).exists() {
        println!("configuration.yaml not found.");
        return;
    }

    util::backup(CONFIG);

    let mut cfg = fs::read_to_string(CONFIG).unwrap();

    if !cfg.contains("# TriPatch USB") {
        cfg.push_str(
            r#"

# TriPatch USB
# Android/PRoot compatibility
default_config:

"#,
        );

        fs::write(CONFIG, cfg).unwrap();
    }

    fs::write(marker("usb"), "installed").ok();

    println!("Backup created.");
    println!("USB compatibility patch applied.");
}

fn rollback_usb() {
    util::restore(CONFIG);

    let p = marker("usb");

    if Path::new(&p).exists() {
        fs::remove_file(p).ok();
    }

    println!("USB patch rolled back.");
}
