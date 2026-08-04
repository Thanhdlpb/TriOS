use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

const HASS: &str = "/root/homeassistant/bin/hass";

fn running() -> bool {
    Command::new("sh")
        .arg("-c")
        .arg("pgrep -f '/root/homeassistant/bin/hass' >/dev/null")
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn version() {
    Command::new(HASS).arg("--version").status().ok();
}

pub fn status() {
    println!("");

    if !Path::new(HASS).exists() {
        println!("Home Assistant chưa được cài.");
        return;
    }

    if running() {
        println!("Status : RUNNING");
    } else {
        println!("Status : STOPPED");
    }
}

pub fn start() {
    if running() {
        println!("Home Assistant đã chạy.");
        return;
    }

    Command::new(HASS)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Không thể khởi động Home Assistant");

    thread::sleep(Duration::from_secs(3));

    status();
}

pub fn stop() {
    Command::new("pkill")
        .arg("-f")
        .arg("/root/homeassistant/bin/hass")
        .status()
        .ok();

    thread::sleep(Duration::from_secs(2));

    status();
}

pub fn restart() {
    stop();

    thread::sleep(Duration::from_secs(2));

    start();
}

pub fn logs() {
    Command::new("tail")
        .args(["-100", "/root/.homeassistant/home-assistant.log"])
        .status()
        .ok();
}
