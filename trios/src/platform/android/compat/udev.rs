use super::CompatModule;

pub struct UdevCompat;

impl CompatModule for UdevCompat {

    fn name(&self) -> &'static str {
        "udev"
    }

    fn supported(&self) -> bool {
        std::path::Path::new("/run/udev").exists()
    }

    fn patch(&self) {
        println!("TriPatch: udev compatibility enabled.");
    }
}
