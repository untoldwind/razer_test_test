use super::razer_report::{Color, RazerReport, RazerVarstore};
use super::{Device, DeviceFactory};
use errors::Result;
use hidapi::HidDevice;

#[derive(Clone, Debug)]
pub struct MatrixMiceFactory {
    name: &'static str,
    brightness_led: u8,
}

impl MatrixMiceFactory {
    pub fn new(name: &'static str, brightness_led: u8) -> Box<MatrixMiceFactory> {
        Box::new(MatrixMiceFactory { name, brightness_led })
    }
}

impl DeviceFactory for MatrixMiceFactory {
    fn name(&self) -> &'static str {
        self.name
    }

    fn open(&self, hid_device: HidDevice) -> Box<Device> {
        Box::new(MatrixMice {
            name: self.name,
            brightness_led: self.brightness_led,
            hid_device,
        })
    }
}

pub struct MatrixMice {
    name: &'static str,
    brightness_led: u8,
    hid_device: HidDevice,
}

impl Device for MatrixMice {
    fn name(&self) -> &'static str {
        self.name
    }

    fn hid_device<'a>(&'a self) -> &'a HidDevice {
        &self.hid_device
    }

    fn get_brightness(&self) -> Result<u8> {
        self.send_report(RazerReport::matrix_get_brightness(
            RazerVarstore::Store,
            self.brightness_led,
        ))?;
        Ok(0)
    }

    fn set_color(&self, color: Color) -> Result<()> {
        Ok(())
    }
}
