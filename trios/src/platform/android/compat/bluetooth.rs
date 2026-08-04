use super::CompatModule;

pub struct BluetoothCompat;

impl CompatModule for BluetoothCompat {

    fn name(&self) -> &'static str {
        "Bluetooth"
    }

    fn supported(&self) -> bool {
        false
    }

    fn patch(&self) {
        println!("TriPatch: Bluetooth disabled.");
    }
}
