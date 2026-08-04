#[derive(Clone)]
pub struct PatchRule {
    pub name: &'static str,
    pub description: &'static str,
}

pub fn rules() -> Vec<PatchRule> {
    vec![
        PatchRule {
            name: "usb",
            description: "Disable pyudev USB monitor",
        },
        PatchRule {
            name: "bluetooth",
            description: "Disable Bluetooth discovery",
        },
        PatchRule {
            name: "dhcp",
            description: "Disable DHCP packet watcher",
        },
        PatchRule {
            name: "systemd",
            description: "Replace systemd with TriService",
        },
        PatchRule {
            name: "libpcap",
            description: "Disable packet capture",
        },
    ]
}
