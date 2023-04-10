use crate::constants::{CPU_CLOCK, DISPLAY_WIDTH, SCREEN_WIDTH, SCREEN_HEIGHT};
use egui::Context;

const PAL_SQUARE_PX: usize = 16;

pub struct HD6845S {
    pub addr: u8,
    horiz_char_total: u8,
    horiz_disp: u8,
    horiz_sync_pos: u8,
    sync_width: u8,
    vert_char_total: u8,
    vert_total_adj: u8,
    pub vert_disp: u8,
    pub vert_sync_pos: u8,
    mode_control: u8,
    pub max_ras_addr: u8,
    cursor_start_ras: u8,
    cursor_end_ras: u8,
    disp_start_addr: u16,
    cursor_addr: u16,
}

impl HD6845S {
    pub fn new() -> Self {
        Self {
            addr: 0,
            horiz_char_total: 0,
            horiz_disp: 0,
            horiz_sync_pos: 0,
            sync_width: 0,
            vert_char_total: 0,
            vert_total_adj: 0,
            vert_disp: 0,
            vert_sync_pos: 0,
            mode_control: 0,
            max_ras_addr: 0,
            cursor_start_ras: 0,
            cursor_end_ras: 0,
            disp_start_addr: 0,
            cursor_addr: 0,
        }
    }

    pub fn set_addr(&mut self, value: u8) {
        match self.addr {
            0x0 => self.horiz_char_total = value,
            0x1 => self.horiz_disp = value,
            0x2 => self.horiz_sync_pos = value,
            0x3 => self.sync_width = value,
            0x4 => self.vert_char_total = value & 0x7f,
            0x5 => self.vert_total_adj = value & 0x1f,
            0x6 => self.vert_disp = value & 0x7f,
            0x7 => self.vert_sync_pos = value & 0x7f,
            0x8 => self.mode_control = value,
            0x9 => self.max_ras_addr = value & 0x1f,
            0xa => self.cursor_start_ras = value & 0x7f,
            0xb => self.cursor_end_ras = value & 0x1f,
            0xc => {
                self.disp_start_addr =
                    (((value as u16) & 0x3f) << 8) | (self.disp_start_addr & 0xff)
            }
            0xd => self.disp_start_addr = (self.disp_start_addr & 0xff00) | value as u16,
            0xe => self.cursor_addr = (((value as u16) & 0x3f) << 8) | (self.cursor_addr & 0xff),
            0xf => self.cursor_addr = (self.cursor_addr & 0xff00) | value as u16,
            _ => panic!("Setting hd6845s addr {:x}", self.addr),
        }
    }
}

pub struct Video {
    bitmapbank2: bool,
    bitmapdata0: [u8; 0xc000],
    bitmapdata1: [u8; 0xc000],
    pub hd6845s: HD6845S,
    palettes: [u32; 16], // rgba
    red_pal: u8,
    green_pal: u8,
    blue_pal: u8,
    pub pri: u8,
    pub avram: [u8; 0x800],
    pub tvram: [u8; 0x800],
    // ank: Vec<u8>,
    fnt: [u8; 0x1800],
    pcg_ram: [u8; 0x1800],
    pub cycles: u32,

    texture_handle: Option<egui::TextureHandle>,
    palettes_open: bool,
    palettes_canvas: [u8; 8*PAL_SQUARE_PX * 2*PAL_SQUARE_PX * 4],
    bitmap0_open: bool,
    bitmap0_canvas: [u8; (SCREEN_WIDTH * SCREEN_HEIGHT * 4) as usize],
    pcgrom_open: bool,
    pcgrom_canvas: [u8; 128 * 128 * 4],
    pcgram_open: bool,
    pcgram_canvas: [u8; 128 * 128 * 4],
}

