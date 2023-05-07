use crate::cart::Cart;
use crate::fdc::FDC;
use crate::gui::Framework;
use crate::i8255::I8255;
use crate::keyboard::Keyboard;
use crate::rtc::RTC;
use crate::video::Video;
use crate::z80::{Z80, Z80IO};

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

#[macro_use]
extern crate savefile_derive;

mod breakpoints;
mod cart;
mod constants;
mod disassembler;
mod fdc;
mod gui;
mod i8255;
mod keyboard;
mod rtc;
mod tests;
mod video;
mod watchpoints;
mod z80;

#[derive(Savefile)]
pub struct IO {
    mem: [u8; 0x10000],
    ipl_loaded: bool,
    ipl: [u8; 0x1000],
    io_bank: bool,
    video: Video,
    i8255: I8255,
    fdc: FDC,
    cart: Cart,
    rtc: RTC,
    sub_cmd: u8,
    sub_cmd_len: u8,
    sub_vals: [u8; 8],
    sub_obf: u8,
    key_i: usize,
    sub_val_ptr: usize,
    key_irq_vector: u8,

    keyboard: Keyboard,
    last_key_press: u8,

    last_addr: u16,
    last_is_read: bool,
    last_is_mem: bool,
    paused: bool,
    pause_pressed: bool,
    step_pressed: bool,
    reset_pressed: bool,
}

impl IO {
    fn new(ipl: Vec<u8>, fnt: Vec<u8>, floppy_data: Vec<u8>, cart_rom: Vec<u8>) -> Self {
        let mut io = Self {
            mem: [0; 0x10000],
            ipl_loaded: true,
            ipl: [0; 0x1000],
            io_bank: false,
            video: Video::new(
                // ank,
                fnt,
            ),
            i8255: I8255::new(),
            fdc: FDC::new(floppy_data.try_into().ok().unwrap(), true),
            cart: Cart::new(cart_rom),
            rtc: RTC::new(),
            sub_cmd: 0,
            sub_cmd_len: 0,
            sub_vals: [0; 8],
            sub_obf: 0,
            key_i: 0,
            sub_val_ptr: 0,
            key_irq_vector: 0,
    
            keyboard: Keyboard::new(),
            last_key_press: 0,
    
            last_addr: 0xffff,
            last_is_mem: true,
            last_is_read: true,
            paused: true,
            pause_pressed: false,
            step_pressed: false,
            reset_pressed: false,
        };
        for (i, byte) in ipl.iter().enumerate() {
            io.ipl[i] = *byte;
        }

        io
    }
}

