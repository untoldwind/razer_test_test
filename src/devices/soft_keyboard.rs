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

    fn open(&self, hid_device: HidDevice) -> Box<Device> {
        Box::new(SoftKeyboard { name: self.name.clone(), hid_device })
    }
}

pub struct SoftKeyboard {
    name: String,
    hid_device: HidDevice,
}

impl Device for SoftKeyboard {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn hid_device<'a>(&'a self) -> &'a HidDevice {
        &self.hid_device
    }
}