fn draw_pixel(
    canvas: &mut [u8],
    canvas_width: u32,
    plotcol: i16,
    plotrow: i16,
    color: u32,
) {
    let offs = ((plotrow as usize) * canvas_width as usize + plotcol as usize) * 4;
    canvas[offs + 0] = ((color >> 24) & 0xff) as u8; // r
    canvas[offs + 1] = ((color >> 16) & 0xff) as u8; // g
    canvas[offs + 2] = ((color >> 8) & 0xff) as u8; // b
    canvas[offs + 3] = ((color >> 0) & 0xff) as u8; // a
}

fn draw_pcg_tile(
    palettes: [u32; 16],
    canvas: &mut [u8],
    canvas_width: u32,
    fnt: [u8; 0x1800],
    tile_idx: u8,
    row: u8,
    col: u8,
    pen_mask: u8,
    double_width: bool,
    double_height: bool,
    invert: bool,
) {
    // let xstart = ((self.hd6845s.horiz_char_total - self.hd6845s.horiz_sync_pos) as i16 * 8) / 2;
    // let ystart = ((self.hd6845s.vert_char_total - self.hd6845s.vert_sync_pos) as i16 * 8) / 2;

    for yi in 0..8 {
        for xi in 0..8 {
            let plotcol = (col as i16) * 8 + xi; // + xstart;
            let plotrow = (row as i16) * 8 + yi; // + ystart;

            let mut bit_to_check = xi;
            if double_width {
                bit_to_check /= 2;
                if col % 2 == 1 {
                    bit_to_check += 4;
                }
            }

            let mut yoffs = yi;
            if double_height {
                yoffs /= 2;
                if row % 2 == 1 {
                    yoffs += 4;
                }
            }

            // PCG read
            let tile_offset = ((tile_idx as usize) * 8) + yoffs as usize;

            let pen0 = fnt[tile_offset + 0x0000] >> (7 - bit_to_check) & (pen_mask & 1) >> 0;
            let pen1 = fnt[tile_offset + 0x0800] >> (7 - bit_to_check) & (pen_mask & 2) >> 1;
            let pen2 = fnt[tile_offset + 0x1000] >> (7 - bit_to_check) & (pen_mask & 4) >> 2;
            let mut pen_val = pen0 | (pen1 << 1) | (pen2 << 2);
            if invert {pen_val ^= 7}

            if pen_val != 0 {
                let color = palettes[pen_val as usize];
                draw_pixel(canvas, canvas_width, plotcol, plotrow, color);
            }
        }
    }
}

impl Video {
    pub fn new(
        // ank: Vec<u8>, 
        fnt: Vec<u8>,
    ) -> Self {
        let mut new_fnt = [0; 0x1800];
        for thing in 0..=2 {
            for i in 0..=0x7ff {
                new_fnt[i + thing * 0x800] = fnt[i];
            }
        }

        let palettes: [u32; 16] = [0xffffffff; 16];

        let mut pcgrom_canvas = [0; 128 * 128 * 4];
        for tilerow in 0..16 {
            for tilecol in 0..16 {
                let tile_idx = tilerow * 16 + tilecol;
                draw_pcg_tile(
                    palettes,
                    &mut pcgrom_canvas,
                    128,
                    new_fnt,
                    tile_idx,
                    tilerow,
                    tilecol,
                    7,
                    false, false,
                    false,
                );
            }
        }

        let mut video = Self {
            bitmapbank2: false,
            bitmapdata0: [0; 0xc000],
            bitmapdata1: [0; 0xc000],
            hd6845s: HD6845S::new(),
            palettes: [0; 16],
            red_pal: 0,
            green_pal: 0,
            blue_pal: 0,
            pri: 0,
            avram: [0; 0x800],
            tvram: [0; 0x800],
            // ank: ank,
            fnt: new_fnt,
            pcg_ram: [0; 0x1800],
            cycles: 0,

            texture_handle: None,
            palettes_open: false,
            palettes_canvas: [0; 8*PAL_SQUARE_PX * 2*PAL_SQUARE_PX * 4],
            bitmap0_open: false,
            bitmap0_canvas: [0; (SCREEN_WIDTH * SCREEN_HEIGHT * 4) as usize],
            pcgrom_open: false,
            pcgrom_canvas: pcgrom_canvas,
            pcgram_open: false,
            pcgram_canvas: [0; 128 * 128 * 4],
        };

        for i in 0..16 {
            let mut pal = 0xffu32;
            if i & 2 == 2 {
                pal += 0xff000000u32
            };
            if i & 4 == 4 {
                pal += 0x00ff0000u32
            };
            if i & 1 == 1 {
                pal += 0x0000ff00u32
            };
            video.palettes[i] = pal;
            let gridx = i % 8;
            let gridy = i / 8;
            video.draw_pal_square(gridx, gridy, pal);
        }
        video
    }

