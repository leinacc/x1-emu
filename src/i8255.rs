#[derive(PartialEq, Savefile)]
pub enum I8255OpMode {
    BSR,
    IO,
}

#[derive(Savefile)]
pub enum I8255IOMode {
    Simple,
    Strobed,
    // StrobedBiDirectional,
}

#[derive(Savefile)]
pub struct I8255 {
    op_mode: I8255OpMode,
    group_b_io_mode: I8255IOMode,
    pub port_c: u8,
}

impl I8255 {
    pub fn new() -> Self {
        I8255 {
            op_mode: I8255OpMode::BSR,
            group_b_io_mode: I8255IOMode::Simple,
            port_c: 0,
        }
    }

    pub fn set_ctrl(&mut self, value: u8) {
        // I8255 ctrl
        match value & 0x80 {
            0 => self.op_mode = I8255OpMode::BSR,
            _ => self.op_mode = I8255OpMode::IO,
        }
        if self.op_mode == I8255OpMode::BSR {
            let port_c_reg = (value & 0xe) >> 1;
            let set = (value & 1) << port_c_reg;
            let mask = 0xff - (1u8 << port_c_reg);
            self.port_c = (self.port_c & mask) | set;
        } else {
            match value & 0x02 {
                0 => self.group_b_io_mode = I8255IOMode::Simple,
                _ => self.group_b_io_mode = I8255IOMode::Strobed,
            }
            if (value & 0x7d) != 0 {
                panic!("Implement port out addr 1a03 value {:x}", value);
            }
        }
    }
}
