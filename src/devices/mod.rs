use std::collections::HashMap;

mod matrix_mice;
mod razer_report;
mod soft_keyboard;

use errors::Result;
use hidapi::HidApi;
use std::ffi::CStr;

use self::matrix_mice::MatrixMiceFactory;
use self::soft_keyboard::SoftKeyboardFactory;

const RAZER_VENDOR: u16 = 0x1532;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DeviceId {
    vendor_id: u16,
    product_id: u16,
    interface_number: i32,
}

pub trait DeviceFactory: Sync {
    fn name(&self) -> String;

    fn open(&self, path: &CStr) -> Result<Box<Device>>;
}

pub trait Device {}

impl DeviceId {
    pub fn new(vendor_id: u16, product_id: u16, interface_number: i32) -> DeviceId {
        DeviceId {
            vendor_id,
            product_id,
            interface_number,
        }
    }
}

lazy_static! {
    static ref known_devices: HashMap<DeviceId, Box<DeviceFactory>> = {
        let mut map = HashMap::<DeviceId, Box<DeviceFactory>>::new();

        map.insert(
            DeviceId::new(RAZER_VENDOR, 0x0060, 0),
            MatrixMiceFactory::new("Razer Lancehead TE"),
        );
        map.insert(
            DeviceId::new(RAZER_VENDOR, 0x0226, 0),
            SoftKeyboardFactory::new("Razer Huntsman Elite"),
        );
        map
    };
}

pub fn list_devices() -> Result<Vec<&'static DeviceFactory>> {
    let api = HidApi::new()?;
    let mut devices: Vec<&'static DeviceFactory> = Vec::new();

    for hid_device in api.devices() {
        if let Some(device_factory) = known_devices.get(&DeviceId::new(
            hid_device.vendor_id,
            hid_device.product_id,
            hid_device.interface_number,
        )) {
            devices.push(device_factory.as_ref());
        }
    }

    Ok(devices)
}
