use super::razer_report::{Color, RazerMouseMatrixEffectId, RazerReport, RazerVarstore};
use super::{Device, DeviceFactory};
use errors::Result;
use hidapi::HidDevice;

#[derive(Clone, Debug)]
pub struct MatrixMiceFactory {
    name: &'static str,
    led_ids: &'static [u8],
}

impl MatrixMiceFactory {
    pub fn new(name: &'static str, led_ids: &'static [u8]) -> Box<MatrixMiceFactory> {
        Box::new(MatrixMiceFactory { name, led_ids })
    }
}

impl DeviceFactory for MatrixMiceFactory {
    fn name(&self) -> &'static str {
        self.name
    }

    fn open(&self, hid_device: HidDevice) -> Box<Device> {
        Box::new(MatrixMice {
            name: self.name,
            led_ids: self.led_ids,
            hid_device,
        })
    }
}

pub struct MatrixMice {
    name: &'static str,
    led_ids: &'static [u8],
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
            self.led_ids[0],
        ))?;
        Ok(0)
    }

    fn set_color(&self, color: Color) -> Result<()> {
        for led_id in self.led_ids {
            let mut report = RazerReport::extended_mouse_matrix_effect(
                RazerVarstore::Store,
                *led_id,
                RazerMouseMatrixEffectId::Static,
            );
            report.arguments[5] = 1;
            report.arguments[6] = color.red;
            report.arguments[7] = color.green;
            report.arguments[8] = color.blue;

            self.send_report(report)?;
        }
        Ok(())
    }
}
