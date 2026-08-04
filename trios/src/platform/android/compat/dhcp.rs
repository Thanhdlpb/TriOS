use super::CompatModule;

pub struct DhcpCompat;

impl CompatModule for DhcpCompat {

    fn name(&self) -> &'static str {
        "DHCP"
    }

    fn supported(&self) -> bool {
        false
    }

    fn patch(&self) {
        println!("TriPatch: DHCP watcher disabled.");
    }
}
