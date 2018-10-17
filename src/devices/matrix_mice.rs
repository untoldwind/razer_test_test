use super::{Device, DeviceFactory};
use errors::Result;
use hidapi::{HidApi, HidDevice};
use std::ffi::CStr;

#[derive(Clone, Debug)]
pub struct MatrixMiceFactory {
    name: String,
}

impl MatrixMiceFactory {
    pub fn new<S: Into<String>>(name: S) -> Box<MatrixMiceFactory> {
        Box::new(MatrixMiceFactory { name: name.into() })
    }
}

impl DeviceFactory for MatrixMiceFactory {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn open(&self, hid_device: HidDevice) -> Box<Device> {
        Box::new(MatrixMice { name: self.name.clone(), hid_device })
    }
}

pub struct MatrixMice {
    name: String,
    hid_device: HidDevice,
}

impl Device for MatrixMice {
    fn name(& self) -> String {
        self.name.clone()
    }

    fn hid_device<'a>(&'a self) -> &'a HidDevice {
        &self.hid_device
    }
}
