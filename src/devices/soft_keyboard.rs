use super::razer_report::{Color, RazerReport, RazerVarstore};
use super::{Device, DeviceFactory};
use errors::Result;
use hidapi::HidDevice;

pub struct SoftKeyboardFactory {
    name: &'static str,
}

impl SoftKeyboardFactory {
    pub fn new(name: &'static str) -> Box<SoftKeyboardFactory> {
        Box::new(SoftKeyboardFactory { name })
    }
}

impl DeviceFactory for SoftKeyboardFactory {
    fn name(&self) -> &'static str {
        self.name.clone()
    }

    fn open(&self, hid_device: HidDevice) -> Box<Device> {
        Box::new(SoftKeyboard {
            name: self.name,
            hid_device,
        })
    }
}

pub struct SoftKeyboard {
    name: &'static str,
    hid_device: HidDevice,
}

impl Device for SoftKeyboard {
    fn name(&self) -> &'static str {
        self.name
    }

    fn hid_device<'a>(&'a self) -> &'a HidDevice {
        &self.hid_device
    }

    fn get_brightness(&self) -> Result<u8> {
        self.send_report(RazerReport::matrix_get_brightness(RazerVarstore::Store, 0))?;
        Ok(0)
    }

    fn set_color(&self, color: Color) -> Result<()> {
        let colors = [color; 23];
        for row in 0..9 {
            self.send_report(RazerReport::soft_matrix_frame(row, 0, &colors))?;
        }
        Ok(())
    }
}
