use egui::{Color32, RichText, TextStyle, Ui};
use crate::z80::{Z80, Z80_io};

const ADDRESS_TEXT_COLOR: Color32 = Color32::from_rgb(125, 0, 125);
const WHITE_COLOR: Color32 = Color32::from_rgb(0xff, 0xff, 0xff);
const FADE_COLOR: Color32 = Color32::from_rgb(0x55, 0x55, 0x55);
const MNEM_COLOR: Color32 = Color32::from_rgb(0x00, 0x55, 0xaa);
const REG_COLOR: Color32 = Color32::from_rgb(0xaa, 0xaa, 0x00);
const MONOSPACE: TextStyle = TextStyle::Monospace;

struct Token {
    color: Color32,
    text: String,
}

pub struct Disassembler {
    lines: Vec<Vec<Token>>,
}

fn get_tokens(cpu: &mut Z80<crate::IO>, start_pc: u16, is_first: bool) -> (Vec<Token>, u16) {
    let mut pc = start_pc;
    let mut ret = vec![];

    // 1st token: the address
    ret.push(Token {
        color: ADDRESS_TEXT_COLOR,
        text: format!("{:04X}", start_pc),
    });

    // 2nd set of tokens: up to 3 bytes used for the instruction
    let mut bytes: Vec<u8> = vec![];
    let op = cpu.io.read_byte(pc);
    pc += 1;
    bytes.push(op);
    
    // 3rd set of tokens: the instruction and params
    let mut ins_tokens: Vec<Token> = vec![];
    let mut param1: u8 = 0;
    let mut param2: u8 = 0;
    let mnemonic = match op {
        0x8e => "adc",
        0x09 | 0x19 | 0x29 | 0x80 | 0x83..=0x85 | 0x87 | 0xc6 => "add",
        0xa0 | 0xa2..=0xa5 | 0xa7 | 0xe6 => "and",
        0xc4 | 0xcc | 0xcd | 0xd4 => "call",
        0x3f => "ccf",
        0xb8 | 0xbb..=0xbe | 0xfe => "cp",
        0x2f => "cpl",
        0x05 | 0x0b | 0x0d | 0x15 | 0x1b | 0x1d | 0x25 | 0x2b | 0x2d | 0x35 | 0x3d => "dec",
        0xf3 => "di",
        0x10 => "djnz",
        0xfb => "ei",
        0x08 | 0xe3 | 0xeb => "ex",
        0xd9 => "exx",
        0xdb => "in",
        0x03 | 0x04 | 0x0c | 0x13 | 0x14 | 0x1c | 0x23 | 0x24 | 0x2c | 0x34 | 0x3c => "inc",
        0x01 | 0x02 | 0x06 | 0x0a | 0x0e | 0x11 | 0x12 | 0x16 | 0x1a | 0x1e | 0x21 | 0x22
            | 0x26 | 0x2a | 0x2e | 0x31 | 0x32 | 0x36 | 0x3a | 0x3e | 0x40 | 0x42 | 0x44
            | 0x46..=0x48 | 0x4a..=0x50 | 0x53 | 0x55..=0x57 | 0x59..=0x5b | 0x5d..=0x69
            | 0x6c | 0x6f..=0x73 | 0x75 | 0x77..=0x7e => "ld",
        0x00 => "nop",
        0xc2 | 0xc3 | 0xca | 0xda | 0xe9 | 0xf2 | 0xfa => "jp",
        0x18 | 0x20 | 0x28 | 0x30 | 0x38 => "jr",
        0xb1 | 0xb3 | 0xb5 | 0xb7 | 0xf6 => "or",
        0xc1 | 0xd1 | 0xe1 | 0xf1 => "pop",
        0xc5 | 0xd5 | 0xe5 | 0xf5 => "push",
        0xc0 | 0xc8 | 0xc9 | 0xd0 | 0xd8 | 0xf0 => "ret",
        0x17 => "rla",
        0x07 => "rlca",
        0x1f => "rra",
        0x0f => "rrca",
        0xc7 | 0xcf | 0xff => "rst",
        0x9b | 0x9f => "sbc",
        0x37 => "scf",
        0x90 | 0x92 | 0x93 | 0x96 | 0xd6 => "sub",
        0xaa | 0xab | 0xad | 0xaf | 0xee => "xor",

        0xcb => {
            param1 = cpu.io.read_byte(pc);
            pc += 1;
            bytes.push(param1);
            match param1 {
                0x6c | 0x70 | 0x74 | 0x77 => "bit",
                0x12 => "rl",
                0x07 => "rlc",
                0x1a => "rr",
                0xa0 | 0xa4 => "res",
                0xc7 | 0xcf | 0xd7 | 0xdf | 0xe0 | 0xe4 | 0xe7 | 0xf7 | 0xff => "set",
                0x3f => "srl",
                _ => {
                    if is_first {
                        panic!("implement mnem for cb {:x} at {:04x}", param1, cpu.pc);
                    }
                    "???"
                },
            }
        }

        0xdd => {
            param1 = cpu.io.read_byte(pc);
            pc += 1;
            bytes.push(param1);
            match param1 {
                0x19 => "add",
                0x4e | 0x7e => "ld",
                0xe1 => "pop",
                0xe5 => "push",

                0xcb => {
                    param2 = cpu.io.read_byte(pc);
                    pc += 1;
                    bytes.push(param2);
                    match param2 {
                        0x06 => "rlc",
                        _ => "???",
                    }
                }
                _ => "???",
            }
        }

        0xed => {
            param1 = cpu.io.read_byte(pc);
            pc += 1;
            bytes.push(param1);
            match param1 {
                0x44 => "neg",
                0x49 | 0x51 | 0x59 | 0x61 | 0x69 | 0x79 => "out",
                0x4b | 0x53 | 0x5b | 0x5f | 0x73 | 0x7b => "ld",
                0xb0 => "ldir",
                0x42 | 0x52 | 0x62 => "sbc",
                0x68 | 0x78 => "in",
                _ => {
                    if is_first {
                        panic!("implement mnem for ed {:x} at {:04x}", param1, cpu.pc);
                    }
                    "???"
                },
            }
        }

        0xfd => {
            param1 = cpu.io.read_byte(pc);
            pc += 1;
            bytes.push(param1);
            match param1 {
                0xe1 => "pop",
                0xe5 => "push",
                _ => "???",
            }
        }

        _ => {
            if is_first {
                panic!("implement mnem for {:x} at {:04x}", op, cpu.pc);
            }
            "???"
        },
    };

    // Registers before a value
    let initial_reg = match op {
        0x3c | 0x3d | 0xa7 | 0xaf | 0xb7 => Some("a"),
        0x3a | 0x3e | 0xc6 | 0xdb => Some("a,"),
        0x87 | 0x9f => Some("a, a"),
        0x78 | 0x80 => Some("a, b"),
        0x79 => Some("a, c"),
        0x7a => Some("a, d"),
        0x7b | 0x83 | 0x9b => Some("a, e"),
        0x7c | 0x84 => Some("a, h"),
        0x7d | 0x85 => Some("a, l"),
        0x0a => Some("a, (bc)"),
        0x1a => Some("a, (de)"),
        0x7e | 0x8e => Some("a, (hl)"),
        0xf1 | 0xf5 => Some("af"),
        0x08 => Some("af, af'"),

        0x04 | 0x05 | 0x90 | 0xa0 | 0xb8 => Some("b"),
        0x06 => Some("b,"),
        0x47 => Some("b, a"),
        0x40 => Some("b, b"),
        0x42 => Some("b, d"),
        0x44 => Some("b, h"),
        0x46 => Some("b, (hl)"),
        0x03 | 0x0b | 0xc1 | 0xc5 => Some("bc"),
        0x01 => Some("bc,"),
        0x02 => Some("(bc), a"),

        0x0c | 0x0d | 0xb1 | 0xd8 => Some("c"),
        0x0e | 0x38 | 0xda => Some("c,"),
        0x4f => Some("c, a"),
        0x48 => Some("c, b"),
        0x4a => Some("c, d"),
        0x4b => Some("c, e"),
        0x4c => Some("c, h"),
        0x4d => Some("c, l"),
        0x4e => Some("c, (hl)"),

        0x14 | 0x15 | 0x92 | 0xa2 | 0xaa => Some("d"),
        0x16 => Some("d,"),
        0x57 => Some("d, a"),
        0x50 => Some("d, b"),
        0x53 | 0x5a => Some("d, e"),
        0x55 => Some("d, l"),
        0x56 => Some("d, (hl)"),
        0x13 | 0x1b | 0xd1 | 0xd5 => Some("de"),
        0x11 => Some("de,"),
        0xeb => Some("de, hl"),
        0x12 => Some("(de), a"),

        0x1c | 0x1d | 0x93 | 0xa3 | 0xab | 0xb3 | 0xbb => Some("e"),
        0x1e => Some("e,"),
        0x5f => Some("e, a"),
        0x59 => Some("e, c"),
        0x5b => Some("e, e"),
        0x5d => Some("e, l"),
        0x5e => Some("e, (hl)"),

        0x24 | 0x25 | 0xa4 | 0xbc => Some("h"),
        0x26 => Some("h,"),
        0x67 => Some("h, a"),
        0x60 => Some("h, b"),
        0x61 => Some("h, c"),
        0x62 => Some("h, d"),
        0x63 => Some("h, e"),
        0x64 => Some("h, h"),
        0x65 => Some("h, l"),
        0x66 => Some("h, (hl)"),
        0x23 | 0x2b | 0xe1 | 0xe5 => Some("hl"),
        0x21 | 0x2a => Some("hl,"),
        0x09 => Some("hl, bc"),
        0x19 => Some("hl, de"),
        0x29 => Some("hl, hl"),
        0x34 | 0x35 | 0x96 | 0xbe | 0xe9 => Some("(hl)"),
        0x36 => Some("(hl),"),
        0x77 => Some("(hl), a"),
        0x70 => Some("(hl), b"),
        0x71 => Some("(hl), c"),
        0x72 => Some("(hl), d"),
        0x73 => Some("(hl), e"),
        0x75 => Some("(hl), l"),

        0x2c | 0x2d | 0xa5 | 0xad | 0xb5 | 0xbd => Some("l"),
        0x2e => Some("l,"),
        0x6f => Some("l, a"),
        0x68 => Some("l, b"),
        0x69 => Some("l, c"),
        0x6c => Some("l, h"),

        0xe3 => Some("(sp), hl"),

        0xfa => Some("m,"),
        0xd0 => Some("nc"),
        0x30 | 0xd4 => Some("nc,"),
        0xc0 => Some("nz"),
        0x20 | 0xc2 | 0xc4 => Some("nz,"),
        0xf0 => Some("p"),
        0xf2 => Some("p,"),
        0x31 => Some("sp,"),
        0xc8 => Some("z"),
        0x28 | 0xca | 0xcc => Some("z,"),

        0xcb => {
            match param1 {
                0x07 | 0x3f => Some("a"),
                0x12 | 0x1a => Some("d"),
                _ => None,
            }
        }

        0xdd => {
            match param1 {
                0x7e => Some("a, (ix +"),
                0x4e => Some("c, (ix +"),
                0xe1 | 0xe5 => Some("ix"),
                0x19 => Some("ix, de"),

                0xcb => {
                    match param2 {
                        0x06 => Some("(ix +"),
                        _ => None,
                    }
                }

                _ => None,
            }
        }

        0xed => {
            match param1 {
                0x5f => Some("a, r"),
                0x78 => Some("a, (c)"),
                0x4b => Some("bc,"),
                0x5b => Some("de,"),
                0x42 => Some("hl, bc"),
                0x52 => Some("hl, de"),
                0x62 => Some("hl, hl"),
                0x68 => Some("l, (c)"),
                0x79 => Some("(c), a"),
                0x49 => Some("(c), c"),
                0x51 => Some("(c), d"),
                0x59 => Some("(c), e"),
                0x61 => Some("(c), h"),
                0x69 => Some("(c), l"),
                0x7b => Some("sp,"),
                _ => None,
            }
        }

        0xfd => {
            match param1 {
                0xe1 | 0xe5 => Some("iy"),
                _ => None,
            }
        }
        _ => None,
    };
    if initial_reg != None {
        ins_tokens.push(Token {text: String::from(initial_reg.unwrap()), color: REG_COLOR})
    }

    let mut target: Option<u16> = None;

    // Values
    match op {
        0x01 | 0x11 | 0x21 | 0x31 | 0xc2 | 0xc3 | 0xc4 | 0xca | 0xcc | 0xcd | 0xd4 | 0xda | 0xf2 | 0xfa => {
            // a16
            let op1 = cpu.io.read_byte(pc);
            pc += 1;
            let op2 = cpu.io.read_byte(pc);
            pc += 1;
            bytes.push(op1);
            bytes.push(op2);
            let val = ((op2 as u16) << 8) | op1 as u16;
            ins_tokens.push(Token {text: format!("${:04x}", val), color: WHITE_COLOR});
        }
        0x06 | 0x0e | 0x16 | 0x1e | 0x26 | 0x2e | 0x36 | 0x3e | 0xc6 | 0xd6 | 0xe6 | 0xee | 0xf6 | 0xfe => {
            // d8
            let op1 = cpu.io.read_byte(pc);
            pc += 1;
            bytes.push(op1);
            ins_tokens.push(Token {text: format!("${:02x}", op1), color: WHITE_COLOR});
        }
        0xdb => {
            // (d8)
            let op1 = cpu.io.read_byte(pc);
            pc += 1;
            bytes.push(op1);
            ins_tokens.push(Token {text: format!("(${:02x})", op1), color: WHITE_COLOR});
        }
        0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38 => {
            // r8
            let op1 = cpu.io.read_byte(pc);
            pc += 1;
            bytes.push(op1);
            let op1 = op1 as u16;
            ins_tokens.push(Token {text: format!("${:02x}", op1), color: WHITE_COLOR});
            if op1 >= 0x80 {
                target = Some(pc.wrapping_sub(0x100-op1));
            } else {
                target = Some(pc.wrapping_add(op1));
            }
        }
        0x22 | 0x32 => {
            // (a16),
            let op1 = cpu.io.read_byte(pc);
            pc += 1;
            let op2 = cpu.io.read_byte(pc);
            pc += 1;
            bytes.push(op1);
            bytes.push(op2);
            let val = ((op2 as u16) << 8) | op1 as u16;
            ins_tokens.push(Token {text: format!("(${:04x}),", val), color: WHITE_COLOR});
        }
        0x2a | 0x3a => {
            // (a16)
            let op1 = cpu.io.read_byte(pc);
            pc += 1;
            let op2 = cpu.io.read_byte(pc);
            pc += 1;
            bytes.push(op1);
            bytes.push(op2);
            let val = ((op2 as u16) << 8) | op1 as u16;
            ins_tokens.push(Token {text: format!("(${:04x})", val), color: WHITE_COLOR});
        }
        0xc7 => {
            // rst $00
            ins_tokens.push(Token {text: String::from("$00"), color: WHITE_COLOR});
        }
        0xcf => {
            // rst $08
            ins_tokens.push(Token {text: String::from("$08"), color: WHITE_COLOR});
        }
        0xff => {
            // rst $38
            ins_tokens.push(Token {text: String::from("$38"), color: WHITE_COLOR});
        }
        0xcb => {
            match param1 {
                0xc7 => {
                    // 0,
                    ins_tokens.push(Token {text: String::from("0,"), color: WHITE_COLOR});
                }
                0xcf => {
                    // 1,
                    ins_tokens.push(Token {text: String::from("1,"), color: WHITE_COLOR});
                }
                0xd7 => {
                    // 2,
                    ins_tokens.push(Token {text: String::from("2,"), color: WHITE_COLOR});
                }
                0xdf => {
                    // 3,
                    ins_tokens.push(Token {text: String::from("3,"), color: WHITE_COLOR});
                }
                0xa0 | 0xa4 | 0xe0 | 0xe4 | 0xe7 => {
                    // 4,
                    ins_tokens.push(Token {text: String::from("4,"), color: WHITE_COLOR});
                }
                0x6c => {
                    // 5,
                    ins_tokens.push(Token {text: String::from("5,"), color: WHITE_COLOR});
                }
                0x70 | 0x74 | 0x77 | 0xf7 => {
                    // 6,
                    ins_tokens.push(Token {text: String::from("6,"), color: WHITE_COLOR});
                }
                0xff => {
                    // 7,
                    ins_tokens.push(Token {text: String::from("7,"), color: WHITE_COLOR});
                }
                _ => (),
            }
        }
        0xdd => {
            match param1 {
                0x4e | 0x7e => {
                    // d8)
                    let op1 = cpu.io.read_byte(pc);
                    pc += 1;
                    bytes.push(op1);
                    ins_tokens.push(Token {text: format!("${:02x})", op1), color: WHITE_COLOR});
                }
                0xcb => {
                    match param2 {
                        0x06 => {
                            // d8)
                            let op1 = cpu.io.read_byte(pc);
                            pc += 1;
                            bytes.push(op1);
                            ins_tokens.push(Token {text: format!("${:02x})", op1), color: WHITE_COLOR});
                        }
                        _ => ()
                    }
                }
                _ => (),
            }
        }
        0xed => unsafe {
            match param1 {
                0x49 | 0x51 | 0x59 | 0x61 | 0x69 | 0x78 | 0x79 => target = Some(cpu.c2rust_unnamed_0.bc),
                0x4b | 0x5b | 0x7b => {
                    // (a16)
                    let op1 = cpu.io.read_byte(pc);
                    pc += 1;
                    let op2 = cpu.io.read_byte(pc);
                    pc += 1;
                    bytes.push(op1);
                    bytes.push(op2);
                    let val = ((op2 as u16) << 8) | op1 as u16;
                    ins_tokens.push(Token {text: format!("(${:04x})", val), color: WHITE_COLOR});
                }
                0x53 | 0x73 => {
                    // (a16),
                    let op1 = cpu.io.read_byte(pc);
                    pc += 1;
                    let op2 = cpu.io.read_byte(pc);
                    pc += 1;
                    bytes.push(op1);
                    bytes.push(op2);
                    let val = ((op2 as u16) << 8) | op1 as u16;
                    ins_tokens.push(Token {text: format!("(${:04x}),", val), color: WHITE_COLOR});
                }
                _ => (),
            };
        }
        _ => ()
    }

    // Registers after a value
    match op {
        0x22 => ins_tokens.push(Token {text: String::from("hl"), color: REG_COLOR}),
        0x32 => ins_tokens.push(Token {text: String::from("a"), color: REG_COLOR}),
        0xcb => {
            match param1 {
                0x77 | 0xc7 | 0xcf | 0xd7 | 0xdf | 0xe7 | 0xf7 | 0xff => ins_tokens.push(Token {text: String::from("a"), color: REG_COLOR}),
                0x70 | 0xa0 | 0xe0 => ins_tokens.push(Token {text: String::from("b"), color: REG_COLOR}),
                0x6c | 0x74 | 0xa4 | 0xe4 => ins_tokens.push(Token {text: String::from("h"), color: REG_COLOR}),
                _ => (),
            }
        }
        0xed => {
            match param1 {
                0x53 => ins_tokens.push(Token {text: String::from("de"), color: REG_COLOR}),
                0x73 => ins_tokens.push(Token {text: String::from("sp"), color: REG_COLOR}),
                _ => (),
            }
        }
        _ => ()
    }

    let start_idx = bytes.len();
    for byte in bytes {
        ret.push(Token {
            color: FADE_COLOR,
            text: format!("{:02x}", byte),
        });
    }
    
    for _ in start_idx..4 {
        ret.push(Token {
            color: WHITE_COLOR,
            text: String::from("  "),
        })
    }

    ret.push(Token {
        color: MNEM_COLOR,
        text: String::from(mnemonic),
    });

    for token in ins_tokens {
        ret.push(token);
    }

    match target {
        None => (),
        Some(addr) => {
            ret.push(Token {
                color: FADE_COLOR,
                text: String::from("=>"),
            });
            ret.push(Token {
                color: WHITE_COLOR,
                text: format!("${:04x}", addr),
            });
        }
    }
    
    (ret, pc)
}

