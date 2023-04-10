use crate::cart::Cart;
use crate::fdc::FDC;
use crate::gui::Framework;
use crate::i8255::I8255;
use crate::video::Video;
use crate::z80::{Z80_io, Z80};
use egui_winit::winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use std::fs::{metadata, File};
use std::io::Read;
use winit_input_helper::WinitInputHelper;

use crate::constants::{CPU_CLOCK, DISPLAY_HEIGHT, DISPLAY_WIDTH};

use crate::breakpoints::Breakpoints;
use crate::disassembler::Disassembler;
use crate::watchpoints::Watchpoints;

mod breakpoints;
mod cart;
mod constants;
mod disassembler;
mod fdc;
mod gui;
mod i8255;
mod old_z80;
mod video;
mod watchpoints;
mod z80;

pub struct IO {
    pub mem: [u8; 0x10000],
    ipl_loaded: bool,
    ipl: [u8; 0x1000],
    io_bank: bool,
    video: Video,
    i8255: I8255,
    fdc: FDC,
    cart: Cart,
    sub_cmd: u8,
    sub_cmd_len: u8,
    sub_vals: [u8; 8],
    sub_obf: u8,
    key_i: usize,
    sub_val_ptr: usize,
    key_irq_vector: u8,

    last_addr: u16,
    last_is_read: bool,
    last_is_mem: bool,
}

impl Z80_io for IO {
    fn read_byte(&mut self, addr: u16) -> u8 {
        self.last_addr = addr;
        self.last_is_read = true;
        self.last_is_mem = true;

        match self.ipl_loaded {
            true => match addr {
                0..=0xfff => self.ipl[addr as usize],
                0x1000..=0x7fff => 0,
                _ => self.mem[addr as usize],
            },
            false => self.mem[addr as usize],
        }
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        self.last_addr = addr;
        self.last_is_read = false;
        self.last_is_mem = true;

        self.mem[addr as usize] = value;
    }

    fn port_in(&mut self, addr: u16) -> u8 {
        self.last_addr = addr;
        self.last_is_read = true;
        self.last_is_mem = false;

        if self.io_bank {
            // todo: get extra gfx bitmap ram value
            self.io_bank = false;
            0
        } else {
            match addr {
                0x0000 => 0, // todo: Sofia and Brain Breaker need this?
                0x0e03 => self.cart.read_byte(),
                0x0ff8 => self.fdc.status(),
                0x0ffa => 0,
                // 0x0ffa => self.fdc.sector,
                0x0ffb => self.fdc.data,
                0x1900 => {
                    if self.sub_obf != 0 {
                        let ret = self.sub_vals[self.key_i];
                        self.key_i += 1;
                        if self.key_i >= 2 {
                            self.key_i = 0;
                        }

                        ret
                    } else {
                        self.sub_cmd_len -= 1;
                        match self.sub_cmd_len {
                            0 => self.sub_obf = 0x20,
                            _ => self.sub_obf = 0x00,
                        }
                        let ret = self.sub_vals[self.sub_val_ptr];
                        self.sub_val_ptr += 1;
                        if self.sub_cmd_len <= 0 {
                            self.sub_val_ptr = 0;
                        }

                        ret
                    }
                }
                0x1a01 => {
                    /*
                    x--- ---- "v disp"
                    -x-- ---- "sub cpu ibf"
                    --x- ---- "sub cpu obf"
                    ---x ---- ROM/RAM flag (0=ROM, 1=RAM)
                    ---- x--- "busy" <- allow printer data output
                    ---- -x-- "v sync"
                    ---- --x- "cmt read"
                    ---- ---x "cmt test" (active low) <- actually this is "Sub CPU detected BREAK"
                    */
                    let tile_height = self.video.hd6845s.max_ras_addr as u16 + 1;
                    let vblank_line = self.video.hd6845s.vert_disp as u16 * tile_height;
                    let vsync_line = self.video.hd6845s.vert_sync_pos as u16 * tile_height;
                    let m_vdisp = if self.video.vpos() < vblank_line {
                        0x80
                    } else {
                        0x00
                    };
                    let m_vsync = if self.video.vpos() < vsync_line { 0 } else { 4 };
                    let m_ram_bank = 0;

                    let res = m_ram_bank | self.sub_obf | m_vsync | m_vdisp;

                    // if(m_cassette->input() > 0.03)
                    //     res |= 0x02;

                    // CMT test bit is set low when the CMT Stop command is issued, and becomes
                    // high again when this bit is read.
                    // res |= 0x01;
                    // if(m_cmt_test != 0)
                    // {
                    //     m_cmt_test = 0;
                    //     res &= ~0x01;
                    // }

                    res
                }
                0x1a02 => {
                    /*
                    x--- ---- Printer port output
                    -x-- ---- 320 mode (r/w), divider for the pixel clock
                    --x- ---- i/o mode (r/w)
                    ---x ---- smooth scroll enabled (?)
                    ---- ---x cassette output data
                    */
                    self.i8255.port_c
                }
                0x1b00 => {
                    // ay sound
                    // println!("Read from port 1b00");
                    0
                }
                0x2000..=0x27ff => self.video.avram[addr as usize - 0x2000],
                0x2800..=0x2fff => self.video.avram[addr as usize - 0x2800],
                0x3000..=0x37ff => self.video.tvram[addr as usize - 0x3000],
                0x3800..=0x3fff => self.video.tvram[addr as usize - 0x3800],
                0x4000..=0xffff => self.video.get_bitmap_data((addr - 0x4000) as usize),
                _ => {
                    panic!("Implement port in addr {:x}", addr);
                }
            }
        }
    }

