use super::{Device, DeviceFactory};
use errors::Result;
use hidapi::{HidApi, HidDevice};
use std::ffi::CStr;

pub struct SoftKeyboardFactory {
    name: String,
}

impl SoftKeyboardFactory {
    pub fn new<S: Into<String>>(name: S) -> Box<SoftKeyboardFactory> {
        Box::new(SoftKeyboardFactory { name: name.into() })
    }
}

impl DeviceFactory for SoftKeyboardFactory {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn open(&self, path: &CStr) -> Result<Box<Device>> {
        let api = HidApi::new()?;
        let hid_device = api.open_path(path)?;

        Ok(Box::new(SoftKeyboard { hid_device }))
    }
}

pub struct SoftKeyboard {
    hid_device: HidDevice,
}

impl Device for SoftKeyboard {}
