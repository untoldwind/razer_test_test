use std::collections::HashMap;

mod matrix_mice;
mod razer_report;
mod soft_keyboard;

use errors::{Error, ErrorKind, Result};
use hidapi::{HidApi, HidDevice};
use std::ffi::{CStr, CString};
use std::thread;
use std::time;

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
    fn name(&self) -> String;

    fn open(&self, hid_device: HidDevice) -> Box<Device>;
}

pub trait Device {
    fn name(&self) -> String;

    fn hid_device<'a>(&'a self) -> &'a HidDevice;

    fn send_report(&self, mut request: RazerReport) -> Result<RazerReport> {
        let mut result: RazerReport = Default::default();
        request.calculate_crc();

        let mut last_error: Option<Error> = None;

        for retry in 0..3 {
            match self.hid_device().send_feature_report(request.as_raw()) {
                Ok(_) => (),
                Err(error) => {
                    println!("One: {:?}", error);
                    last_error = Some(error.into());
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
                    println!("Two: {:?}", error);
                    last_error = Some(error.into());
                    continue;
                }
            }

            println!("{}", result.status);
            if result.status == RazerStatus::NotSupported as u8 {
                return Err(ErrorKind::NotSupported.into());
            } else if result.status == RazerStatus::Successful as u8 {
                return Ok(result);
            }
        }

        println!("{:?}", last_error);
        Err(last_error.unwrap())
    }

    fn get_serial(&self) -> Result<CString> {
        let result = self.send_report(RazerReport::standard_get_serial())?;

        Ok(CString::new(&result.arguments[..])?)
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
            MatrixMiceFactory::new("Razer Lancehead TE"),
        );
/*        map.insert(
            DeviceId::new(RAZER_VENDOR, 0x0226, 0),
            SoftKeyboardFactory::new("Razer Huntsman Elite"),
        );*/
        map
    };
}

pub fn list_devices() -> Result<Vec<Box<Device>>> {
    let api = HidApi::new()?;
    let mut devices: Vec<Box<Device>> = Vec::new();

    for hid_device in api.devices() {
        if let Some(device_factory) = known_devices.get(&DeviceId::new(
            hid_device.vendor_id,
            hid_device.product_id,
            hid_device.interface_number,
        )) {
            devices.push(device_factory.open(hid_device.open_device(&api)?));
        }
    }

    Ok(devices)
}
