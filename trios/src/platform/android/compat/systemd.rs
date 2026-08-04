use super::CompatModule;

pub struct SystemdCompat;

impl CompatModule for SystemdCompat {

    fn name(&self) -> &'static str {
        "Systemd"
    }

    fn supported(&self) -> bool {
        std::path::Path::new("/run/systemd/system").exists()
    }

    fn patch(&self) {
        println!("TriPatch: Using TriService instead of systemd.");
    }
}
