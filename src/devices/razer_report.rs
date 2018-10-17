use std::mem;
use std::slice;

pub enum RazerStatus {
    New = 0x00,
    Busy = 0x01,
    Successful = 0x02,
    Failure = 0x03,
    NoResponseTimeout = 0x04,
    NotSupported = 0x05,
}

#[repr(C)]
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

            for i in 3..89 {
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
}

impl Default for RazerReport {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}