    fn draw_pal_square(&mut self, gridx: usize, gridy: usize, color: u32) {
        for y in 0..PAL_SQUARE_PX {
            for x in 0..PAL_SQUARE_PX {
                draw_pixel(
                    &mut self.palettes_canvas, 
                    (8*PAL_SQUARE_PX) as u32,
                    (gridx * PAL_SQUARE_PX + x) as i16,
                    (gridy * PAL_SQUARE_PX + y) as i16,
                    color,
                );
            }
        }
    }

    fn priority_mixer_pri(&self, color: u8) -> u8 {
        let mut pri_i = 0u8;
        let mut pri_mask_calc = 1u8;

        while pri_i < 7 {
            if (color & 7) == pri_i {
                break;
            }
    
            pri_i += 1;
            pri_mask_calc <<= 1;
        }
    
        pri_mask_calc
    }

    fn draw_gfxbitmap(&mut self, canvas: &mut [u8], xsize: u8, ysize: u8, pri: u8) {
        // bitmap
        for row in 0..ysize {
            for col in 0..xsize {
                for yi in 0..8 {
                    for xi in 0..8 {
                        let x = col as u16;
                        let y = row as u16;
                        let mut gfx_offset = ((x + (y * xsize as u16))
                            + (self.hd6845s.disp_start_addr & 0x3f00))
                            & 0x7ff;
                        gfx_offset += yi * 0x800;
                        let pen_b =
                            (self.bitmapdata0[gfx_offset as usize + 0x0000] >> (7 - xi)) & 1;
                        let pen_r =
                            (self.bitmapdata0[gfx_offset as usize + 0x4000] >> (7 - xi)) & 1;
                        let pen_g =
                            (self.bitmapdata0[gfx_offset as usize + 0x8000] >> (7 - xi)) & 1;

                        let color = pen_g << 2 | pen_r << 1 | pen_b << 0;

                        // checkout priority mixing
                        // TODO: due of this we loop twice, it should be handled at mixing time instead.
                        let pri_mask_val = self.priority_mixer_pri(color);
                        if (pri_mask_val & pri) != 0 {
                            continue;
                        }

                        // apply partial update
                        // TODO: not working properly, see top of file
                        // if(y*(mc6845_tile_height)+yi < cliprect.min_y || y*(mc6845_tile_height)+yi > cliprect.max_y)
                        //     continue;

                        // TODO: call a fn subset instead of looping for a width/height that is never hit

                        let plotcol = (x * 8 + xi) as i16;
                        let plotrow = (y * 8 + yi) as i16;
                        draw_pixel(
                            canvas,
                            DISPLAY_WIDTH,
                            plotcol, plotrow,
                            self.palettes[color as usize|8],
                        );
                        draw_pixel(
                            &mut self.bitmap0_canvas,
                            SCREEN_WIDTH,
                            plotcol, plotrow,
                            self.palettes[color as usize],
                        );
                    }
                }
            }
        }
    }

