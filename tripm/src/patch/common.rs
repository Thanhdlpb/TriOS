use std::path::Path;
use std::process::Command;

pub trait Patch {
    fn name(&self) -> &'static str;

    fn check(&self) -> Result<(), String>;

    fn apply(&self) -> Result<(), String>;

    fn rollback(&self) -> Result<(), String>;

    fn status(&self) -> String;
}

pub fn is_android() -> bool {
    Path::new("/system/build.prop").exists()
}

pub fn is_proot() -> bool {
    Command::new("sh")
        .arg("-c")
        .arg("uname -r | grep -qi proot")
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn command_exists(cmd: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {} >/dev/null 2>&1", cmd))
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn hass_installed() -> bool {
    command_exists("hass") || Path::new("/root/homeassistant/bin/hass").exists()
}

pub fn hass_config() -> &'static str {
    "/root/.homeassistant/configuration.yaml"
}

pub fn hass_log() -> &'static str {
    "/root/.homeassistant/home-assistant.log"
}

pub fn print_header(name: &str) {
    println!();
    println!("======================================");
    println!(" TriPatch :: {}", name);
    println!("======================================");
}

pub fn ok(msg: &str) {
    println!("[ OK ] {}", msg);
}

pub fn warn(msg: &str) {
    println!("[WARN] {}", msg);
}

pub fn fail(msg: &str) {
    println!("[FAIL] {}", msg);
}
