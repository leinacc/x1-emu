use egui::Context;

pub struct FDC {
    loaded: bool,
    pub sector: u8,
    side1: bool,
    floppy_bay_select: u8,
    offs_in_sector: u16,
    pub data: u8,
    reading: bool,
    disk_data: [u8; 327680],
    pub track: u8,

    status_open: bool,
}

impl FDC {
    pub fn new(disk_data: [u8; 327680], loaded: bool) -> Self {
        Self {
            loaded: loaded,
            sector: 0,
            side1: false,
            floppy_bay_select: 0,
            offs_in_sector: 0,
            data: 0,
            reading: false,
            disk_data: disk_data,
            track: 0,

            status_open: false,
        }
    }

    pub fn status(&mut self) -> u8 {
        /*
         * bit 7: clear if 'ready'
         * bit 1: clear if 'seeking to byte'
         * bit 0: clear once done reading sector
         */
        let mut ret = 2;
        if self.reading {
            if self.offs_in_sector == 0x100 {
                self.reading = false;
                self.offs_in_sector = 0;
            } else {
                let sector = (self.sector as usize)
                    + (if self.side1 { 0x10 } else { 0 })
                    + (self.track as usize) * 0x20;
                self.data = self.disk_data[sector * 0x100 + self.offs_in_sector as usize - 0x100];
                self.offs_in_sector += 1;
                ret |= 1;
            }
        }
        ret
    }

    pub fn cmd(&mut self, val: u8) {
        /*
         * $00 - restore
         * $10 - seek
         * $80 - read sector start
         * bit 3: alt sector size
         * bit 2: wait stable
         * bit 1: step times 2
         */
        match val & 0xf0 {
            0x00 => {
                // todo: restore
                self.offs_in_sector = 0;
            }
            0x10 => {
                // todo: seek
                self.track = self.data;
            }
            0x80 => {
                // read sector start
                self.reading = true;
            }
            _ => panic!("cmd {:02x}", val),
        }
    }

    pub fn get_sector(&self) -> u8 {
        if self.loaded {
            self.sector
        } else {
            0
        }
    }

    pub fn set_floppy(&mut self, val: u8) {
        /*
         * bit 7: set if stopping motor, clear if starting
         * bit 4: set if side 1, clear if side 0
         * bits 0-1: floppy bay selected
         */
        self.side1 = (val & 0x10) != 0;
        self.floppy_bay_select = val & 3;
    }

    pub fn ui(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        ui.menu_button("FDC", |ui| {
            if ui.button("Status").clicked() {
                self.status_open = true;
                ui.close_menu();
            }
        });

        egui::Window::new("Status")
            .open(&mut self.status_open)
            .show(ctx, |ui| {
                ui.label(format!("Floppy selected: {}", self.floppy_bay_select));
                ui.label(format!("Track: {:02x}", self.track));
                ui.label(format!("Side: {}", if self.side1 { "B" } else { "A" }));
                ui.label(format!("Sector: {:02x}", self.sector));
                ui.label(format!("Offs in sector: {:02x}", self.offs_in_sector));
            });
    }
}