fn flags(f: u8, ui: &mut Ui) {
    ui.horizontal(|ui| {
        // szxhxpvnc
        ui.label(RichText::new("F:").color(MNEM_COLOR).text_style(MONOSPACE.clone()));
        ui.label(RichText::new("S").color(if (f & 0x80) != 0 {WHITE_COLOR} else {FADE_COLOR}).text_style(MONOSPACE.clone()));
        ui.label(RichText::new("Z").color(if (f & 0x40) != 0 {WHITE_COLOR} else {FADE_COLOR}).text_style(MONOSPACE.clone()));
        ui.label(RichText::new("X").color(if (f & 0x20) != 0 {WHITE_COLOR} else {FADE_COLOR}).text_style(MONOSPACE.clone()));
        ui.label(RichText::new("H").color(if (f & 0x10) != 0 {WHITE_COLOR} else {FADE_COLOR}).text_style(MONOSPACE.clone()));
        ui.label(RichText::new("X").color(if (f & 0x08) != 0 {WHITE_COLOR} else {FADE_COLOR}).text_style(MONOSPACE.clone()));
        ui.label(RichText::new("PV").color(if (f & 0x04) != 0 {WHITE_COLOR} else {FADE_COLOR}).text_style(MONOSPACE.clone()));
        ui.label(RichText::new("N").color(if (f & 0x02) != 0 {WHITE_COLOR} else {FADE_COLOR}).text_style(MONOSPACE.clone()));
        ui.label(RichText::new("C").color(if (f & 0x01) != 0 {WHITE_COLOR} else {FADE_COLOR}).text_style(MONOSPACE.clone()));
    });
}

