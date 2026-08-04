pub mod bluetooth;
pub mod dhcp;
pub mod systemd;
pub mod udev;
pub mod usb;

pub trait CompatModule {
    fn name(&self) -> &'static str;

    fn detect(&self) -> bool;

    fn patch(&self) -> bool;

    fn restore(&self) -> bool;

    fn report(&self) {
        println!(
            "{:<15} {}",
            self.name(),
            if self.detect() {
                "SUPPORTED"
            } else {
                "PATCH REQUIRED"
            }
        );
    }
}

pub fn modules() -> Vec<Box<dyn CompatModule>> {
    vec![
        Box::new(usb::UsbCompat),
        Box::new(bluetooth::BluetoothCompat),
        Box::new(dhcp::DhcpCompat),
        Box::new(systemd::SystemdCompat),
        Box::new(udev::UdevCompat),
    ]
}

pub fn doctor() {
    println!();
    println!("========== TriCompat Doctor ==========");

    for m in modules() {
        m.report();
    }

    println!("======================================");
}

pub fn patch_all() {
    println!();
    println!("========== Applying TriPatch =========");

    for m in modules() {

        if !m.detect() {

            print!("Patching {:<15}", m.name());

            if m.patch() {
                println!("OK");
            } else {
                println!("FAILED");
            }

        }

    }

    println!("======================================");
}
