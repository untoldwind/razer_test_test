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

    fn open(&self, path: &CStr) -> Result<Box<Device>> {
        let api = HidApi::new()?;
        let hid_device = api.open_path(path)?;

        Ok(Box::new(MatrixMice { hid_device }))
    }
}

pub struct MatrixMice {
    hid_device: HidDevice,
}

impl Device for MatrixMice {}