impl Disassembler {
    pub fn new() -> Self {
        Self {
            lines: vec![],
        }
    }

    pub fn prepare(&mut self, cpu: &mut Z80<crate::IO>) {
        self.lines = vec![];

        let mut pc = cpu.pc;
        for _ in 0..=30 {
            let tokens;
            (tokens, pc) = get_tokens(
                cpu, pc,
                cpu.pc != 0xda56,
            );
            self.lines.push(tokens);
        }
    }

    pub fn display(&self, ui: &mut Ui, cpu: &mut Z80<crate::IO>) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("PC:").color(MNEM_COLOR).text_style(MONOSPACE.clone()));
            ui.label(RichText::new(format!("{:04x}", cpu.pc)).color(WHITE_COLOR).text_style(MONOSPACE.clone()));
            ui.label(RichText::new("SP:").color(MNEM_COLOR).text_style(MONOSPACE.clone()));
            ui.label(RichText::new(format!("{:04x}", cpu.sp)).color(WHITE_COLOR).text_style(MONOSPACE.clone()));
            ui.label(RichText::new("IX:").color(MNEM_COLOR).text_style(MONOSPACE.clone()));
            ui.label(RichText::new(format!("{:04x}", cpu.ix)).color(WHITE_COLOR).text_style(MONOSPACE.clone()));
            ui.label(RichText::new("IY:").color(MNEM_COLOR).text_style(MONOSPACE.clone()));
            ui.label(RichText::new(format!("{:04x}", cpu.iy)).color(WHITE_COLOR).text_style(MONOSPACE.clone()));
        });
        ui.separator();
        ui.label(RichText::new("Current Registers:").color(MNEM_COLOR).text_style(MONOSPACE.clone()));
        ui.horizontal(|ui| unsafe {
            ui.label(RichText::new("A:").color(MNEM_COLOR).text_style(MONOSPACE.clone()));
            ui.label(RichText::new(format!("{:02x}", cpu.c2rust_unnamed.c2rust_unnamed.a)).color(WHITE_COLOR).text_style(MONOSPACE.clone()));
            ui.label(RichText::new("BC:").color(MNEM_COLOR).text_style(MONOSPACE.clone()));
            ui.label(RichText::new(format!("{:04x}", cpu.c2rust_unnamed_0.bc)).color(WHITE_COLOR).text_style(MONOSPACE.clone()));
            ui.label(RichText::new("DE:").color(MNEM_COLOR).text_style(MONOSPACE.clone()));
            ui.label(RichText::new(format!("{:04x}", cpu.c2rust_unnamed_1.de)).color(WHITE_COLOR).text_style(MONOSPACE.clone()));
            ui.label(RichText::new("HL:").color(MNEM_COLOR).text_style(MONOSPACE.clone()));
            ui.label(RichText::new(format!("{:04x}", cpu.c2rust_unnamed_2.hl)).color(WHITE_COLOR).text_style(MONOSPACE.clone()));
        });
        unsafe {flags(cpu.c2rust_unnamed.c2rust_unnamed.f, ui);}
        ui.separator();
        ui.label(RichText::new("Backup Registers:").color(MNEM_COLOR).text_style(MONOSPACE.clone()));
        ui.horizontal(|ui| unsafe {
            ui.label(RichText::new("A:").color(MNEM_COLOR).text_style(MONOSPACE.clone()));
            ui.label(RichText::new(format!("{:02x}", cpu.c2rust_unnamed_3.c2rust_unnamed.a_)).color(WHITE_COLOR).text_style(MONOSPACE.clone()));
            ui.label(RichText::new("BC:").color(MNEM_COLOR).text_style(MONOSPACE.clone()));
            ui.label(RichText::new(format!("{:04x}", cpu.c2rust_unnamed_4.b_c_)).color(WHITE_COLOR).text_style(MONOSPACE.clone()));
            ui.label(RichText::new("DE:").color(MNEM_COLOR).text_style(MONOSPACE.clone()));
            ui.label(RichText::new(format!("{:04x}", cpu.c2rust_unnamed_5.d_e_)).color(WHITE_COLOR).text_style(MONOSPACE.clone()));
            ui.label(RichText::new("HL:").color(MNEM_COLOR).text_style(MONOSPACE.clone()));
            ui.label(RichText::new(format!("{:04x}", cpu.c2rust_unnamed_6.h_l_)).color(WHITE_COLOR).text_style(MONOSPACE.clone()));
        });
        unsafe {flags(cpu.c2rust_unnamed_3.c2rust_unnamed.f_, ui);}
        ui.separator();
        for i in 0..self.lines.len() {
            let line = &self.lines[i];
            ui.horizontal(|ui| {
                for token in line {
                    ui.label(RichText::new(token.text.clone()).color(token.color).text_style(MONOSPACE.clone()));
                }
            });
        }
    }
}