    fn draw_fgtilemap(&mut self, canvas: &mut [u8], xsize: u8, ysize: u8) {
        // tile row and tile col
        for row in 0..ysize {
            for col in 0..xsize {
                let tile_offs = (row as usize) * (xsize as usize) + col as usize;
                let tile_idx = self.tvram[tile_offs];

                let attr_byte = self.avram[tile_offs];
                let double_width = (attr_byte & 0x80) != 0;
                let double_height = (attr_byte & 0x40) != 0;
                let pcg_bank = (attr_byte & 0x20) != 0;
                // let blink = (attr_byte & 0x10) != 0;
                let invert = (attr_byte & 0x08) != 0;
                let color = attr_byte & 7;

                draw_pcg_tile(
                    self.palettes,
                    canvas,
                    DISPLAY_WIDTH,
                    if pcg_bank { self.pcg_ram } else { self.fnt },
                    tile_idx,
                    row,
                    col,
                    color,
                    double_width, double_height,
                    invert,
                );
            }
        }

        // pcgram ui
        for tilerow in 0..16 {
            for tilecol in 0..16 {
                let tile_idx = tilerow * 16 + tilecol;

                draw_pcg_tile(
                    self.palettes,
                    &mut self.pcgram_canvas,
                    128,
                    self.pcg_ram,
                    tile_idx,
                    tilerow,
                    tilecol,
                    7,
                    false, false,
                    false,
                );
            }
        }
    }

    pub fn display(&mut self, canvas: &mut [u8]) {
        let xsize = self.hd6845s.horiz_disp;
        let ysize = self.hd6845s.vert_disp;

        self.draw_gfxbitmap(canvas, xsize, ysize, self.pri);
        self.draw_fgtilemap(canvas, xsize, ysize);
        self.draw_gfxbitmap(canvas, xsize, ysize, self.pri^0xff);
    }

    pub fn get_bitmap_data(&self, addr: usize) -> u8 {
        match self.bitmapbank2 {
            false => self.bitmapdata0[addr],
            true => self.bitmapdata1[addr],
        }
    }

    pub fn set_bitmap_data(&mut self, addr: usize, value: u8) {
        match self.bitmapbank2 {
            false => self.bitmapdata0[addr] = value,
            true => self.bitmapdata1[addr] = value,
        };
    }

    pub fn recreate_bg_palettes(&mut self) {
        for i in 0..8usize {
            let r = ((self.red_pal >> i) & 1) as u32;
            let g = ((self.green_pal >> i) & 1) as u32;
            let b = ((self.blue_pal >> i) & 1) as u32;
            let color = r * 0xff000000 + g * 0x00ff0000 + b * 0x0000ff00 + 0xff;
            self.palettes[8 | i] = color;
            self.draw_pal_square(i, 1, color);
        }
    }

    pub fn set_red(&mut self, value: u8) {
        self.red_pal = value;
        self.recreate_bg_palettes();
    }

    pub fn set_green(&mut self, value: u8) {
        self.green_pal = value;
        self.recreate_bg_palettes();
    }

    pub fn set_blue(&mut self, value: u8) {
        self.blue_pal = value;
        self.recreate_bg_palettes();
    }

    pub fn pcg_w(&mut self, addr: u16, value: u8) {
        if addr == 0 {
            panic!("Writing to ANK area");
        }

        let mut y_char_size = if self.hd6845s.max_ras_addr + 1 > 8 {
            self.hd6845s.max_ras_addr + 1 - 8
        } else {
            self.hd6845s.max_ras_addr + 1
        };
        if y_char_size == 0 {
            y_char_size = 1;
        }
        let offs = self.get_pcg_addr(self.hd6845s.horiz_disp, y_char_size) as usize;
        let mut pcg_offset = self.tvram[offs] as u16 * 8;
        // println!("pcg_w {:x} {:x} {:x} {:x} {:x}", addr, value, self.hpos(), self.vpos(), pcg_offset);
        pcg_offset += self.vpos() & (y_char_size as u16 - 1);
        pcg_offset += (addr - 1) * 0x800;

        self.pcg_ram[pcg_offset as usize] = value;
    }

