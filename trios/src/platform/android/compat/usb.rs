use super::CompatModule;

pub struct UsbCompat;

impl CompatModule for UsbCompat {

    fn name(&self) -> &'static str {
        "USB"
    }

    fn supported(&self) -> bool {
        std::path::Path::new("/run/udev").exists()
    }

    fn patch(&self) {
        println!("TriPatch: USB monitor disabled.");
    }
}