impl Z80IO for IO {
    fn peek_byte(&mut self, addr: u16, _: bool) -> u8 {
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

    fn write_byte(&mut self, addr: u16, value: u8, side_effects: bool) {
        self.last_addr = addr;
        self.last_is_read = false;
        self.last_is_mem = true;
        if !side_effects {
            return;
        }

        self.mem[addr as usize] = value;
    }

    fn peek_io(&mut self, addr: u16, side_effects: bool) -> u8 {
        self.last_addr = addr;
        self.last_is_read = true;
        self.last_is_mem = false;

        if self.io_bank {
            // todo: get extra gfx bitmap ram value
            if side_effects {
                self.io_bank = false;
            }
            0
        } else {
            match addr {
                0x0000 => 0, // todo: Sofia and Brain Breaker need this?
                0x0e03 => self.cart.read_byte(),
                0x0ff8 => self.fdc.status(side_effects),
                0x0ffa => self.fdc.get_sector(),
                0x0ffb => self.fdc.data,
                0x1900 => {
                    if self.sub_obf != 0 {
                        let ret = self.sub_vals[self.key_i];
                        if !side_effects {
                            return ret;
                        }
                        self.key_i += 1;
                        if self.key_i >= 2 {
                            self.key_i = 0;
                        }

                        ret
                    } else {
                        let ret = self.sub_vals[self.sub_val_ptr];
                        if !side_effects {
                            return ret;
                        }
                        self.sub_cmd_len -= 1;
                        match self.sub_cmd_len {
                            0 => self.sub_obf = 0x20,
                            _ => self.sub_obf = 0x00,
                        }
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
                0x1ff0 => {
                    // todo: is for x1 turbo
                    0xff
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

    fn write_io(&mut self, addr: u16, value: u8, side_effects: bool) {
        self.last_addr = addr;
        self.last_is_read = false;
        self.last_is_mem = false;
        if !side_effects {
            return;
        }

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
                    if self.sub_cmd == 0xe7 {
                        println!("Setting TV ctrl: {:02x}", data);
                    }
                    if self.sub_cmd == 0xe9 {
                        // todo: CMT command
                        data = 0;
                    }
                    if (data & 0xf0) == 0xd0 {
                        // todo: TV-related
                        self.sub_vals[0] = 0;
                        self.sub_vals[1] = 0;
                        self.sub_vals[2] = 0;
                        self.sub_vals[3] = 0;
                        self.sub_vals[4] = 0;
                        self.sub_vals[5] = 0;
                        self.sub_cmd_len = 6;
                    }
                    match data {
                        0xe3 => {
                            panic!("Implement sub cmd {:x} {:x}", self.sub_cmd, data);
                        }
                        0xe4 => {
                            // Key IRQ vector set above
                        }
                        0xe5 => {
                            panic!("Implement sub cmd {:x} {:x}", self.sub_cmd, data);
                        }
                        0xe6 => {
                            self.sub_vals[1] = self.keyboard.check_press() as u8;
                            self.sub_vals[0] = self.keyboard.check_shift();
                            self.sub_cmd_len = 2;
                        }
                        0xe7 => {
                            // todo: unknown TV ctrl
                        }
                        0xe8 => {
                            // todo: TV ctrl read-out
                            self.sub_vals[0] = self.sub_cmd;
                            self.sub_cmd_len = 1;
                        }
                        0xe9 => {
                            // todo: CMT ctrl
                        }
                        0xea => {
                            panic!("Implement sub cmd {:x} {:x}", self.sub_cmd, data);
                        }
                        0xeb => {
                            // todo: CMT tape status
                            self.sub_vals[0] = 5;
                            self.sub_cmd_len = 1;
                        }
                        0xec => {
                            panic!("Implement sub cmd {:x} {:x}", self.sub_cmd, data);
                        }
                        0xed => {
                            self.sub_vals[0] = self.rtc.day;
                            self.sub_vals[1] = (self.rtc.month << 4) | (self.rtc.weekday & 0xf);
                            self.sub_vals[2] = self.rtc.year;
                            self.sub_cmd_len = 3;
                        }
                        0xee => {
                            panic!("Implement sub cmd {:x} {:x}", self.sub_cmd, data);
                        }
                        0xef => {
                            self.sub_vals[0] = self.rtc.hour;
                            self.sub_vals[1] = self.rtc.minute;
                            self.sub_vals[2] = self.rtc.second;
                            self.sub_cmd_len = 3;
                        }
                        _ => (),
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
                }
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
                0x1fd0 => {
                    // todo: is x1 turbo
                }
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

fn get_new_io() -> IO {
    let ipl = get_file_as_byte_vec(&String::from("res/ipl.x1"));
    // let ank = get_file_as_byte_vec(&String::from("res/ank.fnt")); // 8x16
    let fnt = get_file_as_byte_vec(&String::from("res/fnt0808.x1")); // 8x8
                                                                     // let cart_rom = get_file_as_byte_vec(&String::from("res/spaceBurger.bin"));
    let cart_rom = vec![0];
    let floppy_data = get_file_as_byte_vec(&String::from("res/cz8cb01.2d"));
    // let floppy_data = vec![0; 327680];

    IO::new(ipl, fnt, floppy_data, cart_rom)
}

fn main() -> Result<(), Error> {
    let mut io = get_new_io();

    // Backup CPU used to trip breakpoints/watchpoints without causing side effects
    let mut cpu = Z80::new(true);
    let mut backup_cpu = Z80::new(false);
    cpu.reset();
    backup_cpu.reset();

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

            if io.pause_pressed {
                io.pause_pressed = false;
                io.paused = !io.paused;
            }

            if io.step_pressed {
                io.step_pressed = false;
                if !io.paused {
                    io.paused = true;
                } else {
                    backup_cpu.step(&mut io);
                    let added = cpu.step(&mut io);
                    cyc += added;
                    io.video.cycles += added;
                }
            }

            if io.reset_pressed {
                io.reset_pressed = false;
                backup_cpu = Z80::new(false);
                cpu = Z80::new(true);
                backup_cpu.reset();
                cpu.reset();
                io = get_new_io();
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

            while !io.paused && cyc < CPU_CLOCK / 60 {
                backup_cpu.step(&mut io);

                io.paused = watchpoints.check(io.last_addr, io.last_is_read, io.last_is_mem);
                if io.paused {
                    backup_cpu = cpu.clone();
                    backup_cpu.side_effects = false;
                    break;
                }

                let added = cpu.step(&mut io);
                cyc += added;
                io.video.cycles += added;

                io.paused = breakpoints.check(backup_cpu.pc);
                if io.paused {
                    break;
                }
            }

            if cyc >= CPU_CLOCK / 60 {
                cyc -= CPU_CLOCK / 60;
                io.video.cycles -= CPU_CLOCK / 60;

                io.keyboard.set_btns_pressed(&input);
                if io.key_irq_vector != 0 {
                    if io.keyboard.key_pressed != io.last_key_press {
                        io.sub_vals[1] = io.keyboard.check_press() as u8;
                        io.sub_vals[0] = io.keyboard.check_shift();
                        io.sub_cmd_len = 2;
                        cpu.assert_irq(io.key_irq_vector);
                        io.sub_cmd = 0xe6;
                        io.sub_obf = 0x00;
                    }
                    io.last_key_press = io.keyboard.key_pressed;
                }
            }

            io.video.display(pixels.frame_mut());
            window.request_redraw();
        }

        match event {
            Event::WindowEvent { event, .. } => {
                // Update egui inputs
                framework.handle_event(&event);
            }
            Event::RedrawRequested(_) => {
                // Prepare egui
                disassembler.prepare(&mut cpu, &mut io);
                framework.prepare(
                    &window,
                    &mut cpu,
                    &mut io,
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
