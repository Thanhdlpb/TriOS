use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
pub struct AndroidInfo {
    pub android: bool,
    pub termux: bool,
    pub proot: bool,
    pub distro: String,
    pub kernel: String,
    pub hostname: String,
    pub systemd: bool,
    pub udev: bool,
    pub python: String,
    pub rust: String,
    pub cargo: String,
    pub git: String,
}

fn run(cmd: &str, args: &[&str]) -> String {
    match Command::new(cmd).args(args).output() {
        Ok(o) => {
            let out = if o.stdout.is_empty() {
                String::from_utf8_lossy(&o.stderr).to_string()
            } else {
                String::from_utf8_lossy(&o.stdout).to_string()
            };
            out.trim().to_string()
        }
        Err(_) => "Not Found".into(),
    }
}

pub fn detect() -> AndroidInfo {
    let distro = fs::read_to_string("/etc/os-release")
        .unwrap_or_default()
        .lines()
        .find(|l| l.starts_with("PRETTY_NAME="))
        .unwrap_or("PRETTY_NAME=Unknown")
        .replace("PRETTY_NAME=", "")
        .replace("\"", "");

    AndroidInfo {
        android: Path::new("/system").exists(),
        termux: env::var("PREFIX").unwrap_or_default().contains("com.termux"),
        proot: env::var("PROOT_TMP_DIR").is_ok()
            || env::var("PROOT_LOADER").is_ok(),
        distro,
        kernel: run("uname", &["-r"]),
        hostname: run("hostname", &[]),
        systemd: Path::new("/run/systemd/system").exists(),
        udev: Path::new("/run/udev").exists(),
        python: run("python3", &["--version"]),
        rust: run("rustc", &["--version"]),
        cargo: run("cargo", &["--version"]),
        git: run("git", &["--version"]),
    }
}
