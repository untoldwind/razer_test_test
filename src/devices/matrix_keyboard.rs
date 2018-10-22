use super::razer_report::{Color, RazerMatrixEffectId, RazerReport, RazerVarstore};
use super::{Device, DeviceFactory};
use errors::Result;
use hidapi::HidDevice;

#[derive(Clone, Debug)]
pub struct MatrixKeyboardFactory {
    name: &'static str,
    led_ids: &'static [u8],
}

impl MatrixKeyboardFactory {
    pub fn new(name: &'static str, led_ids: &'static [u8]) -> Box<MatrixKeyboardFactory> {
        Box::new(MatrixKeyboardFactory { name, led_ids })
    }
}

impl DeviceFactory for MatrixKeyboardFactory {
    fn name(&self) -> &'static str {
        self.name
    }

    fn open(&self, hid_device: HidDevice) -> Box<Device> {
        Box::new(MatrixKeyboard {
            name: self.name,
            led_ids: self.led_ids,
            hid_device,
        })
    }
}

pub struct MatrixKeyboard {
    name: &'static str,
    led_ids: &'static [u8],
    hid_device: HidDevice,
}

impl Device for MatrixKeyboard {
    fn name(&self) -> &'static str {
        self.name
    }

    fn hid_device<'a>(&'a self) -> &'a HidDevice {
        &self.hid_device
    }

    fn get_brightness(&self) -> Result<u8> {
        self.send_report(RazerReport::standard_get_led_brightness(
            RazerVarstore::Store,
            self.led_ids[0],
        ))?;
        Ok(0)
    }

    fn set_brightness(&self, brightness: u8) -> Result<()> {
        for led_id in self.led_ids {
            self.send_report(RazerReport::standard_set_led_brightness(
                RazerVarstore::Store,
                *led_id,
                brightness,
            ))?;
        }
        Ok(())
    }

    fn set_color(&self, color: Color) -> Result<()> {
        let mut report = RazerReport::standard_matrix_effect(RazerMatrixEffectId::Static);
        report.arguments[1] = color.red;
        report.arguments[2] = color.green;
        report.arguments[3] = color.blue;

        self.send_report(report)?;
        Ok(())
    }
}
