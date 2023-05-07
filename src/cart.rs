#[derive(Savefile)]
pub struct Cart {
    address: u32,
    rom: Vec<u8>,
}

impl Cart {
    pub fn new(rom: Vec<u8>) -> Self {
        Cart {
            address: 0,
            rom: rom,
        }
    }

    pub fn set_high(&mut self, val: u8) {
        self.address = (self.address & 0x00ffff) | ((val as u32) << 16);
    }

    pub fn set_mid(&mut self, val: u8) {
        self.address = (self.address & 0xff00ff) | ((val as u32) << 8);
    }

    pub fn set_low(&mut self, val: u8) {
        self.address = (self.address & 0xffff00) | ((val as u32) << 0);
    }

    pub fn read_byte(&self) -> u8 {
        self.rom[self.address as usize]
    }
}
