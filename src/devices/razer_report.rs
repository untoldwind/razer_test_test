use hex::ToHex;
use std::fmt;
use std::mem;
use std::ptr;
use std::slice;

use errors::{ErrorKind, Result};

#[allow(dead_code)]
pub enum RazerStatus {
    New = 0x00,
    Busy = 0x01,
    Successful = 0x02,
    Failure = 0x03,
    NoResponseTimeout = 0x04,
    NotSupported = 0x05,
}

#[allow(dead_code)]
pub enum RazerVarstore {
    NoStore = 0x00,
    Store = 0x01,
}

#[allow(dead_code)]
pub enum RazerMouseMatrixEffectId {
    Off = 0x00,
    Static = 0x01,
    Breathing = 0x02,
    Spectrum = 0x03,
    Wave = 0x04,
    Reactive = 0x05,
    Starlight = 0x07,
    CustomFrame = 0x08,
}

#[derive(Default, Clone, Copy)]
#[repr(packed)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    #[allow(dead_code)]
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    pub fn parse(color_str: &str) -> Result<Color> {
        let parts: Vec<u8> = color_str.split(',').map(|p| p.parse::<u8>().unwrap_or(0)).collect();

        if parts.len() != 3 {
            return Err(ErrorKind::InvalidColorFormat.into());
        }

        Ok(Color {
            red: parts[0],
            green: parts[1],
            blue: parts[2],
        })
    }
}

#[repr(packed)]
pub struct RazerReport {
    pub report_id: u8,
    pub status: u8,
    pub transaction_id: u8,
    pub remaining_packets: u16,
    pub protocol_type: u8,
    pub data_size: u8,
    pub command_class: u8,
    pub command_id: u8,
    pub arguments: [u8; 80],
    pub crc: u8,
    pub reserved: u8,
}

impl RazerReport {
    pub fn as_raw(&self) -> &[u8] {
        unsafe { slice::from_raw_parts((self as *const Self) as *const u8, mem::size_of::<Self>()) }
    }

    pub fn as_mut_raw(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut((self as *mut Self) as *mut u8, mem::size_of::<Self>()) }
    }

    pub fn calculate_crc(&mut self) {
        let mut crc = 0u8;
        {
            let raw = self.as_raw();

            for i in 4..90 {
                crc ^= raw[i]
            }
        }

        self.crc = crc;
    }

    pub fn standard_get_serial() -> RazerReport {
        RazerReport {
            transaction_id: 0x1f,
            command_class: 0x0,
            command_id: 0x82,
            data_size: 0x16,
            ..Default::default()
        }
    }

    pub fn soft_matrix_frame(row: u8, start_col: u8, colors: &[Color]) -> RazerReport {
        let mut report = RazerReport {
            transaction_id: 0x1f,
            command_class: 0xf,
            command_id: 0x03,
            data_size: 5 + 3 * (colors.len() as u8),
            ..Default::default()
        };
        report.arguments[2] = row;
        report.arguments[3] = start_col;
        report.arguments[4] = start_col + (colors.len() as u8);
        unsafe {
            ptr::copy_nonoverlapping::<u8>(
                (&colors[0] as *const Color) as *const u8,
                &mut report.arguments[5],
                colors.len() * 3,
            )
        }

        report
    }

    pub fn matrix_get_brightness(store: RazerVarstore, led_id: u8) -> RazerReport {
        let mut report = RazerReport {
            transaction_id: 0x1f,
            command_class: 0xf,
            command_id: 0x84,
            data_size: 0x3,
            ..Default::default()
        };
        report.arguments[0] = store as u8;
        report.arguments[1] = led_id;

        report
    }

    pub fn extended_mouse_matrix_effect(
        store: RazerVarstore,
        led_id: u8,
        effect: RazerMouseMatrixEffectId,
    ) -> RazerReport {
        let mut report = RazerReport {
            transaction_id: 0x1f,
            command_class: 0xf,
            command_id: 0x02,
            data_size: 80,
            ..Default::default()
        };
        report.arguments[0] = store as u8;
        report.arguments[1] = led_id;
        report.arguments[2] = effect as u8;

        report
    }
}

impl Default for RazerReport {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl fmt::Debug for RazerReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "status: {:X} tx: {:X} class: {:X} cmd: {:X} data: {:X} |",
            self.status, self.transaction_id, self.command_class, self.command_id, self.data_size
        )?;
        (&self.arguments[..]).write_hex_upper(f)?;
        write!(f, "| crc: {:X}", self.crc)
    }
}
