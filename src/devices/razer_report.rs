use std::mem;
use std::slice;

#[repr(C)]
pub struct RazerReport {
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
}
