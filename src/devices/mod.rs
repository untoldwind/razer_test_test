use std::collections::HashMap;

mod matrix_keyboard;
mod matrix_mice;
mod razer_report;
mod soft_keyboard;

pub use self::razer_report::Color;
use errors::{Error, ErrorKind, Result};
use hidapi::{HidApi, HidDevice};
use log::Level;
use std::ffi::CString;
use std::thread;
use std::time;

use self::matrix_keyboard::MatrixKeyboardFactory;
use self::matrix_mice::MatrixMiceFactory;
use self::razer_report::{RazerReport, RazerStatus};
use self::soft_keyboard::SoftKeyboardFactory;

const RAZER_VENDOR: u16 = 0x1532;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DeviceId {
    vendor_id: u16,
    product_id: u16,
    interface_number: i32,
}

pub trait DeviceFactory: Sync {
    fn name(&self) -> &'static str;

    fn open(&self, hid_device: HidDevice) -> Box<Device>;
}

pub trait Device {
    fn name(&self) -> &'static str;

    fn hid_device<'a>(&'a self) -> &'a HidDevice;

    fn get_brightness(&self) -> Result<u8>;

    fn set_brightness(&self, brightness: u8) -> Result<()>;

    fn set_color(&self, color: Color) -> Result<()>;

    fn get_manufacturer(&self) -> Result<Option<String>> {
        Ok(self.hid_device().get_manufacturer_string()?)
    }

    fn get_product(&self) -> Result<Option<String>> {
        Ok(self.hid_device().get_product_string()?)
    }

    fn send_report(&self, mut request: RazerReport) -> Result<RazerReport> {
        let mut result: RazerReport = Default::default();
        request.calculate_crc();

        let mut last_error: Error = ErrorKind::NotSuccessful.into();

        for retry in 0..3 {
            if log_enabled!(Level::Debug) {
                debug!("Sending  >>>: {:?}", request);
            }
            match self.hid_device().send_feature_report(request.as_raw()) {
                Ok(_) => (),
                Err(error) => {
                    last_error = error.into();
                    continue;
                }
            }

            if retry == 0 {
                thread::sleep(time::Duration::from_micros(800));
            } else {
                thread::sleep(time::Duration::from_micros(8000));
            }

            match self.hid_device().get_feature_report(result.as_mut_raw()) {
                Ok(_) => (),
                Err(error) => {
                    last_error = error.into();
                    continue;
                }
            }
            if log_enabled!(Level::Debug) {
                debug!("Received <<<: {:?}", result);
            }

            if result.status == RazerStatus::NotSupported as u8 {
                return Err(ErrorKind::NotSupported.into());
            } else if result.status == RazerStatus::Successful as u8 {
                return Ok(result);
            }
        }

        Err(last_error)
    }

    fn get_serial(&self) -> Result<CString> {
        let result = self.send_report(RazerReport::standard_get_serial())?;
        let mut size = result.data_size as usize;

        size = result
            .arguments
            .iter()
            .take(size)
            .position(|c| *c == 0x0)
            .unwrap_or(size);

        Ok(CString::new(&result.arguments[0..size])?)
    }
}

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
            MatrixMiceFactory::new("Razer Lancehead TE", &[1, 4, 16, 17]),
        );
        map.insert(
            DeviceId::new(RAZER_VENDOR, 0x0226, 0),
            SoftKeyboardFactory::new("Razer Huntsman Elite"),
        );
        map.insert(
            DeviceId::new(RAZER_VENDOR, 0x0221, 0),
            MatrixKeyboardFactory::new("Razer BlackWidow Chroma V2", &[5]),
        );
        map
    };
}

pub fn list_devices() -> Result<Vec<Box<Device>>> {
    let api = HidApi::new()?;
    let mut devices: Vec<Box<Device>> = Vec::new();

    for hid_device_info in api.devices() {
        if let Some(device_factory) = known_devices.get(&DeviceId::new(
            hid_device_info.vendor_id,
            hid_device_info.product_id,
            hid_device_info.interface_number,
        )) {
            let hid_device = hid_device_info.open_device(&api)?;

            devices.push(device_factory.open(hid_device));
        }
    }

    Ok(devices)
}
