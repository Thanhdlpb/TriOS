use crate::patch::common::Patch;

use crate::patch::bluetooth::BluetoothPatch;
use crate::patch::usb::UsbPatch;

pub fn registry() -> Vec<Box<dyn Patch>> {
    vec![Box::new(UsbPatch), Box::new(BluetoothPatch)]
}
