use core::fmt;

const DEFAULT_BASE_ADDRESS: usize = 0x900_0000;
const FLAG_REGISTER_OFFSET: usize = 0x18;
const FR_BUSY: u8 = 1 << 3;
const FR_TXFF: u8 = 1 << 5;

pub struct Pl011 {
    base_address: *mut u8
}

impl Default for Pl011 {
    fn default() -> Self {
        Pl011 {base_address: DEFAULT_BASE_ADDRESS as *mut u8 }
    }
}

impl Pl011 {
    pub fn write_byte(&self, byte: u8) {
        while self.read_flag_register() & FR_TXFF != 0 {}
        unsafe {
            self.base_address.write_volatile(byte);
        }
        while self.read_flag_register() & FR_BUSY != 0 {}
    }

    fn read_flag_register(&self) -> u8 {
        unsafe {
            self.base_address.add(FLAG_REGISTER_OFFSET).read_volatile()
        }
    }
}

impl fmt::Write for Pl011 {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.as_bytes() {
            self.write_byte(*c);
        }
        Ok(())
    }
}

unsafe impl Send for Pl011 {}
