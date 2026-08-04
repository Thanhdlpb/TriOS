use super::detect;

pub fn report() {
    let d = detect::detect();

    println!();
    println!("========== TriOS Android Doctor ==========");
    println!("Android      : {}", d.android);
    println!("Termux       : {}", d.termux);
    println!("PRoot        : {}", d.proot);
    println!("Distribution : {}", d.distro);
    println!("Kernel       : {}", d.kernel);
    println!("Hostname     : {}", d.hostname);
    println!("Systemd      : {}", d.systemd);
    println!("Udev         : {}", d.udev);
    println!("Python       : {}", d.python);
    println!("Rust         : {}", d.rust);
    println!("Cargo        : {}", d.cargo);
    println!("Git          : {}", d.git);
    println!("==========================================");
}