    fn port_out(&mut self, addr: u16, value: u8) {
        self.last_addr = addr;
        self.last_is_read = false;
        self.last_is_mem = false;

        if self.io_bank {
            // todo: extra gfx bitmap ram
        } else {
            match addr {
                0x0e00 => self.cart.set_high(value),
                0x0e01 => self.cart.set_mid(value),
                0x0e02 => self.cart.set_low(value),
                0x0ff8 => self.fdc.cmd(value),
                0x0ff9 => self.fdc.track = value,
                0x0ffa => self.fdc.sector = value,
                0x0ffb => self.fdc.data = value,
                0x0ffc => self.fdc.set_floppy(value),
                0x1000..=0x10ff => self.video.set_blue(value),
                0x1100..=0x11ff => self.video.set_red(value),
                0x1200..=0x12ff => self.video.set_green(value),
                0x1300 => self.video.pri = value,
                0x1400..=0x17ff => self.video.pcg_w((addr & 0x300) >> 8, value),
                0x1800 => self.video.hd6845s.addr = value & 0x1f,
                0x1801 => self.video.hd6845s.set_addr(value),
                0x1900 => {
                    let mut data = value;
                    if self.sub_cmd == 0xe4 {
                        self.key_irq_vector = value;
                        data = 0;
                    }
                    if self.sub_cmd == 0xe9 {
                        // todo: cmt command
                    }
                    match data {
                        0xe4 => {}
                        0xe6 => {
                            // todo: keyboard
                            self.sub_vals[0] = 0xff;
                            self.sub_vals[1] = 0xff;
                            self.sub_cmd_len = 2;
                        }
                        0xe7 => {} // todo: tv control
                        0xe8 => {
                            self.sub_vals[0] = self.sub_cmd;
                            self.sub_cmd_len = 1;
                        }
                        0xe9 => {}
                        0xeb => {
                            // todo: cmt
                            self.sub_vals[0] = 5;
                            self.sub_cmd_len = 1;
                        }
                        0x00 => {} // todo: frig to allow 0xe4
                        0x04 => {} // todo: frig for 0xe7 cmd
                        _ => {
                            panic!("Implement sub cmd {:x} {:x}", self.sub_cmd, data);
                        }
                    }
                    self.sub_cmd = data;

                    match self.sub_cmd_len {
                        0 => self.sub_obf = 0x20,
                        _ => self.sub_obf = 0x00,
                    }
                }
                0x1a02 => {
                    let prev_portc = self.i8255.port_c;
                    self.i8255.port_c &= 0xdf;
                    self.i8255.port_c |= value & 0x20;
                    if (self.i8255.port_c & 0x20) == 0 && (prev_portc & 0x20) != 0 {
                        self.io_bank = true;
                    }
                }
                0x1a03 => {
                    let prev_portc = self.i8255.port_c;
                    self.i8255.set_ctrl(value);
                    if (self.i8255.port_c & 0x20) == 0 && (prev_portc & 0x20) != 0 {
                        self.io_bank = true;
                    }
                },
                0x1b00 => {
                    // ay sound
                    // println!("Write from port 1b00 val {:x}", value);
                }
                0x1c00 => {
                    // ay sound
                    // println!("Write from port 1c00 val {:x}", value);
                }
                0x1d00..=0x1dff => self.ipl_loaded = true,
                0x1e00 => self.ipl_loaded = false,
                0x2000..=0x27ff => self.video.avram[addr as usize - 0x2000] = value,
                0x2800..=0x2fff => self.video.avram[addr as usize - 0x2800] = value,
                0x3000..=0x37ff => self.video.tvram[addr as usize - 0x3000] = value,
                0x3800..=0x3fff => self.video.tvram[addr as usize - 0x3800] = value,
                0x4000..=0xffff => self.video.set_bitmap_data((addr - 0x4000) as usize, value),
                _ => {
                    panic!("Implement port out addr {:x} value {:x}", addr, value);
                }
            }
        }
    }
}

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn main() -> Result<(), Error> {
    let ipl = get_file_as_byte_vec(&String::from("res/ipl.x1"));
    // let ank = get_file_as_byte_vec(&String::from("res/ank.fnt")); // 8x16
    let fnt = get_file_as_byte_vec(&String::from("res/fnt0808.x1")); // 8x8
    let cart_rom = get_file_as_byte_vec(&String::from("res/spaceBurger.bin"));
    // let cart_rom = vec![0];
    // let floppy_data = get_file_as_byte_vec(&String::from("res/cz8cb01.2d"));
    let floppy_data = vec![0; 327680];

    let mut cpu = Z80::new(IO {
        mem: [0; 0x10000],
        ipl_loaded: true,
        ipl: [0; 0x1000],
        io_bank: false,
        video: Video::new(
            // ank, 
            fnt,
        ),
        i8255: I8255::new(),
        fdc: FDC::new(floppy_data.try_into().ok().unwrap()),
        cart: Cart::new(cart_rom),
        sub_cmd: 0,
        sub_cmd_len: 0,
        sub_vals: [0; 8],
        sub_obf: 0,
        key_i: 0,
        sub_val_ptr: 0,
        key_irq_vector: 0,

        last_addr: 0xffff,
        last_is_mem: true,
        last_is_read: true,
    });

    for (i, byte) in ipl.iter().enumerate() {
        cpu.io.ipl[i] = *byte;
    }

    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let xscale: f64 = 2.5;
        let yscale = xscale * 2.0;
        let size = LogicalSize::new(DISPLAY_WIDTH as f64, DISPLAY_HEIGHT as f64);
        let scaled_size = LogicalSize::new(
            DISPLAY_WIDTH as f64 * xscale,
            DISPLAY_HEIGHT as f64 * yscale,
        );
        WindowBuilder::new()
            .with_title("Sharp X1 Emulator")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(DISPLAY_WIDTH, DISPLAY_HEIGHT, surface_texture)?
    };

    let window_size = window.inner_size();
    let scale_factor = window.scale_factor() as f32;
    let mut framework = Framework::new(
        &event_loop,
        window_size.width,
        window_size.height,
        scale_factor,
        &pixels,
    );

    let mut paused = true;

    let mut breakpoints = Breakpoints::new();
    let mut disassembler = Disassembler::new();
    let mut watchpoints = Watchpoints::new();

    let mut cyc = 0u32;

    event_loop.run(move |event, _, control_flow| {
        // For everything else, for let winit_input_helper collect events to build its state.
        // It returns `true` when it is time to update our game state and request a redraw.
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape)
                || input.close_requested()
                || input.destroyed()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::Space) {
                paused = !paused;
            }

            if input.key_pressed_os(VirtualKeyCode::F5) {
                if !paused {
                    paused = true;
                } else {
                    cpu.step();
                }
            }

            // Update the scale factor
            if let Some(scale_factor) = input.scale_factor() {
                framework.scale_factor(scale_factor);
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    error!("pixels.resize_surface() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                framework.resize(size.width, size.height);
            }

            while !paused && cyc < CPU_CLOCK / 60 {
                let added = cpu.step();
                cyc += added;
                cpu.io.video.cycles += added;

                paused = breakpoints.check(cpu.pc);
                if !paused {
                    paused = watchpoints.check(cpu.io.last_addr, cpu.io.last_is_read, cpu.io.last_is_mem);
                }
            }

            if cyc >= CPU_CLOCK / 60 {
                cyc -= CPU_CLOCK / 60;
                cpu.io.video.cycles -= CPU_CLOCK / 60;
            }

            cpu.io.video.display(pixels.frame_mut());
            window.request_redraw();
        }

        match event {
            Event::WindowEvent { event, .. } => {
                // Update egui inputs
                framework.handle_event(&event);
            }
            Event::RedrawRequested(_) => {
                // Prepare egui
                disassembler.prepare(&mut cpu);
                framework.prepare(
                    &window, 
                    &mut cpu,
                    &disassembler, 
                    &mut breakpoints,
                    &mut watchpoints,
                );

                // Render everything together
                let render_result = pixels.render_with(|encoder, render_target, context| {
                    // Render the world texture
                    context.scaling_renderer.render(encoder, render_target);

                    // Render egui
                    framework.render(encoder, render_target, context);

                    Ok(())
                });

                // Basic error handling
                if let Err(err) = render_result {
                    error!("pixels.render() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => (),
        }
    });
}