    fn get_pcg_addr(&self, width: u8, y_char_size: u8) -> u16 {
        let hbeam = self.hpos() >> 3;
        let vbeam = self.vpos() / (y_char_size as u16);
        ((hbeam + vbeam * (width as u16)) + (self.hd6845s.disp_start_addr & 0x3f00)) & 0x7ff
    }

    fn hpos(&self) -> u16 {
        let cyc_per_line: f64 = CPU_CLOCK as f64 / 264.0 / 60.0;
        let cyc_per_x = cyc_per_line / SCREEN_WIDTH as f64;
        ((self.cycles as f64 % cyc_per_line) / cyc_per_x) as u16
    }

    pub fn vpos(&self) -> u16 {
        let cyc_per_line: f64 = CPU_CLOCK as f64 / 264.0 / 60.0;
        (self.cycles as f64 / cyc_per_line) as u16
    }

    pub fn ui(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        ui.menu_button("Video", |ui| {
            if ui.button("Palettes").clicked() {
                self.palettes_open = true;
                ui.close_menu();
            }
            if ui.button("Bitmap 0").clicked() {
                self.bitmap0_open = true;
                ui.close_menu();
            }
            if ui.button("PCG ROM Viewer").clicked() {
                self.pcgrom_open = true;
                ui.close_menu();
            }
            if ui.button("PCG RAM Viewer").clicked() {
                self.pcgram_open = true;
                ui.close_menu();
            }
        });

        egui::Window::new("Palettes")
            .open(&mut self.palettes_open)
            .show(ctx, |ui| {
                let texture: &egui::TextureHandle = self.texture_handle.insert(ui.ctx().load_texture(
                    "palettes",
                    egui::ColorImage::from_rgba_unmultiplied(
                        [8*PAL_SQUARE_PX, 2*PAL_SQUARE_PX], 
                        &self.palettes_canvas,
                    ),
                    Default::default(),
                ));
                ui.image(texture, texture.size_vec2());
            });

        egui::Window::new("Bitmap 0")
            .open(&mut self.bitmap0_open)
            .show(ctx, |ui| {
                let texture: &egui::TextureHandle = self.texture_handle.insert(ui.ctx().load_texture(
                    "bitmap0",
                    egui::ColorImage::from_rgba_unmultiplied(
                        [SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize],
                        &self.bitmap0_canvas,
                    ),
                    Default::default(),
                ));
                ui.image(texture, texture.size_vec2());
                ui.label(format!("Bitmap 0 src: ${:04x}", self.hd6845s.disp_start_addr & 0x3f00));
                ui.label(format!("Horiz disp: {:02x}", self.hd6845s.horiz_disp));
                ui.label(format!("Vert disp: {:02x}", self.hd6845s.vert_disp));
                ui.label(format!("Pri: {:02x}", self.pri));
            });

        egui::Window::new("PCG ROM Viewer")
            .open(&mut self.pcgrom_open)
            .show(ctx, |ui| {
                let texture: &egui::TextureHandle = self.texture_handle.insert(ui.ctx().load_texture(
                    "pcgrom",
                    egui::ColorImage::from_rgba_unmultiplied(
                        [128, 128], 
                        &self.pcgrom_canvas,
                    ),
                    Default::default(),
                ));
                ui.image(texture, texture.size_vec2());
            });

        egui::Window::new("PCG RAM Viewer")
            .open(&mut self.pcgram_open)
            .show(ctx, |ui| {
                let texture: &egui::TextureHandle = self.texture_handle.insert(ui.ctx().load_texture(
                    "pcgram",
                    egui::ColorImage::from_rgba_unmultiplied(
                        [128, 128], 
                        &self.pcgram_canvas,
                    ),
                    Default::default(),
                ));
                ui.image(texture, texture.size_vec2());
            });
    }
}
