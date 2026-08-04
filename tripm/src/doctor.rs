use std::path::Path;
use std::process::Command;

fn ok(name: &str) {
    println!("[ OK ] {}", name);
}

fn warn(name: &str) {
    println!("[WARN] {}", name);
}

fn check(cmd: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {} >/dev/null 2>&1", cmd))
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn output(cmd: &str) -> String {
    let out = Command::new("sh").arg("-c").arg(cmd).output();

    match out {
        Ok(v) => String::from_utf8_lossy(&v.stdout).trim().to_string(),
        Err(_) => String::new(),
    }
}

pub fn run() {
    println!();
    println!("========== TriOS Doctor ==========");

    if Path::new("/system/build.prop").exists() {
        ok("Android");
    } else {
        warn("Android");
    }

    if std::env::var("PROOT_TMP_DIR").is_ok() {
        ok("PRoot");
    } else {
        warn("PRoot");
    }

    println!(
        "OS      : {}",
        output("grep PRETTY_NAME /etc/os-release|cut -d= -f2|tr -d '\"'")
    );
    println!("Kernel  : {}", output("uname -r"));
    println!("CPU     : {}", output("uname -m"));
    println!(
        "Memory  : {}",
        output("free -h|awk '/Mem/{print $2\" total, \"$7\" available\"}'")
    );
    println!(
        "Disk    : {}",
        output("df -h /|awk 'NR==2{print $4\" free\"}'")
    );

    println!();

    for c in [
        "git",
        "cargo",
        "rustc",
        "python3",
        "pip",
        "node",
        "java",
        "hass",
        "mosquitto",
        "nginx",
        "psql",
        "mariadb",
    ] {
        if check(c) {
            ok(c);
        } else {
            warn(c);
        }
    }

    println!();

    println!("Rust   : {}", output("rustc --version"));
    println!("Cargo  : {}", output("cargo --version"));
    println!("Python : {}", output("python3 --version"));
    println!("HASS   : {}", output("hass --version"));

    println!("==================================");
}
