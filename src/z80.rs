pub trait Z80IO {
    fn peek_byte(&mut self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, val: u8);
    fn peek_io(&mut self, addr: u16) -> u8;
    fn write_io(&mut self, addr: u16, val: u8);
}

#[derive(PartialEq, Savefile, Clone)]
pub enum FDEPhase {
    Init,
    ReadMem,
    Fetch,
    Execute,
}

#[derive(Copy, Clone, PartialEq, Savefile)]
enum Prefix {
    DD,
    FD,
    NONE,
}

const OP_CALL_COND: u16 = 0x700;
const OP_CHECK_RET: u16 = 0x701;
const OP_IM: u16 = 0x702;

#[derive(Savefile, Clone)]
enum Cycle {
    AddrIsALow,
    AddrIsBC,
    AddrIsDE,
    AddrIsDecBC,
    AddrIsIM2High,
    AddrIsIM2Low,
    AddrIsPrefixedHL,
    AddrIsSP0,
    AddrIsSP1,
    CallCond,
    CheckIrqData,
    CheckRep,
    CheckRet,
    DecLow,
    FetchHigh,
    FetchIndHigh,
    FetchIndLow,
    FetchLow,
    IncLow,
    IndInitHi,
    IndInitLo,
    Init,
    JrCond,
    Nop,
    PeekByte,
    PeekHigh,
    PeekLow,
    PopStackHigh,
    PopStackLow,
    PushStack,
    ReadIO,
    ReadMem,
    ResExt,
    RLData,
    RLCData,
    RLDData,
    RRData,
    RRCData,
    RRDData,
    SetExt,
    SetWordHigh,
    SetWordLow,
    SLAData,
    SLLData,
    SRAData,
    SRLData,
    Unwrite,
    WriteA,
    WriteB,
    WriteC,
    WriteD,
    WriteE,
    WriteF,
    WriteH,
    WriteHigh,
    WriteIOLow,
    WriteIOU8(u8),
    WriteL,
    WriteLow,
    WritePCHigh,
    WritePCLow,
    WritePrefixedH,
    WritePrefixedL,
    WriteSPHigh,
    WriteSPLow,
}

const FLAG_S: u8 = 0x80; // sign
const FLAG_Z: u8 = 0x40; // zero
const FLAG_H: u8 = 0x10; // half-carry
const FLAG_PV: u8 = 0x04; // parity/overflow
const FLAG_N: u8 = 0x02; // add/subtract
const FLAG_C: u8 = 0x01; // carry


#[derive(Savefile, Clone)]
pub struct Z80 {
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub i: u8,
    pub r: u8,
    pub ei: u8,
    pub wz: u16,
    pub ix: u16,
    pub iy: u16,
    pub af_: u16,
    pub bc_: u16,
    pub de_: u16,
    pub hl_: u16,
    pub im: u8,
    pub p: u8,
    pub q: u8,
    pub iff1: u8,
    pub iff2: u8,

    pub addr_bus: Option<u16>,
    pub data_bus: Option<u8>,
    pub phase: FDEPhase,
    microcodes: Vec<Cycle>,
    read_pin: bool,
    write_pin: bool,
    memory_pin: bool,
    io_pin: bool,
    curr_op: u16,
    halt: bool,

    low_byte: u8,
    high_byte: u8,
    ind_addr: u16,
    set_q: bool,
    group_8: u8,
    group_1: u8,
    prefix: Prefix,

    pub irq_req: bool,
    curr_irq_data: u8,
    init_pc: u16,
    is_ext: bool,
    is_im0: bool,
}

fn bit_set(val: u8, bit: u8) -> bool {
    val & flag(bit) != 0
}

fn flag(bit: u8) -> u8 {
    match bit {
        0 => 0x01,
        1 => 0x02,
        2 => 0x04,
        3 => 0x08,
        4 => 0x10,
        5 => 0x20,
        6 => 0x40,
        7 => 0x80,
        _ => panic!("Invalid flag bit"),
    }
}

fn mask(bit: u8) -> u8 {
    0xff - flag(bit)
}

fn parity(val: u8) -> bool {
    let mut total = 0;
    for i in 0..=7 {
        if bit_set(val, i) {
            total += 1;
        }
    }
    (total % 2) == 0
}

fn word(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}

fn bit(cpu: &mut Z80) {
    let bit = cpu.group_8;
    cpu.f &= 0xff - (FLAG_N | FLAG_PV | 0x28 | FLAG_Z | FLAG_S);

    let val;
    if cpu.group_1 == 6 || cpu.prefix != Prefix::NONE {
        val = cpu.low_byte;
        cpu.f |= FLAG_H | (((cpu.wz >> 8) as u8) & 0x28);
    } else {
        val = cpu.prefixed_group_reg_r(cpu.group_1);
        cpu.f |= FLAG_H | (val & 0x28);
    }
    if !bit_set(val, bit) {
        cpu.f |= FLAG_PV | FLAG_Z;
    }
    if bit == 7 && bit_set(val, 7) {
        cpu.f |= FLAG_S;
    }
}

fn add_prefixed_hl_rp(cpu: &mut Z80, rp: u16) {
    cpu.f &= 0xff - (FLAG_C | FLAG_N | FLAG_H | 0x28);
    let base = cpu.prefixed_hl();
    let new = base.wrapping_add(rp);
    if new < base {
        cpu.f |= FLAG_C
    };
    if (base & 0xfff) + (rp & 0xfff) >= 0x1000 {
        cpu.f |= FLAG_H
    }
    match cpu.prefix {
        Prefix::DD => cpu.ix = new,
        Prefix::FD => cpu.iy = new,
        Prefix::NONE => {
            cpu.h = (new >> 8) as u8;
            cpu.l = new as u8;
        }
    }
    cpu.f |= ((new >> 8) as u8) & 0x28;
}

fn ccf(cpu: &mut Z80) {
    cpu.f &= 0xff - (FLAG_N | FLAG_H);
    if cpu.flag(FLAG_C) {
        cpu.f |= FLAG_H;
    }
    cpu.f ^= FLAG_C;
    if cpu.q != 0 {
        cpu.f &= 0xff - 0x28;
    }
    cpu.f |= cpu.a & 0x28;
}

fn cpl(cpu: &mut Z80) {
    cpu.f &= 0xff - 0x28;
    cpu.a ^= 0xff;
    cpu.f |= (cpu.a & 0x28) | FLAG_N | FLAG_H;
}

fn daa(cpu: &mut Z80) {
    let a = cpu.a;
    let n = cpu.flag(FLAG_N);
    let hc = cpu.flag(FLAG_H);
    let c = cpu.flag(FLAG_C);
    cpu.f &= 0xff - (FLAG_C | FLAG_H | 0x28 | FLAG_Z | FLAG_S);

    if c || cpu.a > 0x99 {
        if n {
            cpu.a = cpu.a.wrapping_sub(0x60);
        } else {
            cpu.a = cpu.a.wrapping_add(0x60);
        }
        cpu.f |= FLAG_C;
    }
    if hc || (cpu.a & 0xf) > 9 {
        if n {
            cpu.a = cpu.a.wrapping_sub(6);
        } else {
            cpu.a = cpu.a.wrapping_add(6);
        }
    }
    if ((a ^ cpu.a) & 0x10) != 0 {
        cpu.f |= FLAG_H;
    }

    // Do flags
    cpu.set_parity_a();
    cpu.f |= cpu.a & 0x28;
    if cpu.a == 0x00 {
        cpu.f |= FLAG_Z
    }
    if cpu.a >= 0x80 {
        cpu.f |= FLAG_S
    }
}

fn dec_word(high: &mut u8, low: &mut u8) {
    *low = (*low).wrapping_sub(1);
    if *low == 0xff {
        *high = (*high).wrapping_sub(1);
    }
}

fn di(cpu: &mut Z80) {
    cpu.iff1 = 0;
    cpu.iff2 = 0;
}

fn ei(cpu: &mut Z80) {
    cpu.ei = 2;
    cpu.iff1 = 1;
    cpu.iff2 = 1;
}

fn ex_af_af_(cpu: &mut Z80) {
    let new_shadow = word(cpu.a, cpu.f);
    cpu.a = (cpu.af_ >> 8) as u8;
    cpu.f = cpu.af_ as u8;
    cpu.af_ = new_shadow;
}

fn ex_de_hl(cpu: &mut Z80) {
    std::mem::swap::<u8>(&mut cpu.d, &mut cpu.h);
    std::mem::swap::<u8>(&mut cpu.e, &mut cpu.l);
}

fn exx(cpu: &mut Z80) {
    let new_shadow = cpu.bc();
    cpu.b = (cpu.bc_ >> 8) as u8;
    cpu.c = cpu.bc_ as u8;
    cpu.bc_ = new_shadow;

    let new_shadow = cpu.de();
    cpu.d = (cpu.de_ >> 8) as u8;
    cpu.e = cpu.de_ as u8;
    cpu.de_ = new_shadow;

    let new_shadow = cpu.hl();
    cpu.h = (cpu.hl_ >> 8) as u8;
    cpu.l = cpu.hl_ as u8;
    cpu.hl_ = new_shadow;
}

fn get_cb_op(cpu: &mut Z80) {
    cpu.phase = FDEPhase::Execute;
    if cpu.prefix == Prefix::NONE {
        cpu.addr_bus = Some(word(cpu.i, cpu.r));
    }
    let op = cpu.data_bus.unwrap();

    cpu.group_8 = (op / 8) % 8;
    cpu.group_1 = op % 8;

    cpu.microcodes = match op {
        // eg rlc (hl)
        0x00..=0x07 if (op == 0x06 || cpu.prefix != Prefix::NONE) => vec![
            Cycle::AddrIsPrefixedHL,
            Cycle::ReadMem,
            Cycle::PeekByte,
            Cycle::RLCData,
            Cycle::Nop,
            Cycle::WriteLow,
            Cycle::Unwrite,
        ],
        // eg rrc (hl)
        0x08..=0x0f if (op == 0x0e || cpu.prefix != Prefix::NONE) => vec![
            Cycle::AddrIsPrefixedHL,
            Cycle::ReadMem,
            Cycle::PeekByte,
            Cycle::RRCData,
            Cycle::Nop,
            Cycle::WriteLow,
            Cycle::Unwrite,
        ],
        // eg rl (hl)
        0x10..=0x17 if (op == 0x16 || cpu.prefix != Prefix::NONE) => vec![
            Cycle::AddrIsPrefixedHL,
            Cycle::ReadMem,
            Cycle::PeekByte,
            Cycle::RLData,
            Cycle::Nop,
            Cycle::WriteLow,
            Cycle::Unwrite,
        ],
        // eg rr (hl)
        0x18..=0x1f if (op == 0x1e || cpu.prefix != Prefix::NONE) => vec![
            Cycle::AddrIsPrefixedHL,
            Cycle::ReadMem,
            Cycle::PeekByte,
            Cycle::RRData,
            Cycle::Nop,
            Cycle::WriteLow,
            Cycle::Unwrite,
        ],
        // eg sla (hl)
        0x20..=0x27 if (op == 0x26 || cpu.prefix != Prefix::NONE) => vec![
            Cycle::AddrIsPrefixedHL,
            Cycle::ReadMem,
            Cycle::PeekByte,
            Cycle::SLAData,
            Cycle::Nop,
            Cycle::WriteLow,
            Cycle::Unwrite,
        ],
        // eg sra (hl)
        0x28..=0x2f if (op == 0x2e || cpu.prefix != Prefix::NONE) => vec![
            Cycle::AddrIsPrefixedHL,
            Cycle::ReadMem,
            Cycle::PeekByte,
            Cycle::SRAData,
            Cycle::Nop,
            Cycle::WriteLow,
            Cycle::Unwrite,
        ],
        // eg sll (hl)
        0x30..=0x37 if (op == 0x36 || cpu.prefix != Prefix::NONE) => vec![
            Cycle::AddrIsPrefixedHL,
            Cycle::ReadMem,
            Cycle::PeekByte,
            Cycle::SLLData,
            Cycle::Nop,
            Cycle::WriteLow,
            Cycle::Unwrite,
        ],
        // eg srl (hl)
        0x38..=0x3f if (op == 0x3e || cpu.prefix != Prefix::NONE) => vec![
            Cycle::AddrIsPrefixedHL,
            Cycle::ReadMem,
            Cycle::PeekByte,
            Cycle::SRLData,
            Cycle::Nop,
            Cycle::WriteLow,
            Cycle::Unwrite,
        ],
        // eg bit 0, (ix+d),
        0x40..=0x7f if cpu.prefix != Prefix::NONE => vec![
            Cycle::AddrIsPrefixedHL,
            Cycle::ReadMem,
            Cycle::PeekLow,
            Cycle::Nop,
        ],
        // eg bit 0, (hl)
        0x46 | 0x4e | 0x56 | 0x5e | 0x66 | 0x6e | 0x76 | 0x7e if cpu.prefix == Prefix::NONE => {
            vec![
                Cycle::AddrIsPrefixedHL,
                Cycle::ReadMem,
                Cycle::PeekLow,
                Cycle::Nop,
            ]
        }
        // eg res 0, (ix+d),
        0x80..=0xbf if cpu.prefix != Prefix::NONE => vec![
            Cycle::AddrIsPrefixedHL,
            Cycle::ReadMem,
            Cycle::PeekLow,
            Cycle::ResExt,
            Cycle::Nop,
            Cycle::WriteLow,
            Cycle::Unwrite,
        ],
        // eg res 0, (hl)
        0x86 | 0x8e | 0x96 | 0x9e | 0xa6 | 0xae | 0xb6 | 0xbe if cpu.prefix == Prefix::NONE => {
            vec![
                Cycle::AddrIsPrefixedHL,
                Cycle::ReadMem,
                Cycle::PeekLow,
                Cycle::ResExt,
                Cycle::Nop,
                Cycle::WriteLow,
                Cycle::Unwrite,
            ]
        }
        // eg set 0, (ix+d)
        0xc0..=0xff if cpu.prefix != Prefix::NONE => vec![
            Cycle::AddrIsPrefixedHL,
            Cycle::ReadMem,
            Cycle::PeekLow,
            Cycle::SetExt,
            Cycle::Nop,
            Cycle::WriteLow,
            Cycle::Unwrite,
        ],
        // eg set 0, (hl)
        0xc6 | 0xce | 0xd6 | 0xde | 0xe6 | 0xee | 0xf6 | 0xfe if cpu.prefix == Prefix::NONE => {
            vec![
                Cycle::AddrIsPrefixedHL,
                Cycle::ReadMem,
                Cycle::PeekLow,
                Cycle::SetExt,
                Cycle::Nop,
                Cycle::WriteLow,
                Cycle::Unwrite,
            ]
        }
        _ => vec![],
    };
    cpu.microcodes.insert(0, Cycle::Nop);
    if cpu.prefix != Prefix::NONE {
        cpu.microcodes.insert(0, Cycle::Nop);
    }
    cpu.curr_op = ((cpu.curr_op & 0xff00)+0x100)|(op as u16);
    cpu.set_q = true;
    cpu.is_ext = true;
}

fn get_ed_op(cpu: &mut Z80) {
    cpu.phase = FDEPhase::Execute;
    cpu.addr_bus = Some(word(cpu.i, cpu.r));
    let op = cpu.data_bus.unwrap();

    cpu.group_8 = (op / 8) % 8;
    cpu.group_1 = op % 8;

    cpu.microcodes = match op {
        // eg in b, (c)
        0x40 | 0x48 | 0x50 | 0x58 | 0x60 | 0x68 | 0x70 | 0x78 => {
            vec![Cycle::AddrIsBC, Cycle::Nop, Cycle::ReadIO, Cycle::PeekByte]
        }
        // eg out (c), b
        0x41 | 0x49 | 0x51 | 0x59 | 0x61 | 0x69 | 0x79 => vec![
            Cycle::AddrIsBC,
            Cycle::Nop,
            Cycle::WriteIOU8(cpu.prefixed_group_reg_r(cpu.group_8)),
            Cycle::Unwrite,
        ],
        // eg sbc hl, bc
        0x42 | 0x4a | 0x52 | 0x5a | 0x62 | 0x6a | 0x72 | 0x7a => vec![
            Cycle::Nop,
            Cycle::Nop,
            Cycle::Nop,
            Cycle::Nop,
            Cycle::Nop,
            Cycle::Nop,
            Cycle::Nop,
        ],
        // ld (nn), bc
        0x43 => vec![
            Cycle::Init,
            Cycle::ReadMem,
            Cycle::FetchLow,
            Cycle::Init,
            Cycle::ReadMem,
            Cycle::FetchHigh,
            Cycle::SetWordLow,
            Cycle::WriteC,
            Cycle::Unwrite,
            Cycle::SetWordHigh,
            Cycle::WriteB,
            Cycle::Unwrite,
        ],
        // eg retn
        0x45 | 0x4d | 0x55 | 0x5d | 0x65 | 0x6d | 0x75 | 0x7d => vec![
            Cycle::PopStackLow,
            Cycle::ReadMem,
            Cycle::PeekLow,
            Cycle::PopStackHigh,
            Cycle::ReadMem,
            Cycle::PeekHigh,
        ],
        // eg ld i, a
        0x47 | 0x4f | 0x57 | 0x5f => vec![Cycle::Nop],
        // eg ld bc, (nn)
        0x4b | 0x5b | 0x6b | 0x7b => vec![
            Cycle::Init,
            Cycle::ReadMem,
            Cycle::FetchLow,
            Cycle::Init,
            Cycle::ReadMem,
            Cycle::FetchHigh,
            Cycle::IndInitLo,
            Cycle::ReadMem,
            Cycle::FetchIndLow,
            Cycle::IndInitHi,
            Cycle::ReadMem,
            Cycle::FetchIndHigh,
        ],
        // ld (nn), de
        0x53 => vec![
            Cycle::Init,
            Cycle::ReadMem,
            Cycle::FetchLow,
            Cycle::Init,
            Cycle::ReadMem,
            Cycle::FetchHigh,
            Cycle::SetWordLow,
            Cycle::WriteE,
            Cycle::Unwrite,
            Cycle::SetWordHigh,
            Cycle::WriteD,
            Cycle::Unwrite,
        ],
        // ld (nn), hl
        0x63 => vec![
            Cycle::Init,
            Cycle::ReadMem,
            Cycle::FetchLow,
            Cycle::Init,
            Cycle::ReadMem,
            Cycle::FetchHigh,
            Cycle::SetWordLow,
            Cycle::WriteL,
            Cycle::Unwrite,
            Cycle::SetWordHigh,
            Cycle::WriteH,
            Cycle::Unwrite,
        ],
        // rrd
        0x67 => vec![
            Cycle::AddrIsPrefixedHL,
            Cycle::ReadMem,
            Cycle::PeekLow,
            Cycle::RRDData,
            Cycle::Nop,
            Cycle::WriteLow,
            Cycle::Unwrite,
            Cycle::Nop,
            Cycle::Nop,
            Cycle::Nop,
        ],
        // rld
        0x6f => vec![
            Cycle::AddrIsPrefixedHL,
            Cycle::ReadMem,
            Cycle::PeekLow,
            Cycle::RLDData,
            Cycle::Nop,
            Cycle::WriteLow,
            Cycle::Unwrite,
            Cycle::Nop,
            Cycle::Nop,
            Cycle::Nop,
        ],
        // ld (nn), sp
        0x73 => vec![
            Cycle::Init,
            Cycle::ReadMem,
            Cycle::FetchLow,
            Cycle::Init,
            Cycle::ReadMem,
            Cycle::FetchHigh,
            Cycle::SetWordLow,
            Cycle::WriteSPLow,
            Cycle::Unwrite,
            Cycle::SetWordHigh,
            Cycle::WriteSPHigh,
            Cycle::Unwrite,
        ],
        // eg out (c), 0
        0x71 => vec![
            Cycle::AddrIsBC,
            Cycle::Nop,
            Cycle::WriteIOU8(0),
            Cycle::Unwrite,
        ],
        // eg ldi
        0xa0 | 0xa8 | 0xb0 | 0xb8 => vec![
            Cycle::AddrIsPrefixedHL,
            Cycle::ReadMem,
            Cycle::PeekLow,
            Cycle::AddrIsDE,
            Cycle::WriteLow,
            Cycle::Unwrite,
            Cycle::Nop,
            Cycle::Nop,
        ],
        // eg cpi
        0xa1 | 0xa9 | 0xb1 | 0xb9 => vec![
            Cycle::AddrIsPrefixedHL,
            Cycle::ReadMem,
            Cycle::PeekLow,
            Cycle::Nop,
            Cycle::Nop,
            Cycle::Nop,
            Cycle::Nop,
            Cycle::Nop,
        ],
        // eg ini
        0xa2 | 0xaa | 0xb2 | 0xba => vec![
            Cycle::Nop,
            Cycle::AddrIsBC,
            Cycle::Nop,
            Cycle::ReadIO,
            Cycle::PeekLow,
            Cycle::AddrIsPrefixedHL,
            Cycle::WriteLow,
            Cycle::Unwrite,
        ],
        // eg outi
        0xa3 | 0xab | 0xb3 | 0xbb => vec![
            Cycle::Nop,
            Cycle::AddrIsPrefixedHL,
            Cycle::ReadMem,
            Cycle::PeekLow,
            Cycle::AddrIsDecBC,
            Cycle::Nop,
            Cycle::WriteIOLow,
            Cycle::Unwrite,
        ],
        _ => vec![],
    };
    cpu.microcodes.insert(0, Cycle::Nop);
    if op >= 0xb0 {
        cpu.microcodes.pop();
        cpu.microcodes.push(Cycle::CheckRep);
    }
    cpu.curr_op = 0x400|(op as u16);
    if op == 0x57 || op == 0x5f {
        cpu.p = 1;
    }
    cpu.set_q = match op {
        0x40
        | 0x48
        | 0x50
        | 0x58
        | 0x60
        | 0x68
        | 0x70
        | 0x78
        | 0x42
        | 0x4a
        | 0x52
        | 0x5a
        | 0x62
        | 0x6a
        | 0x72
        | 0x7a
        | 0x44
        | 0x4c
        | 0x54
        | 0x5c
        | 0x64
        | 0x6c
        | 0x74
        | 0x7c
        | 0x57
        | 0x5f
        | 0x67
        | 0x6f
        | 0xa0..=0xa3
        | 0xa8..=0xab
        | 0xb0..=0xb3
        | 0xb8..=0xbb => true,
        _ => false,
    };
    cpu.is_ext = true;
}

fn prefixed_hl_is_word(cpu: &mut Z80) {
    cpu.wz = word(cpu.high_byte, cpu.low_byte);
    match cpu.prefix {
        Prefix::DD => cpu.ix = cpu.wz,
        Prefix::FD => cpu.iy = cpu.wz,
        Prefix::NONE => {
            cpu.h = cpu.high_byte;
            cpu.l = cpu.low_byte;
        }
    }
}

fn in_reg_bc(cpu: &mut Z80) {
    cpu.f &= 0xff - (FLAG_N | FLAG_H | 0x28 | FLAG_Z | FLAG_S);
    let val = cpu.data_bus.unwrap();
    if cpu.group_8 != 6 {
        cpu.group_reg_w(cpu.group_8, val);
    }
    cpu.set_parity(val);
    cpu.f |= val & 0x28;
    if val == 0x00 {
        cpu.f |= FLAG_Z
    }
    if val >= 0x80 {
        cpu.f |= FLAG_S
    }
    cpu.wz = cpu.bc().wrapping_add(1);
}

fn inc_word(high: &mut u8, low: &mut u8) {
    *low = (*low).wrapping_add(1);
    if *low == 0 {
        *high = (*high).wrapping_add(1);
    }
}

fn jp_cond(cpu: &mut Z80) {
    cpu.wz = word(cpu.high_byte, cpu.low_byte);
    if match cpu.group_8 {
        0 => !cpu.flag(FLAG_Z),
        1 => cpu.flag(FLAG_Z),
        2 => !cpu.flag(FLAG_C),
        3 => cpu.flag(FLAG_C),
        4 => !cpu.flag(FLAG_PV),
        5 => cpu.flag(FLAG_PV),
        6 => !cpu.flag(FLAG_S),
        7 => cpu.flag(FLAG_S),
        _ => panic!("Invalid jp_cond"),
    } {
        cpu.pc = cpu.wz;
    }
}

fn rla(cpu: &mut Z80) {
    let high_bit = bit_set(cpu.a, 7);
    let carry = cpu.flag(FLAG_C);
    cpu.a <<= 1;
    cpu.f &= 0xff - (FLAG_C | FLAG_N | FLAG_H | 0x28);
    cpu.f |= cpu.a & 0x28;
    if carry {
        cpu.a |= 1;
    }
    if high_bit {
        cpu.f |= FLAG_C;
    }
}

fn rlca(cpu: &mut Z80) {
    let high_bit = bit_set(cpu.a, 7);
    cpu.a <<= 1;
    cpu.f &= 0xff - (FLAG_C | FLAG_N | FLAG_H | 0x28);
    cpu.f |= cpu.a & 0x28;
    if high_bit {
        cpu.a |= 1;
        cpu.f |= FLAG_C;
    }
}

fn rra(cpu: &mut Z80) {
    let low_bit = bit_set(cpu.a, 0);
    let carry = cpu.flag(FLAG_C);
    cpu.a >>= 1;
    cpu.f &= 0xff - (FLAG_C | FLAG_N | FLAG_H | 0x28);
    cpu.f |= cpu.a & 0x28;
    if carry {
        cpu.a |= 0x80;
    }
    if low_bit {
        cpu.f |= FLAG_C;
    }
}

fn rrca(cpu: &mut Z80) {
    let low_bit = bit_set(cpu.a, 0);
    cpu.a >>= 1;
    cpu.f &= 0xff - (FLAG_C | FLAG_N | FLAG_H | 0x28);
    cpu.f |= cpu.a & 0x28;
    if low_bit {
        cpu.a |= 0x80;
        cpu.f |= FLAG_C;
    }
}

fn rst(cpu: &mut Z80) {
    cpu.wz = cpu.group_8 as u16 * 8;
    cpu.pc = cpu.wz;
}

fn adc_hl_rp(cpu: &mut Z80) {
    let carry = cpu.f & FLAG_C;
    cpu.f &= 0xff - (FLAG_C | FLAG_N | FLAG_PV | 0x28 | FLAG_H | FLAG_Z | FLAG_S);
    let hl = cpu.hl();
    let hl32 = hl as u32;
    let val = match cpu.group_8 {
        1 => cpu.bc(),
        3 => cpu.de(),
        5 => hl,
        7 => cpu.sp,
        _ => panic!("Invalid adc_hl_rp"),
    };
    let val32 = val as u32;
    let c32 = carry as u32;
    if hl32 + val32 + c32 > 0xffff {
        cpu.f |= FLAG_C;
    }
    let high_bit = hl & 0x8000;
    if high_bit == (val & 0x8000) {
        if high_bit == 0 {
            if (hl32 & 0x7fff) + (val32 & 0x7fff) + c32 > 0x7fff {
                cpu.f |= FLAG_PV;
            }
        } else {
            if (hl32 & 0x7fff) + (val32 & 0x7fff) + c32 < 0x8000 {
                cpu.f |= FLAG_PV;
            }
        }
    }
    if (hl & 0xfff) + (val & 0xfff) + (carry as u16) > 0xfff {
        cpu.f |= FLAG_H;
    }
    cpu.wz = hl.wrapping_add(1);
    let res = hl.wrapping_add(val).wrapping_add(carry as u16);
    cpu.h = (res >> 8) as u8;
    cpu.l = res as u8;
    cpu.f |= cpu.h & 0x28;
    if res == 0x0000 {
        cpu.f |= FLAG_Z
    }
    if cpu.h >= 0x80 {
        cpu.f |= FLAG_S
    }
}

fn sbc_hl_rp(cpu: &mut Z80) {
    let carry: u8 = cpu.f & FLAG_C;
    cpu.f &= 0xff - (FLAG_C | FLAG_PV | 0x28 | FLAG_H | FLAG_Z | FLAG_S);
    let hl = cpu.hl();
    let hl32 = hl as u32;
    let val = match cpu.group_8 {
        0 => cpu.bc(),
        2 => cpu.de(),
        4 => hl,
        6 => cpu.sp,
        _ => panic!("Invalid sbc_hl_rp"),
    };
    let val32 = val as u32;
    let c32 = carry as u32;
    if hl32 < val32 + c32 {
        cpu.f |= FLAG_C;
    }
    let high_bit = hl & 0x8000;
    if high_bit != (val & 0x8000) {
        if high_bit == 0 {
            if hl32 + 0x8000 >= val32 + c32 {
                cpu.f |= FLAG_PV;
            }
        } else {
            if hl32 < val32 + 0x8000 + c32 {
                cpu.f |= FLAG_PV;
            }
        }
    }
    if (hl & 0xfff) < (val & 0xfff) + (carry as u16) {
        cpu.f |= FLAG_H;
    }
    cpu.wz = hl.wrapping_add(1);
    let res = hl.wrapping_sub(val).wrapping_sub(carry as u16);
    cpu.h = (res >> 8) as u8;
    cpu.l = res as u8;
    cpu.f |= FLAG_N | (cpu.h & 0x28);
    if res == 0x0000 {
        cpu.f |= FLAG_Z
    }
    if cpu.h >= 0x80 {
        cpu.f |= FLAG_S
    }
}

fn scf(cpu: &mut Z80) {
    cpu.f &= 0xff - (FLAG_N | FLAG_H);
    if cpu.q != 0 {
        cpu.f &= 0xff - 0x28;
    }
    cpu.f |= FLAG_C | (cpu.a & 0x28);
}

fn set_af(cpu: &mut Z80) {
    cpu.a = cpu.high_byte;
    cpu.f = cpu.low_byte;
}

fn set_bc(cpu: &mut Z80) {
    cpu.b = cpu.high_byte;
    cpu.c = cpu.low_byte;
}

fn set_de(cpu: &mut Z80) {
    cpu.d = cpu.high_byte;
    cpu.e = cpu.low_byte;
}

fn set_prefixed_hl(cpu: &mut Z80) {
    match cpu.prefix {
        Prefix::DD => cpu.ix = word(cpu.high_byte, cpu.low_byte),
        Prefix::FD => cpu.iy = word(cpu.high_byte, cpu.low_byte),
        Prefix::NONE => {
            cpu.h = cpu.high_byte;
            cpu.l = cpu.low_byte;
        }
    }
}

impl Z80 {
    pub fn new() -> Z80 {
        Z80 {
            pc: 0,
            sp: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            i: 0,
            r: 0,
            ei: 0,
            wz: 0,
            ix: 0,
            iy: 0,
            af_: 0,
            bc_: 0,
            de_: 0,
            hl_: 0,
            im: 0,
            p: 0,
            q: 0,
            iff1: 0,
            iff2: 0,

            addr_bus: None,
            data_bus: None,
            phase: FDEPhase::Init,
            microcodes: vec![],
            read_pin: false,
            write_pin: false,
            memory_pin: false,
            io_pin: false,
            curr_op: 0,
            halt: false,

            low_byte: 0,
            high_byte: 0,
            ind_addr: 0,
            set_q: false,
            group_8: 0,
            group_1: 0,
            prefix: Prefix::NONE,

            irq_req: false,
            curr_irq_data: 0,
            init_pc: 0,
            is_ext: false,
            is_im0: false,
        }
    }

    fn and_a_r(&mut self, val: u8) {
        self.f &= 0xff - (FLAG_C | FLAG_N | 0x28 | FLAG_Z | FLAG_S);
        self.a &= val;
        self.set_parity_a();
        self.f |= FLAG_H | (self.a & 0x28);
        if self.a == 0x00 {
            self.f |= FLAG_Z
        }
        if self.a >= 0x80 {
            self.f |= FLAG_S
        }
    }

    fn adc_a_r(&mut self, val: u8) {
        let carry = self.f & FLAG_C;
        self.f &= 0xff - (FLAG_C | FLAG_N | FLAG_PV | 0x28 | FLAG_H | FLAG_Z | FLAG_S);
        if (self.a as u16) + (val as u16) + (carry as u16) > 0xff {
            self.f |= FLAG_C;
        }
        let high_bit = self.a & 0x80;
        if high_bit == (val & 0x80) {
            if high_bit == 0 {
                if (self.a & 0x7f) + (val & 0x7f) + carry > 0x7f {
                    self.f |= FLAG_PV;
                }
            } else {
                if (self.a & 0x7f) + (val & 0x7f) + carry < 0x80 {
                    self.f |= FLAG_PV;
                }
            }
        }
        if (self.a & 0xf) + (val & 0xf) + carry > 0xf {
            self.f |= FLAG_H;
        }
        self.a = self.a.wrapping_add(val).wrapping_add(carry);
        self.f |= self.a & 0x28;
        if self.a == 0x00 {
            self.f |= FLAG_Z
        }
        if self.a >= 0x80 {
            self.f |= FLAG_S
        }
    }

    fn add_a_r(&mut self, val: u8) {
        self.f &= 0xff - (FLAG_C | FLAG_N | FLAG_PV | 0x28 | FLAG_H | FLAG_Z | FLAG_S);
        if (self.a as u16) + (val as u16) > 0xff {
            self.f |= FLAG_C;
        }
        let high_bit = self.a & 0x80;
        if high_bit == (val & 0x80) {
            if high_bit == 0 {
                if (self.a & 0x7f) + (val & 0x7f) > 0x7f {
                    self.f |= FLAG_PV;
                }
            } else {
                if (self.a & 0x7f) + (val & 0x7f) < 0x80 {
                    self.f |= FLAG_PV;
                }
            }
        }
        if (self.a & 0xf) + (val & 0xf) > 0xf {
            self.f |= FLAG_H;
        }
        self.a = self.a.wrapping_add(val);
        self.f |= self.a & 0x28;
        if self.a == 0x00 {
            self.f |= FLAG_Z
        }
        if self.a >= 0x80 {
            self.f |= FLAG_S
        }
    }

    fn cp_a_r(&mut self, val: u8) -> u8 {
        self.f &= 0xff - (FLAG_C | FLAG_PV | 0x28 | FLAG_H | FLAG_Z | FLAG_S);
        if self.a < val {
            self.f |= FLAG_C;
        }
        let high_bit = self.a & 0x80;
        if high_bit != (val & 0x80) {
            if high_bit == 0 {
                if self.a + 0x80 >= val {
                    self.f |= FLAG_PV;
                }
            } else {
                if self.a < val + 0x80 {
                    self.f |= FLAG_PV;
                }
            }
        }
        if (self.a & 0xf) < (val & 0xf) {
            self.f |= FLAG_H;
        }
        let res = self.a.wrapping_sub(val);
        self.f |= FLAG_N | (val & 0x28);
        if res == 0x00 {
            self.f |= FLAG_Z
        }
        if res >= 0x80 {
            self.f |= FLAG_S
        }
        res
    }

    fn ld_a_ir(&mut self, val: u8) {
        self.a = val;
        self.f &= 0xff - (FLAG_N | FLAG_PV | FLAG_H | 0x28 | FLAG_Z | FLAG_S);
        self.f |= self.a & 0x28;
        if self.iff2 == 1 {
            self.f |= FLAG_PV;
        }
        if self.a == 0x00 {
            self.f |= FLAG_Z
        }
        if self.a >= 0x80 {
            self.f |= FLAG_S
        }
    }

    fn or_a_r(&mut self, val: u8) {
        self.f &= 0xff - (FLAG_C | FLAG_N | 0x28 | FLAG_H | FLAG_Z | FLAG_S);
        self.a |= val;
        self.set_parity_a();
        self.f |= self.a & 0x28;
        if self.a == 0x00 {
            self.f |= FLAG_Z
        }
        if self.a >= 0x80 {
            self.f |= FLAG_S
        }
    }

    fn sbc_a_r(&mut self, val: u8) {
        let carry = self.f & FLAG_C;
        self.f &= 0xff - (FLAG_C | FLAG_PV | 0x28 | FLAG_H | FLAG_Z | FLAG_S);
        let a16 = self.a as u16;
        let val16 = val as u16;
        let c16 = carry as u16;
        if a16 < val16 + c16 {
            self.f |= FLAG_C;
        }
        let high_bit = self.a & 0x80;
        if high_bit != (val & 0x80) {
            if high_bit == 0 {
                if a16 + 0x80 >= val16 + c16 {
                    self.f |= FLAG_PV;
                }
            } else {
                if a16 < val16 + 0x80 + c16 {
                    self.f |= FLAG_PV;
                }
            }
        }
        if (self.a & 0xf) < (val & 0xf) + carry {
            self.f |= FLAG_H;
        }
        self.a = self.a.wrapping_sub(val).wrapping_sub(carry);
        self.f |= FLAG_N | (self.a & 0x28);
        if self.a == 0x00 {
            self.f |= FLAG_Z
        }
        if self.a >= 0x80 {
            self.f |= FLAG_S
        }
    }

    fn sub_a_r(&mut self, val: u8) {
        self.f &= 0xff - (FLAG_C | FLAG_PV | 0x28 | FLAG_H | FLAG_Z | FLAG_S);
        if self.a < val {
            self.f |= FLAG_C;
        }
        let high_bit = self.a & 0x80;
        if high_bit != (val & 0x80) {
            if high_bit == 0 {
                if self.a + 0x80 >= val {
                    self.f |= FLAG_PV;
                }
            } else {
                if self.a < val + 0x80 {
                    self.f |= FLAG_PV;
                }
            }
        }
        if (self.a & 0xf) < (val & 0xf) {
            self.f |= FLAG_H;
        }
        self.a = self.a.wrapping_sub(val);
        self.f |= FLAG_N | (self.a & 0x28);
        if self.a == 0x00 {
            self.f |= FLAG_Z
        }
        if self.a >= 0x80 {
            self.f |= FLAG_S
        }
    }

    fn xor_a_r(&mut self, val: u8) {
        self.f &= 0xff - (FLAG_C | FLAG_N | 0x28 | FLAG_H | FLAG_Z | FLAG_S);
        self.a ^= val;
        self.set_parity_a();
        self.f |= self.a & 0x28;
        if self.a == 0x00 {
            self.f |= FLAG_Z
        }
        if self.a >= 0x80 {
            self.f |= FLAG_S
        }
    }

    fn read_byte(&mut self, io: &mut dyn Z80IO) {
        self.data_bus = Some(io.peek_byte(self.pc));
        self.pc = self.pc.wrapping_add(1);
    }

    fn do_write(&mut self, io: &mut dyn Z80IO) {
        let addr = self.addr_bus.unwrap();
        let val = self.data_bus.unwrap();
        if self.memory_pin {
            io.write_byte(addr, val);
        }
        if self.io_pin {
            io.write_io(addr, val);
        }
    }

    fn prefixed_hl(&self) -> u16 {
        match self.prefix {
            Prefix::DD => self.ix,
            Prefix::FD => self.iy,
            Prefix::NONE => word(self.h, self.l),
        }
    }

    fn flag(&self, f: u8) -> bool {
        (self.f & f) != 0
    }

    fn prefixed_group_reg_r(&self, val: u8) -> u8 {
        match val {
            0 => self.b,
            1 => self.c,
            2 => self.d,
            3 => self.e,
            4 => self.get_prefixed_h(),
            5 => self.get_prefixed_l(),
            6 => self.data_bus.unwrap(),
            7 => self.a,
            _ => panic!("Invalid group_reg"),
        }
    }

    fn group_reg_w(&mut self, bit: u8, val: u8) {
        match bit {
            0 => self.b = val,
            1 => self.c = val,
            2 => self.d = val,
            3 => self.e = val,
            4 => self.h = val,
            5 => self.l = val,
            7 => self.a = val,
            _ => panic!("Invalid group_reg"),
        }
    }

    fn prefixed_group_reg_w(&mut self, bit: u8, val: u8) {
        match bit {
            0 => self.b = val,
            1 => self.c = val,
            2 => self.d = val,
            3 => self.e = val,
            4 => self.set_prefixed_h(val),
            5 => self.set_prefixed_l(val),
            7 => self.a = val,
            _ => panic!("Invalid group_reg"),
        }
    }

    fn cpid(&mut self, inc: bool) {
        let carry = self.f & FLAG_C;
        let mut res = self.cp_a_r(self.low_byte);
        if self.flag(FLAG_H) {
            res = res.wrapping_sub(1);
        }
        if inc {
            inc_word(&mut self.h, &mut self.l);
            self.wz = self.wz.wrapping_add(1);
        } else {
            dec_word(&mut self.h, &mut self.l);
            self.wz = self.wz.wrapping_sub(1);
        }
        dec_word(&mut self.b, &mut self.c);
        self.f &= 0xff - (FLAG_C | FLAG_PV | 0x28);
        self.f |= carry;
        if self.bc() != 0 {
            self.f |= FLAG_PV;
        }
        if (res & 8) != 0 {
            self.f |= 0x08;
        }
        if (res & 2) != 0 {
            self.f |= 0x20;
        }
    }

    fn cpidr(&mut self, inc: bool) {
        self.cpid(inc);
        if self.bc() != 0 && !self.flag(FLAG_Z) {
            self.wz = self.pc.wrapping_sub(1);
            self.pc = self.pc.wrapping_sub(2);
            self.f &= 0xff - 0x28;
            self.f |= ((self.pc >> 8) & 0x28) as u8;
        }
    }

    fn dec_reg(&mut self, mut reg: u8) -> u8 {
        self.f &= 0xff - (FLAG_PV | FLAG_H | 0x28 | FLAG_Z | FLAG_S);
        reg = reg.wrapping_sub(1);
        self.f |= (reg & 0x28) | FLAG_N;
        if (reg & 0x0f) == 0xf {
            self.f |= FLAG_H
        }
        if reg == 0x7f {
            self.f |= FLAG_PV
        }
        if reg == 0x00 {
            self.f |= FLAG_Z
        }
        if reg >= 0x80 {
            self.f |= FLAG_S
        }
        reg
    }

    fn inc_reg(&mut self, mut reg: u8) -> u8 {
        self.f &= 0xff - (FLAG_N | FLAG_PV | FLAG_H | 0x28 | FLAG_Z | FLAG_S);
        reg = reg.wrapping_add(1);
        self.f |= reg & 0x28;
        if (reg & 0x0f) == 0x0 {
            self.f |= FLAG_H
        }
        if reg == 0x80 {
            self.f |= FLAG_PV
        }
        if reg == 0x00 {
            self.f |= FLAG_Z
        }
        if reg >= 0x80 {
            self.f |= FLAG_S
        }
        reg
    }

    fn inid(&mut self, inc: bool) {
        if inc {
            inc_word(&mut self.h, &mut self.l);
            self.wz = self.bc().wrapping_add(1);
        } else {
            dec_word(&mut self.h, &mut self.l);
            self.wz = self.bc().wrapping_sub(1);
        }
        self.b = self.b.wrapping_sub(1);
        self.f &= 0xff - (FLAG_C | FLAG_N | FLAG_H | 0x28 | FLAG_Z | FLAG_S);
        self.f |= self.b & 0x28;

        let val = if inc {
            self.c.wrapping_add(1)
        } else {
            self.c.wrapping_sub(1)
        };
        let val16 = if inc { (self.c as u16) + 1 } else { val as u16 };
        self.set_parity(val.wrapping_add(self.low_byte) & 7 ^ self.b);
        if self.low_byte >= 0x80 {
            self.f |= FLAG_N
        }
        if (val16 + self.low_byte as u16) >= 0x100 {
            self.f |= FLAG_C | FLAG_H;
        }
        if self.b == 0 {
            self.f |= FLAG_Z
        };
        if self.b >= 0x80 {
            self.f |= FLAG_S
        };
    }

    fn inidr(&mut self, inc: bool) {
        self.inid(inc);
        if self.b != 0 {
            self.pc = self.pc.wrapping_sub(2);
            self.f &= 0xff - 0x28;
            self.f |= ((self.pc >> 8) & 0x28) as u8;

            self.initotidr_jump_flags();
        }
    }

    fn initotidr_jump_flags(&mut self) {
        let res: u8;

        if self.flag(FLAG_C) {
            self.f &= 0xff - FLAG_H;
            if (self.low_byte & 0x80) != 0 {
                res = (self.b.wrapping_sub(1) & 7) ^ 1;
                if (self.b & 0xf) == 0x0 {
                    self.f |= FLAG_H;
                }
            } else {
                res = (self.b.wrapping_add(1) & 7) ^ 1;
                if (self.b & 0xf) == 0xf {
                    self.f |= FLAG_H;
                }
            }
        } else {
            res = (self.b & 7) ^ 1;
        }

        if parity(res) {
            self.f ^= FLAG_PV;
        }
    }

    fn ldid(&mut self, inc: bool) {
        self.f &= 0xff - (FLAG_N | FLAG_PV | FLAG_H | 0x28);
        let res = self.a.wrapping_add(self.low_byte);
        if (res & 8) != 0 {
            self.f |= 0x08;
        }
        if (res & 2) != 0 {
            self.f |= 0x20;
        }
        if inc {
            inc_word(&mut self.h, &mut self.l);
            inc_word(&mut self.d, &mut self.e);
        } else {
            dec_word(&mut self.h, &mut self.l);
            dec_word(&mut self.d, &mut self.e);
        }
        dec_word(&mut self.b, &mut self.c);
        if word(self.b, self.c) != 0 {
            self.f |= FLAG_PV;
        }
    }

    fn ldidr(&mut self, inc: bool) {
        self.ldid(inc);
        if self.bc() != 0 {
            self.wz = self.pc.wrapping_sub(1);
            self.pc = self.pc.wrapping_sub(2);
            self.f &= 0xff - 0x28;
            self.f |= ((self.pc >> 8) & 0x28) as u8;
        }
    }

    fn outid(&mut self, inc: bool) {
        if inc {
            inc_word(&mut self.h, &mut self.l);
            self.wz = self.bc().wrapping_add(1);
        } else {
            dec_word(&mut self.h, &mut self.l);
            self.wz = self.bc().wrapping_sub(1);
        }
        self.f &= 0xff - (FLAG_C | FLAG_N | FLAG_H | 0x28 | FLAG_Z | FLAG_S);
        self.f |= self.b & 0x28;

        let res = self.l.wrapping_add(self.low_byte) & 7 ^ self.b;
        self.set_parity(res);
        if self.low_byte >= 0x80 {
            self.f |= FLAG_N
        }
        if (self.l as u16 + self.low_byte as u16) >= 0x100 {
            self.f |= FLAG_C | FLAG_H;
        }
        if self.b == 0 {
            self.f |= FLAG_Z
        };
        if self.b >= 0x80 {
            self.f |= FLAG_S
        };
    }

    fn otidr(&mut self, inc: bool) {
        self.outid(inc);
        if self.b != 0 {
            self.pc = self.pc.wrapping_sub(2);
            self.f &= 0xff - 0x28;
            self.f |= ((self.pc >> 8) & 0x28) as u8;

            self.initotidr_jump_flags();
        }
    }

    fn rl(&mut self, mut val: u8) -> u8 {
        let carry = self.flag(FLAG_C);
        self.f &= 0xff - (FLAG_C | FLAG_N | FLAG_H | 0x28 | FLAG_Z | FLAG_S);
        if bit_set(val, 7) {
            self.f |= FLAG_C;
        }
        val <<= 1;
        self.f |= val & 0x28;
        if carry {
            val |= 0x01;
        }
        self.set_parity(val);
        if val == 0x00 {
            self.f |= FLAG_Z
        }
        if val >= 0x80 {
            self.f |= FLAG_S
        }

        val
    }

    fn rlc(&mut self, mut val: u8) -> u8 {
        let high_bit = bit_set(val, 7);
        val <<= 1;
        self.f &= 0xff - (FLAG_C | FLAG_N | FLAG_H | 0x28 | FLAG_Z | FLAG_S);
        self.f |= val & 0x28;
        if high_bit {
            val |= 0x01;
            self.f |= FLAG_C;
        }
        self.set_parity(val);
        if val == 0x00 {
            self.f |= FLAG_Z
        }
        if val >= 0x80 {
            self.f |= FLAG_S
        }

        val
    }

    fn rr(&mut self, mut val: u8) -> u8 {
        let carry = self.flag(FLAG_C);
        self.f &= 0xff - (FLAG_C | FLAG_N | FLAG_H | 0x28 | FLAG_Z | FLAG_S);
        if bit_set(val, 0) {
            self.f |= FLAG_C;
        }
        val >>= 1;
        self.f |= val & 0x28;
        if carry {
            val |= 0x80;
        }
        self.set_parity(val);
        if val == 0x00 {
            self.f |= FLAG_Z
        }
        if val >= 0x80 {
            self.f |= FLAG_S
        }

        val
    }

    fn rrc(&mut self, mut val: u8) -> u8 {
        let low_bit = bit_set(val, 0);
        val >>= 1;
        self.f &= 0xff - (FLAG_C | FLAG_N | FLAG_H | 0x28 | FLAG_Z | FLAG_S);
        self.f |= val & 0x28;
        if low_bit {
            val |= 0x80;
            self.f |= FLAG_C;
        }
        self.set_parity(val);
        if val == 0x00 {
            self.f |= FLAG_Z
        }
        if val >= 0x80 {
            self.f |= FLAG_S
        }

        val
    }

    fn sla(&mut self, mut val: u8) -> u8 {
        let high_bit = bit_set(val, 7);
        val <<= 1;
        self.f &= 0xff - (FLAG_C | FLAG_N | FLAG_H | 0x28 | FLAG_Z | FLAG_S);
        self.f |= val & 0x28;
        if high_bit {
            self.f |= FLAG_C;
        }
        self.set_parity(val);
        if val == 0x00 {
            self.f |= FLAG_Z
        }
        if val >= 0x80 {
            self.f |= FLAG_S
        }

        val
    }

    fn sll(&mut self, mut val: u8) -> u8 {
        let high_bit = bit_set(val, 7);
        val <<= 1;
        val |= 1;
        self.f &= 0xff - (FLAG_C | FLAG_N | FLAG_H | 0x28 | FLAG_Z | FLAG_S);
        self.f |= val & 0x28;
        if high_bit {
            self.f |= FLAG_C;
        }
        self.set_parity(val);
        if val == 0x00 {
            self.f |= FLAG_Z
        }
        if val >= 0x80 {
            self.f |= FLAG_S
        }

        val
    }

    fn sra(&mut self, mut val: u8) -> u8 {
        let low_bit = bit_set(val, 0);
        val >>= 1;
        val |= (val & 0x40) * 2; // keep sign
        self.f &= 0xff - (FLAG_C | FLAG_N | FLAG_H | 0x28 | FLAG_Z | FLAG_S);
        self.f |= val & 0x28;
        if low_bit {
            self.f |= FLAG_C;
        }
        self.set_parity(val);
        if val == 0x00 {
            self.f |= FLAG_Z
        }
        if val >= 0x80 {
            self.f |= FLAG_S
        }

        val
    }

    fn srl(&mut self, mut val: u8) -> u8 {
        let low_bit = bit_set(val, 0);
        val >>= 1;
        self.f &= 0xff - (FLAG_C | FLAG_N | FLAG_H | 0x28 | FLAG_Z | FLAG_S);
        self.f |= val & 0x28;
        if low_bit {
            self.f |= FLAG_C;
        }
        self.set_parity(val);
        if val == 0x00 {
            self.f |= FLAG_Z
        }
        if val >= 0x80 {
            self.f |= FLAG_S
        }

        val
    }

    fn set_parity(&mut self, val: u8) {
        if parity(val) {
            self.f |= FLAG_PV;
        } else {
            self.f &= 0xff - FLAG_PV;
        }
    }

    fn set_parity_a(&mut self) {
        self.set_parity(self.a);
    }

    #[cfg(test)]
    pub fn pin_state(&self) -> String {
        format!(
            "{}{}{}{}",
            if self.read_pin { 'r' } else { '-' },
            if self.write_pin { 'w' } else { '-' },
            if self.memory_pin { 'm' } else { '-' },
            if self.io_pin { 'i' } else { '-' },
        )
    }

    fn fetch_high(&mut self, io: &mut dyn Z80IO) {
        self.read_byte(io);
        self.read_pin = false;
        self.memory_pin = false;
        self.high_byte = self.data_bus.unwrap();
    }

    fn fetch_low(&mut self, io: &mut dyn Z80IO) {
        self.read_byte(io);
        self.read_pin = false;
        self.memory_pin = false;
        self.low_byte = self.data_bus.unwrap();
    }

    fn cycle_write_u8(&mut self, val: u8) {
        self.data_bus = Some(val);
        self.write_pin = true;
        self.memory_pin = true;
    }

    fn get_prefixed_h(&self) -> u8 {
        match self.prefix {
            Prefix::DD => (self.ix >> 8) as u8,
            Prefix::FD => (self.iy >> 8) as u8,
            Prefix::NONE => self.h,
        }
    }

    fn get_prefixed_l(&self) -> u8 {
        match self.prefix {
            Prefix::DD => self.ix as u8,
            Prefix::FD => self.iy as u8,
            Prefix::NONE => self.l,
        }
    }

    fn set_prefixed_h(&mut self, val: u8) {
        match self.prefix {
            Prefix::DD => self.ix = (self.ix & 0x00ff) | ((val as u16) << 8),
            Prefix::FD => self.iy = (self.iy & 0x00ff) | ((val as u16) << 8),
            Prefix::NONE => self.h = val,
        }
    }

    fn set_prefixed_l(&mut self, val: u8) {
        match self.prefix {
            Prefix::DD => self.ix = (self.ix & 0xff00) | (val as u16),
            Prefix::FD => self.iy = (self.iy & 0xff00) | (val as u16),
            Prefix::NONE => self.l = val,
        }
    }

    fn unwrite(&mut self) {
        self.data_bus = None;
        self.write_pin = false;
        self.memory_pin = false;
        self.io_pin = false;
    }

    // APIs
    pub fn bc(&self) -> u16 {
        word(self.b, self.c)
    }

    pub fn de(&self) -> u16 {
        word(self.d, self.e)
    }

    pub fn hl(&self) -> u16 {
        word(self.h, self.l)
    }

    pub fn assert_irq(&mut self, val: u8) {
        self.irq_req = true;
        self.curr_irq_data = val;
    }

    pub fn reset(&mut self) {
        self.phase = FDEPhase::Execute;
        self.microcodes = vec![Cycle::Nop, Cycle::Nop, Cycle::Nop];
        self.ei = 0;
        self.iff1 = 0;
        self.iff2 = 0;
        self.pc = 0;
        self.i = 0;
        self.r = 0;
        self.im = 0;
        self.addr_bus = Some(self.pc);
        self.data_bus = None;
        self.read_pin = false;
        self.write_pin = false;
        self.memory_pin = false;
        self.io_pin = false;
    }

    pub fn step(&mut self, io: &mut dyn Z80IO) -> u32 {
        self.tick(io);
        let mut cyc = 1;
        while self.phase != FDEPhase::Init {
            self.tick(io);
            cyc += 1;
        }
        cyc
    }

    pub fn tick(&mut self, io: &mut dyn Z80IO) {
        match self.phase {
            FDEPhase::Init => {
                self.phase = FDEPhase::ReadMem;
                self.addr_bus = Some(self.pc);
                self.data_bus = None;
                self.read_pin = false;
                self.write_pin = false;
                self.memory_pin = false;
                self.io_pin = false;
                self.p = 0;
                self.init_pc = self.pc;
                self.is_ext = false;
                self.prefix = Prefix::NONE;
                self.curr_op = 0;
            }
            FDEPhase::ReadMem => {
                self.phase = FDEPhase::Fetch;
                self.read_pin = true;
                self.memory_pin = true;
            }
            FDEPhase::Fetch => {
                self.phase = FDEPhase::Execute;

                if !self.is_im0 {
                    self.read_byte(io);
                    self.addr_bus = Some(word(self.i, self.r));
                }
                let op = self.data_bus.unwrap();

                self.group_8 = (op / 8) % 8;
                self.group_1 = op % 8;

                self.microcodes = match op {
                    // ld bc, nn | ld de, nn | ld hl, nn | ld sp, nn | jp nz, nn | jp nn | jp z, nn | jp nc, nn | jp c, nn | jp pe, nn | jp po, nn | jp p, nn | jp m, nn
                    0x01 | 0x11 | 0x21 | 0x31 | 0xc2 | 0xc3 | 0xca | 0xd2 | 0xda | 0xe2 | 0xea
                    | 0xf2 | 0xfa => vec![
                        Cycle::Init,
                        Cycle::ReadMem,
                        Cycle::FetchLow,
                        Cycle::Init,
                        Cycle::ReadMem,
                        Cycle::FetchHigh,
                    ],
                    // ld (bc), a
                    0x02 => vec![Cycle::AddrIsBC, Cycle::WriteA, Cycle::Unwrite],
                    // inc bc | dec bc | inc de | dec de | inc hl | dec hl | inc sp | dec sp | ld sp, hl
                    0x03 | 0x0b | 0x13 | 0x1b | 0x23 | 0x2b | 0x33 | 0x3b | 0xf9 => {
                        vec![Cycle::Nop, Cycle::Nop]
                    }
                    // ld b, n | ld c, n | ld d, n | ld e, n | ld h, n | ld l, n | ld a, n | add a, n | prefix | adc a, n | sub n | sbc a, n | and n | prefix | xor n | or n | cp n
                    0x06 | 0x0e | 0x16 | 0x1e | 0x26 | 0x2e | 0x3e | 0xc6 | 0xcb | 0xce | 0xd6
                    | 0xde | 0xe6 | 0xed | 0xee | 0xf6 | 0xfe => {
                        vec![Cycle::Init, Cycle::ReadMem, Cycle::FetchLow]
                    }
                    // add hl, bc | add hl, de | add hl, hl | add hl, sp
                    0x09 | 0x19 | 0x29 | 0x39 => vec![
                        Cycle::Nop,
                        Cycle::Nop,
                        Cycle::Nop,
                        Cycle::Nop,
                        Cycle::Nop,
                        Cycle::Nop,
                        Cycle::Nop,
                    ],
                    // ld a, (bc)
                    0x0a => vec![Cycle::AddrIsBC, Cycle::ReadMem, Cycle::PeekByte],
                    // djnz d
                    0x10 => vec![Cycle::Nop, Cycle::Init, Cycle::ReadMem, Cycle::JrCond],
                    // ld (de), a
                    0x12 => vec![Cycle::AddrIsDE, Cycle::WriteA, Cycle::Unwrite],
                    // jr d | jr nz, d | jr z, d | jr nc, d | jr c, d
                    0x18 | 0x20 | 0x28 | 0x30 | 0x38 => {
                        vec![Cycle::Init, Cycle::ReadMem, Cycle::JrCond]
                    }
                    // ld a, (de)
                    0x1a => vec![Cycle::AddrIsDE, Cycle::ReadMem, Cycle::PeekByte],
                    // ld (nn), hl
                    0x22 => vec![
                        Cycle::Init,
                        Cycle::ReadMem,
                        Cycle::FetchLow,
                        Cycle::Init,
                        Cycle::ReadMem,
                        Cycle::FetchHigh,
                        Cycle::SetWordLow,
                        Cycle::WritePrefixedL,
                        Cycle::Unwrite,
                        Cycle::SetWordHigh,
                        Cycle::WritePrefixedH,
                        Cycle::Unwrite,
                    ],
                    // ld hl, (nn)
                    0x2a => vec![
                        Cycle::Init,
                        Cycle::ReadMem,
                        Cycle::FetchLow,
                        Cycle::Init,
                        Cycle::ReadMem,
                        Cycle::FetchHigh,
                        Cycle::IndInitLo,
                        Cycle::ReadMem,
                        Cycle::FetchIndLow,
                        Cycle::IndInitHi,
                        Cycle::ReadMem,
                        Cycle::FetchIndHigh,
                    ],
                    // ld (nn), a
                    0x32 => vec![
                        Cycle::Init,
                        Cycle::ReadMem,
                        Cycle::FetchLow,
                        Cycle::Init,
                        Cycle::ReadMem,
                        Cycle::FetchHigh,
                        Cycle::SetWordLow,
                        Cycle::WriteA,
                        Cycle::Unwrite,
                    ],
                    // inc (hl)
                    0x34 => vec![
                        Cycle::AddrIsPrefixedHL,
                        Cycle::ReadMem,
                        Cycle::PeekByte,
                        Cycle::IncLow,
                        Cycle::Nop,
                        Cycle::WriteLow,
                        Cycle::Unwrite,
                    ],
                    // dec (hl)
                    0x35 => vec![
                        Cycle::AddrIsPrefixedHL,
                        Cycle::ReadMem,
                        Cycle::PeekByte,
                        Cycle::DecLow,
                        Cycle::Nop,
                        Cycle::WriteLow,
                        Cycle::Unwrite,
                    ],
                    // ld (hl), n
                    0x36 => vec![
                        Cycle::Init,
                        Cycle::ReadMem,
                        Cycle::FetchHigh,
                        Cycle::AddrIsPrefixedHL,
                        Cycle::WriteHigh,
                        Cycle::Unwrite,
                    ],
                    // ld a, (nn)
                    0x3a => vec![
                        Cycle::Init,
                        Cycle::ReadMem,
                        Cycle::FetchLow,
                        Cycle::Init,
                        Cycle::ReadMem,
                        Cycle::FetchHigh,
                        Cycle::SetWordLow,
                        Cycle::ReadMem,
                        Cycle::PeekByte,
                    ],
                    // ld b, (hl) | ld c, (hl) | ld d, (hl) | ld e, (hl) | ld h, (hl) | ld l, (hl) | ld a, (hl) | add a, (hl) | adc a, (hl) | sub a, (hl) | sbc a, (hl) | and (hl) | xor (hl) | or (hl) | cp (hl)
                    0x46 | 0x4e | 0x56 | 0x5e | 0x66 | 0x6e | 0x7e | 0x86 | 0x8e | 0x96 | 0x9e
                    | 0xa6 | 0xae | 0xb6 | 0xbe => {
                        vec![Cycle::AddrIsPrefixedHL, Cycle::ReadMem, Cycle::PeekByte]
                    }
                    // ld (hl), b
                    0x70 => vec![Cycle::AddrIsPrefixedHL, Cycle::WriteB, Cycle::Unwrite],
                    // ld (hl), c
                    0x71 => vec![Cycle::AddrIsPrefixedHL, Cycle::WriteC, Cycle::Unwrite],
                    // ld (hl), d
                    0x72 => vec![Cycle::AddrIsPrefixedHL, Cycle::WriteD, Cycle::Unwrite],
                    // ld (hl), e
                    0x73 => vec![Cycle::AddrIsPrefixedHL, Cycle::WriteE, Cycle::Unwrite],
                    // ld (hl), h
                    0x74 => vec![Cycle::AddrIsPrefixedHL, Cycle::WriteH, Cycle::Unwrite],
                    // ld (hl), l
                    0x75 => vec![Cycle::AddrIsPrefixedHL, Cycle::WriteL, Cycle::Unwrite],
                    // ld (hl), a
                    0x77 => vec![Cycle::AddrIsPrefixedHL, Cycle::WriteA, Cycle::Unwrite],
                    // ret nz | ret z | ret nc | ret c | ret po | ret pe | ret p | ret m
                    0xc0 | 0xc8 | 0xd0 | 0xd8 | 0xe0 | 0xe8 | 0xf0 | 0xf8 => vec![Cycle::CheckRet],
                    // pop bc | ret  | pop de | pop hl | pop af
                    0xc1 | 0xc9 | 0xd1 | 0xe1 | 0xf1 => vec![
                        Cycle::PopStackLow,
                        Cycle::ReadMem,
                        Cycle::PeekLow,
                        Cycle::PopStackHigh,
                        Cycle::ReadMem,
                        Cycle::PeekHigh,
                    ],
                    // call nz, nn | call z, nn | call nc, nn | call c, nn | call pe, nn | call po, nn | call p, nn | call m, nn
                    0xc4 | 0xcc | 0xd4 | 0xdc | 0xe4 | 0xec | 0xf4 | 0xfc => vec![
                        Cycle::Init,
                        Cycle::ReadMem,
                        Cycle::FetchLow,
                        Cycle::Init,
                        Cycle::ReadMem,
                        Cycle::CallCond,
                    ],
                    // push bc
                    0xc5 => vec![
                        Cycle::Nop,
                        Cycle::PushStack,
                        Cycle::WriteB,
                        Cycle::Unwrite,
                        Cycle::PushStack,
                        Cycle::WriteC,
                        Cycle::Unwrite,
                    ],
                    // rst $00 | rst $08 | rst $10 | rst $18 | rst $20 | rst $28 | rst $30 | rst $38
                    0xc7 | 0xcf | 0xd7 | 0xdf | 0xe7 | 0xef | 0xf7 | 0xff => vec![
                        Cycle::Nop,
                        Cycle::PushStack,
                        Cycle::WritePCHigh,
                        Cycle::Unwrite,
                        Cycle::PushStack,
                        Cycle::WritePCLow,
                        Cycle::Unwrite,
                    ],
                    // call nn
                    0xcd => vec![
                        Cycle::Init,
                        Cycle::ReadMem,
                        Cycle::FetchLow,
                        Cycle::Init,
                        Cycle::ReadMem,
                        Cycle::FetchHigh,
                        Cycle::Nop,
                        Cycle::PushStack,
                        Cycle::WritePCHigh,
                        Cycle::Unwrite,
                        Cycle::PushStack,
                        Cycle::WritePCLow,
                        Cycle::Unwrite,
                    ],
                    // out (n), a
                    0xd3 => vec![
                        Cycle::Init,
                        Cycle::ReadMem,
                        Cycle::FetchLow,
                        Cycle::AddrIsALow,
                        Cycle::Nop,
                        Cycle::WriteIOU8(self.a),
                        Cycle::Unwrite,
                    ],
                    // push de
                    0xd5 => vec![
                        Cycle::Nop,
                        Cycle::PushStack,
                        Cycle::WriteD,
                        Cycle::Unwrite,
                        Cycle::PushStack,
                        Cycle::WriteE,
                        Cycle::Unwrite,
                    ],
                    // in a, (n)
                    0xdb => vec![
                        Cycle::Init,
                        Cycle::ReadMem,
                        Cycle::FetchLow,
                        Cycle::AddrIsALow,
                        Cycle::Nop,
                        Cycle::ReadIO,
                        Cycle::PeekByte,
                    ],
                    // prefix
                    0xdd | 0xfd => vec![Cycle::Init, Cycle::ReadMem],
                    // ex (sp), hl
                    0xe3 => vec![
                        Cycle::AddrIsSP0,
                        Cycle::ReadMem,
                        Cycle::PeekLow,
                        Cycle::AddrIsSP1,
                        Cycle::ReadMem,
                        Cycle::PeekHigh,
                        Cycle::Nop,
                        Cycle::AddrIsSP0,
                        Cycle::WritePrefixedL,
                        Cycle::Unwrite,
                        Cycle::AddrIsSP1,
                        Cycle::WritePrefixedH,
                        Cycle::Unwrite,
                        Cycle::Nop,
                        Cycle::Nop,
                    ],
                    // push hl
                    0xe5 => vec![
                        Cycle::Nop,
                        Cycle::PushStack,
                        Cycle::WritePrefixedH,
                        Cycle::Unwrite,
                        Cycle::PushStack,
                        Cycle::WritePrefixedL,
                        Cycle::Unwrite,
                    ],
                    // push af
                    0xf5 => vec![
                        Cycle::Nop,
                        Cycle::PushStack,
                        Cycle::WriteA,
                        Cycle::Unwrite,
                        Cycle::PushStack,
                        Cycle::WriteF,
                        Cycle::Unwrite,
                    ],
                    _ => vec![],
                };
                if !self.is_im0 {
                    self.microcodes.insert(0, Cycle::Nop);
                }
                if self.prefix != Prefix::NONE {
                    match op {
                        0x34
                        | 0x35
                        | 0x46
                        | 0x4e
                        | 0x56
                        | 0x5e
                        | 0x66
                        | 0x6e
                        | 0x70..=0x75
                        | 0x77
                        | 0x7e
                        | 0x86
                        | 0x8e
                        | 0x96
                        | 0x9e
                        | 0xa6
                        | 0xae
                        | 0xb6
                        | 0xbe => {
                            self.microcodes.insert(1, Cycle::Init);
                            self.microcodes.insert(2, Cycle::ReadMem);
                            self.microcodes.insert(3, Cycle::FetchLow);
                            self.microcodes.insert(4, Cycle::Nop);
                            self.microcodes.insert(5, Cycle::Nop);
                            self.microcodes.insert(6, Cycle::Nop);
                            self.microcodes.insert(7, Cycle::Nop);
                            self.microcodes.insert(8, Cycle::Nop);
                        }
                        0x36 => {
                            self.microcodes.insert(1, Cycle::Init);
                            self.microcodes.insert(2, Cycle::ReadMem);
                            self.microcodes.insert(3, Cycle::FetchLow);
                            self.microcodes.insert(4, Cycle::Nop);
                            self.microcodes.insert(5, Cycle::Nop);
                        }
                        0xcb => {
                            self.microcodes.push(Cycle::Init);
                            self.microcodes.push(Cycle::ReadMem);
                            self.microcodes.push(Cycle::FetchHigh);
                            // undo $cb incrementing r
                            self.r = self.r.wrapping_sub(1);
                            if self.r == 0xff {
                                self.r = 0x7f
                            }
                        }
                        _ => (),
                    }
                }
                self.curr_op = (self.curr_op&0xff00)|(op as u16);
                self.set_q = match op {
                    0x04
                    | 0x05
                    | 0x0c
                    | 0x0d
                    | 0x14
                    | 0x15
                    | 0x1c
                    | 0x1d
                    | 0x24
                    | 0x25
                    | 0x2c
                    | 0x2d
                    | 0x34
                    | 0x35
                    | 0x3c
                    | 0x3d
                    | 0x07
                    | 0x0f
                    | 0x17
                    | 0x1f
                    | 0x27
                    | 0x2f
                    | 0x37
                    | 0x3f
                    | 0x09
                    | 0x19
                    | 0x29
                    | 0x39
                    | 0x80..=0xbf
                    | 0xc6
                    | 0xce
                    | 0xd6
                    | 0xde
                    | 0xe6
                    | 0xee
                    | 0xf6
                    | 0xfe => true,
                    _ => false,
                };

                self.read_pin = false;
                self.memory_pin = false;
            }
            FDEPhase::Execute => {
                let microcode = self.microcodes.remove(0);
                match microcode {
                    Cycle::AddrIsALow => {
                        self.addr_bus = Some(word(self.a, self.low_byte));
                        self.data_bus = None;
                    }
                    Cycle::AddrIsBC => {
                        self.addr_bus = Some(self.bc());
                        self.data_bus = None;
                    }
                    Cycle::AddrIsDE => {
                        self.addr_bus = Some(self.de());
                        self.data_bus = None;
                    }
                    Cycle::AddrIsDecBC => {
                        self.b = self.b.wrapping_sub(1);
                        self.addr_bus = Some(self.bc());
                        self.data_bus = None;
                    }
                    Cycle::AddrIsIM2High => {
                        self.addr_bus = Some(self.addr_bus.unwrap().wrapping_add(1));
                        self.data_bus = None;
                    }
                    Cycle::AddrIsIM2Low => {
                        let addr = word(self.i, self.curr_irq_data);
                        self.addr_bus = Some(addr);
                        self.data_bus = None;
                    }
                    Cycle::AddrIsPrefixedHL => {
                        self.addr_bus = match self.prefix {
                            Prefix::DD => {
                                self.wz = if self.low_byte >= 0x80 {
                                    self.ix.wrapping_sub(0x100 - (self.low_byte as u16))
                                } else {
                                    self.ix.wrapping_add(self.low_byte as u16)
                                };
                                Some(self.wz)
                            }
                            Prefix::FD => {
                                self.wz = if self.low_byte >= 0x80 {
                                    self.iy.wrapping_sub(0x100 - (self.low_byte as u16))
                                } else {
                                    self.iy.wrapping_add(self.low_byte as u16)
                                };
                                Some(self.wz)
                            }
                            Prefix::NONE => Some(self.hl()),
                        };
                        self.data_bus = None;
                    }
                    Cycle::AddrIsSP0 => {
                        self.addr_bus = Some(self.sp);
                        self.data_bus = None;
                    }
                    Cycle::AddrIsSP1 => {
                        self.addr_bus = Some(self.sp.wrapping_add(1));
                        self.data_bus = None;
                    }
                    Cycle::CallCond => {
                        self.fetch_high(io);
                        self.wz = word(self.high_byte, self.low_byte);
                        if match self.group_8 {
                            0 => !self.flag(FLAG_Z),
                            1 => self.flag(FLAG_Z),
                            2 => !self.flag(FLAG_C),
                            3 => self.flag(FLAG_C),
                            4 => !self.flag(FLAG_PV),
                            5 => self.flag(FLAG_PV),
                            6 => !self.flag(FLAG_S),
                            7 => self.flag(FLAG_S),
                            _ => panic!("Invalid CheckRet"),
                        } {
                            self.microcodes.push(Cycle::Nop);
                            self.microcodes.push(Cycle::PushStack);
                            self.microcodes.push(Cycle::WritePCHigh);
                            self.microcodes.push(Cycle::Unwrite);
                            self.microcodes.push(Cycle::PushStack);
                            self.microcodes.push(Cycle::WritePCLow);
                            self.microcodes.push(Cycle::Unwrite);
                            self.curr_op = OP_CALL_COND;
                        }
                    }
                    Cycle::CheckIrqData => self.data_bus = Some(self.curr_irq_data),
                    Cycle::CheckRep => {
                        self.unwrite();
                        if match self.group_1 {
                            0 => self.bc() != 1,
                            1 => self.bc() != 1 && self.a != self.low_byte,
                            2 => self.b != 1,
                            3 => self.b != 0,
                            _ => panic!("Invalid CheckRep"),
                        } {
                            self.microcodes.push(Cycle::Nop);
                            self.microcodes.push(Cycle::Nop);
                            self.microcodes.push(Cycle::Nop);
                            self.microcodes.push(Cycle::Nop);
                            self.microcodes.push(Cycle::Nop);
                        }
                    }
                    Cycle::CheckRet => {
                        if match self.group_8 {
                            0 => !self.flag(FLAG_Z),
                            1 => self.flag(FLAG_Z),
                            2 => !self.flag(FLAG_C),
                            3 => self.flag(FLAG_C),
                            4 => !self.flag(FLAG_PV),
                            5 => self.flag(FLAG_PV),
                            6 => !self.flag(FLAG_S),
                            7 => self.flag(FLAG_S),
                            _ => panic!("Invalid CheckRet"),
                        } {
                            self.microcodes.push(Cycle::PopStackLow);
                            self.microcodes.push(Cycle::ReadMem);
                            self.microcodes.push(Cycle::PeekLow);
                            self.microcodes.push(Cycle::PopStackHigh);
                            self.microcodes.push(Cycle::ReadMem);
                            self.microcodes.push(Cycle::PeekHigh);
                            self.curr_op = OP_CHECK_RET;
                        }
                    }
                    Cycle::DecLow => {
                        self.low_byte = self.dec_reg(self.prefixed_group_reg_r(self.group_8));
                        self.data_bus = None;
                    }
                    Cycle::FetchHigh => self.fetch_high(io),
                    Cycle::FetchIndHigh => {
                        self.wz = self.addr_bus.unwrap();
                        self.data_bus = Some(io.peek_byte(self.wz));
                        self.read_pin = false;
                        self.memory_pin = false;
                        self.high_byte = self.data_bus.unwrap();
                    }
                    Cycle::FetchIndLow => {
                        self.data_bus = Some(io.peek_byte(self.addr_bus.unwrap()));
                        self.read_pin = false;
                        self.memory_pin = false;
                        self.low_byte = self.data_bus.unwrap();
                    }
                    Cycle::FetchLow => self.fetch_low(io),
                    Cycle::IncLow => {
                        self.low_byte = self.inc_reg(self.prefixed_group_reg_r(self.group_8));
                        self.data_bus = None;
                    }
                    Cycle::IndInitLo => {
                        self.ind_addr = word(self.high_byte, self.low_byte);
                        self.addr_bus = Some(self.ind_addr);
                        self.data_bus = None;
                    }
                    Cycle::IndInitHi => {
                        self.ind_addr = self.ind_addr.wrapping_add(1);
                        self.addr_bus = Some(self.ind_addr);
                        self.data_bus = None;
                    }
                    Cycle::Init => {
                        self.addr_bus = Some(self.pc);
                        self.data_bus = None;
                    }
                    Cycle::JrCond => {
                        self.fetch_low(io);
                        let jump = match self.group_8 {
                            2 => {
                                self.b = self.b.wrapping_sub(1);
                                self.b != 0
                            }
                            3 => true,
                            4 => !self.flag(FLAG_Z),
                            5 => self.flag(FLAG_Z),
                            6 => !self.flag(FLAG_C),
                            7 => self.flag(FLAG_C),
                            _ => panic!("Invalid JrCond"),
                        };
                        if jump {
                            self.microcodes.push(Cycle::Nop);
                            self.microcodes.push(Cycle::Nop);
                            self.microcodes.push(Cycle::Nop);
                            self.microcodes.push(Cycle::Nop);
                            self.microcodes.push(Cycle::Nop);
                            self.pc = if self.low_byte >= 0x80 {
                                self.pc.wrapping_sub(0x100 - self.low_byte as u16)
                            } else {
                                self.pc.wrapping_add(self.low_byte as u16)
                            };
                            self.wz = self.pc;
                        }
                    }
                    Cycle::Nop => {
                        if self.is_im0 {
                            self.is_im0 = false;
                        } else {
                            self.data_bus = None;
                        }
                    }
                    Cycle::PeekByte => {
                        if self.memory_pin {
                            self.data_bus = Some(io.peek_byte(self.addr_bus.unwrap()));
                            self.memory_pin = false;
                        }
                        if self.io_pin {
                            self.data_bus = Some(io.peek_io(self.addr_bus.unwrap()));
                            self.io_pin = false;
                        }
                        self.read_pin = false;
                    }
                    Cycle::PeekHigh => {
                        self.high_byte = io.peek_byte(self.addr_bus.unwrap());
                        self.data_bus = Some(self.high_byte);
                        self.read_pin = false;
                        self.memory_pin = false;
                    }
                    Cycle::PeekLow => {
                        if self.memory_pin {
                            self.low_byte = io.peek_byte(self.addr_bus.unwrap());
                            self.memory_pin = false;
                        }
                        if self.io_pin {
                            self.low_byte = io.peek_io(self.addr_bus.unwrap());
                            self.io_pin = false;
                        }
                        self.data_bus = Some(self.low_byte);
                        self.read_pin = false;
                    }
                    Cycle::PopStackHigh => {
                        self.addr_bus = Some(self.sp);
                        self.sp = self.sp.wrapping_add(1);
                        self.data_bus = None;
                    }
                    Cycle::PopStackLow => {
                        self.addr_bus = Some(self.sp);
                        self.sp = self.sp.wrapping_add(1);
                    }
                    Cycle::PushStack => {
                        self.sp = self.sp.wrapping_sub(1);
                        self.addr_bus = Some(self.sp);
                        self.data_bus = None;
                    }
                    Cycle::ReadIO => {
                        self.read_pin = true;
                        self.io_pin = true;
                    }
                    Cycle::ReadMem => {
                        self.read_pin = true;
                        self.memory_pin = true;
                    }
                    Cycle::ResExt => {
                        self.low_byte = self.data_bus.unwrap() & mask(self.group_8);
                        self.data_bus = None;
                    }
                    Cycle::RLData => {
                        self.low_byte = self.rl(self.data_bus.unwrap());
                        self.data_bus = None;
                    }
                    Cycle::RLCData => {
                        self.low_byte = self.rlc(self.data_bus.unwrap());
                        self.data_bus = None;
                    }
                    Cycle::RLDData => {
                        let low_a = self.a & 0x0f;
                        let val = self.data_bus.unwrap();
                        self.a = (self.a & 0xf0) | ((val & 0xf0) >> 4);
                        self.low_byte = low_a | ((val & 0x0f) << 4);
                        self.data_bus = None;
                        self.f &= 0xff - (FLAG_N | FLAG_H | 0x28 | FLAG_Z | FLAG_S);
                        self.set_parity_a();
                        self.f |= self.a & 0x28;
                        if self.a == 0x00 {
                            self.f |= FLAG_Z
                        }
                        if self.a >= 0x80 {
                            self.f |= FLAG_S
                        }
                        self.wz = self.hl().wrapping_add(1);
                    }
                    Cycle::RRData => {
                        self.low_byte = self.rr(self.data_bus.unwrap());
                        self.data_bus = None;
                    }
                    Cycle::RRCData => {
                        self.low_byte = self.rrc(self.data_bus.unwrap());
                        self.data_bus = None;
                    }
                    Cycle::RRDData => {
                        let low_a = self.a & 0x0f;
                        let val = self.data_bus.unwrap();
                        self.a = (self.a & 0xf0) | (val & 0x0f);
                        self.low_byte = (low_a << 4) | ((val & 0xf0) >> 4);
                        self.data_bus = None;
                        self.f &= 0xff - (FLAG_N | FLAG_H | 0x28 | FLAG_Z | FLAG_S);
                        self.set_parity_a();
                        self.f |= self.a & 0x28;
                        if self.a == 0x00 {
                            self.f |= FLAG_Z
                        }
                        if self.a >= 0x80 {
                            self.f |= FLAG_S
                        }
                        self.wz = self.hl().wrapping_add(1);
                    }
                    Cycle::SetExt => {
                        self.low_byte = self.data_bus.unwrap() | flag(self.group_8);
                        self.data_bus = None;
                    }
                    Cycle::SetWordHigh => {
                        self.data_bus = None;
                        self.addr_bus = Some(self.addr_bus.unwrap().wrapping_add(1));
                    }
                    Cycle::SetWordLow => {
                        self.data_bus = None;
                        self.addr_bus = Some(word(self.high_byte, self.low_byte));
                    }
                    Cycle::SLAData => {
                        self.low_byte = self.sla(self.data_bus.unwrap());
                        self.data_bus = None;
                    }
                    Cycle::SLLData => {
                        self.low_byte = self.sll(self.data_bus.unwrap());
                        self.data_bus = None;
                    }
                    Cycle::SRAData => {
                        self.low_byte = self.sra(self.data_bus.unwrap());
                        self.data_bus = None;
                    }
                    Cycle::SRLData => {
                        self.low_byte = self.srl(self.data_bus.unwrap());
                        self.data_bus = None;
                    }
                    Cycle::Unwrite => self.unwrite(),
                    Cycle::WriteA => self.cycle_write_u8(self.a),
                    Cycle::WriteB => self.cycle_write_u8(self.b),
                    Cycle::WriteC => self.cycle_write_u8(self.c),
                    Cycle::WriteD => self.cycle_write_u8(self.d),
                    Cycle::WriteE => self.cycle_write_u8(self.e),
                    Cycle::WriteF => self.cycle_write_u8(self.f),
                    Cycle::WriteH => self.cycle_write_u8(self.h),
                    Cycle::WriteHigh => self.cycle_write_u8(self.high_byte),
                    Cycle::WriteIOLow => {
                        self.data_bus = Some(self.low_byte);
                        self.write_pin = true;
                        self.io_pin = true;
                    }
                    Cycle::WriteIOU8(val) => {
                        self.data_bus = Some(val);
                        self.write_pin = true;
                        self.io_pin = true;
                    }
                    Cycle::WriteL => self.cycle_write_u8(self.l),
                    Cycle::WriteLow => self.cycle_write_u8(self.low_byte),
                    Cycle::WritePCHigh => self.cycle_write_u8((self.pc >> 8) as u8),
                    Cycle::WritePCLow => self.cycle_write_u8(self.pc as u8),
                    Cycle::WritePrefixedH => {
                        let val = self.get_prefixed_h();
                        self.cycle_write_u8(val);
                    }
                    Cycle::WritePrefixedL => {
                        let val = self.get_prefixed_l();
                        self.cycle_write_u8(val);
                    }
                    Cycle::WriteSPHigh => self.cycle_write_u8((self.sp >> 8) as u8),
                    Cycle::WriteSPLow => self.cycle_write_u8(self.sp as u8),
                }

                if self.write_pin {
                    self.do_write(io);
                }

                if self.microcodes.len() == 0 {
                    self.r = self.r.wrapping_add(1) & 0x7f;
                    self.ei = 0;
                    self.phase = FDEPhase::Init;
                    let prev_prefix = self.prefix;
                    self.exec_side_effect();
                    if prev_prefix == self.prefix {
                        self.q = if self.set_q { 1 } else { 0 };

                        if !self.is_ext {
                            if self.ei > 0 {
                                self.ei -= 1;
                                if self.ei == 0 {
                                    self.iff1 = 1;
                                    self.iff2 = 1;
                                }
                                return;
                            }

                            // Check IRQs
                            if self.irq_req && self.iff1 == 1 {
                                self.iff1 = 0;
                                self.iff2 = 0;
                                self.halt = false;
                                self.phase = FDEPhase::Execute;
                                self.is_ext = false;
                                self.microcodes = match self.im {
                                    0 => vec![
                                        Cycle::Init,
                                        Cycle::Nop,
                                        Cycle::Nop,
                                        Cycle::CheckIrqData,
                                        Cycle::CheckIrqData,
                                        Cycle::CheckIrqData,
                                    ],
                                    1 => vec![
                                        Cycle::Init,
                                        Cycle::Nop,
                                        Cycle::Nop,
                                        Cycle::CheckIrqData,
                                        Cycle::CheckIrqData,
                                        Cycle::CheckIrqData,
                                        Cycle::CheckIrqData,
                                        Cycle::PushStack,
                                        Cycle::WritePCHigh,
                                        Cycle::Unwrite,
                                        Cycle::PushStack,
                                        Cycle::WritePCLow,
                                        Cycle::Unwrite,
                                    ],
                                    2 => vec![
                                        Cycle::Init,
                                        Cycle::Nop,
                                        Cycle::Nop,
                                        Cycle::CheckIrqData,
                                        Cycle::CheckIrqData,
                                        Cycle::CheckIrqData,
                                        Cycle::CheckIrqData,
                                        Cycle::PushStack,
                                        Cycle::WritePCHigh,
                                        Cycle::Unwrite,
                                        Cycle::PushStack,
                                        Cycle::WritePCLow,
                                        Cycle::Unwrite,
                                        Cycle::AddrIsIM2Low,
                                        Cycle::ReadMem,
                                        Cycle::PeekLow,
                                        Cycle::AddrIsIM2High,
                                        Cycle::ReadMem,
                                        Cycle::PeekHigh,
                                    ],
                                    _ => panic!("Invalid im for irq"),
                                };
                                self.curr_op = OP_IM;
                            }
                        }
                    }
                }
            }
        }
    }

    fn exec_side_effect(&mut self) {
        let f: Option<fn (&mut Z80)> = match self.curr_op & 0xff00 {
            0x000|0x200|0x500 => match self.curr_op & 0x00ff {
                0x01 | 0xc1 => Some(set_bc),
                0x02 => Some(|cpu| {
                    cpu.wz = ((cpu.a as u16) << 8) | (cpu.c.wrapping_add(1) as u16);
                }),
                0x03 => Some(|cpu| inc_word(&mut cpu.b, &mut cpu.c)),
                0x04 | 0x0c | 0x14 | 0x1c | 0x24 | 0x2c | 0x3c => Some(|cpu| {
                    let val = cpu.inc_reg(cpu.prefixed_group_reg_r(cpu.group_8));
                    cpu.prefixed_group_reg_w(cpu.group_8, val);
                }),
                0x05 | 0x0d | 0x15 | 0x1d | 0x25 | 0x2d | 0x3d => Some(|cpu| {
                    let val = cpu.dec_reg(cpu.prefixed_group_reg_r(cpu.group_8));
                    cpu.prefixed_group_reg_w(cpu.group_8, val);
                }),
                0x06 | 0x0e | 0x16 | 0x1e | 0x26 | 0x2e | 0x3e => {
                    Some(|cpu| cpu.prefixed_group_reg_w(cpu.group_8, cpu.data_bus.unwrap()))
                }
                0x07 => Some(rlca),
                0x08 => Some(ex_af_af_),
                0x09 => Some(|cpu| {
                    cpu.wz = cpu.prefixed_hl().wrapping_add(1);
                    add_prefixed_hl_rp(cpu, cpu.bc());
                }),
                0x0a => Some(|cpu| {
                    cpu.a = cpu.data_bus.unwrap();
                    cpu.wz = cpu.bc().wrapping_add(1);
                }),
                0x0b => Some(|cpu| dec_word(&mut cpu.b, &mut cpu.c)),
                0x0f => Some(rrca),
                0x11 | 0xd1 => Some(set_de),
                0x12 => Some(|cpu| {
                    cpu.wz = ((cpu.a as u16) << 8) | (cpu.e.wrapping_add(1) as u16);
                }),
                0x13 => Some(|cpu| inc_word(&mut cpu.d, &mut cpu.e)),
                0x17 => Some(rla),
                0x19 => Some(|cpu| {
                    cpu.wz = cpu.prefixed_hl().wrapping_add(1);
                    add_prefixed_hl_rp(cpu, cpu.de());
                }),
                0x1a => Some(|cpu| {
                    cpu.a = cpu.data_bus.unwrap();
                    cpu.wz = cpu.de().wrapping_add(1);
                }),
                0x1b => Some(|cpu| dec_word(&mut cpu.d, &mut cpu.e)),
                0x1f => Some(rra),
                0x21 | 0x2a | 0xe1 => Some(set_prefixed_hl),
                0x22 => Some(|cpu| {
                    cpu.wz = word(cpu.high_byte, cpu.low_byte).wrapping_add(1);
                }),
                0x23 => Some(|cpu| match cpu.prefix {
                    Prefix::DD => cpu.ix = cpu.ix.wrapping_add(1),
                    Prefix::FD => cpu.iy = cpu.iy.wrapping_add(1),
                    Prefix::NONE => inc_word(&mut cpu.h, &mut cpu.l),
                }),
                0x27 => Some(daa),
                0x29 => Some(|cpu| {
                    let rp = cpu.prefixed_hl();
                    cpu.wz = rp.wrapping_add(1);
                    add_prefixed_hl_rp(cpu, rp);
                }),
                0x2b => Some(|cpu| match cpu.prefix {
                    Prefix::DD => cpu.ix = cpu.ix.wrapping_sub(1),
                    Prefix::FD => cpu.iy = cpu.iy.wrapping_sub(1),
                    Prefix::NONE => dec_word(&mut cpu.h, &mut cpu.l),
                }),
                0x2f => Some(cpl),
                0x31 => Some(|cpu| cpu.sp = word(cpu.high_byte, cpu.low_byte)),
                0x32 => Some(|cpu| {
                    cpu.wz = ((cpu.a as u16) << 8) | (cpu.low_byte.wrapping_add(1) as u16);
                }),
                0x33 => Some(|cpu| cpu.sp = cpu.sp.wrapping_add(1)),
                0x37 => Some(scf),
                0x39 => Some(|cpu| {
                    cpu.wz = cpu.prefixed_hl().wrapping_add(1);
                    add_prefixed_hl_rp(cpu, cpu.sp);
                }),
                0x3a => Some(|cpu| {
                    cpu.a = cpu.data_bus.unwrap();
                    cpu.wz = cpu.addr_bus.unwrap().wrapping_add(1);
                }),
                0x3b => Some(|cpu| cpu.sp = cpu.sp.wrapping_sub(1)),
                0x3f => Some(ccf),
                0x41..=0x47 => Some(|cpu| cpu.b = cpu.prefixed_group_reg_r(cpu.group_1)),
                0x48 | 0x4a..=0x4f => Some(|cpu| cpu.c = cpu.prefixed_group_reg_r(cpu.group_1)),
                0x50 | 0x51 | 0x53..=0x57 => {
                    Some(|cpu| cpu.d = cpu.prefixed_group_reg_r(cpu.group_1))
                }
                0x58..=0x5a | 0x5c..=0x5f => {
                    Some(|cpu| cpu.e = cpu.prefixed_group_reg_r(cpu.group_1))
                }
                0x60..=0x63 | 0x65 | 0x67 => {
                    Some(|cpu| cpu.set_prefixed_h(cpu.prefixed_group_reg_r(cpu.group_1)))
                }
                0x66 => Some(|cpu| cpu.h = cpu.prefixed_group_reg_r(cpu.group_1)),
                0x68..=0x6c | 0x6f => {
                    Some(|cpu| cpu.set_prefixed_l(cpu.prefixed_group_reg_r(cpu.group_1)))
                }
                0x6e => Some(|cpu| cpu.l = cpu.prefixed_group_reg_r(cpu.group_1)),
                0x76 => Some(|cpu| cpu.halt = true),
                0x78..=0x7d => Some(|cpu| cpu.a = cpu.prefixed_group_reg_r(cpu.group_1)),
                0x7e => Some(|cpu| cpu.a = cpu.data_bus.unwrap()),
                0x80..=0x87 => Some(|cpu| cpu.add_a_r(cpu.prefixed_group_reg_r(cpu.group_1))),
                0x88..=0x8f => Some(|cpu| cpu.adc_a_r(cpu.prefixed_group_reg_r(cpu.group_1))),
                0x90..=0x97 => Some(|cpu| cpu.sub_a_r(cpu.prefixed_group_reg_r(cpu.group_1))),
                0x98..=0x9f => Some(|cpu| cpu.sbc_a_r(cpu.prefixed_group_reg_r(cpu.group_1))),
                0xa0..=0xa7 => Some(|cpu| cpu.and_a_r(cpu.prefixed_group_reg_r(cpu.group_1))),
                0xa8..=0xaf => Some(|cpu| cpu.xor_a_r(cpu.prefixed_group_reg_r(cpu.group_1))),
                0xb0..=0xb7 => Some(|cpu| cpu.or_a_r(cpu.prefixed_group_reg_r(cpu.group_1))),
                0xb8..=0xbf => Some(|cpu| {
                    cpu.cp_a_r(cpu.prefixed_group_reg_r(cpu.group_1));
                }),
                0xc2 | 0xca | 0xd2 | 0xda | 0xe2 | 0xea | 0xf2 | 0xfa => Some(jp_cond),
                0xc3 | 0xc9 | 0xcd => Some(|cpu| {
                    cpu.pc = word(cpu.high_byte, cpu.low_byte);
                    cpu.wz = cpu.pc;
                }),
                0xc6 => Some(|cpu| cpu.add_a_r(cpu.low_byte)),
                0xc7 | 0xcf | 0xd7 | 0xdf | 0xe7 | 0xef | 0xf7 | 0xff => Some(rst),
                0xcb => Some(get_cb_op),
                0xce => Some(|cpu| cpu.adc_a_r(cpu.low_byte)),
                0xd3 => Some(|cpu| cpu.wz = word(cpu.a, cpu.low_byte.wrapping_add(1))),
                0xd6 => Some(|cpu| cpu.sub_a_r(cpu.low_byte)),
                0xd9 => Some(exx),
                0xdb => Some(|cpu| {
                    cpu.wz = word(cpu.a, cpu.low_byte).wrapping_add(1);
                    cpu.a = cpu.data_bus.unwrap()
                }),
                0xdd => Some(|cpu| {
                    cpu.phase = FDEPhase::Fetch;
                    cpu.prefix = Prefix::DD;
                    cpu.curr_op = 0x200;
                }),
                0xde => Some(|cpu| cpu.sbc_a_r(cpu.low_byte)),
                0xe3 => Some(prefixed_hl_is_word),
                0xe6 => Some(|cpu| cpu.and_a_r(cpu.low_byte)),
                0xe9 => Some(|cpu| cpu.pc = cpu.prefixed_hl()),
                0xeb => Some(ex_de_hl),
                0xed => Some(get_ed_op),
                0xee => Some(|cpu| cpu.xor_a_r(cpu.low_byte)),
                0xf1 => Some(set_af),
                0xf3 => Some(di),
                0xf6 => Some(|cpu| cpu.or_a_r(cpu.low_byte)),
                0xf9 => Some(|cpu| cpu.sp = cpu.prefixed_hl()),
                0xfb => Some(ei),
                0xfd => Some(|cpu| {
                    cpu.phase = FDEPhase::Fetch;
                    cpu.prefix = Prefix::FD;
                    cpu.curr_op = 0x500;
                }),
                0xfe => Some(|cpu| {
                    cpu.cp_a_r(cpu.low_byte);
                }),
                _ => None,
            },
            0x100|0x300|0x600 => match self.curr_op & 0x00ff {
                0x00..=0x05 | 0x07 => Some(|cpu| {
                    let val = match cpu.prefix {
                        Prefix::NONE => cpu.rlc(cpu.prefixed_group_reg_r(cpu.group_1)),
                        _ => cpu.low_byte,
                    };
                    cpu.group_reg_w(cpu.group_1, val);
                }),
                0x08..=0x0d | 0x0f => Some(|cpu| {
                    let val = match cpu.prefix {
                        Prefix::NONE => cpu.rrc(cpu.prefixed_group_reg_r(cpu.group_1)),
                        _ => cpu.low_byte,
                    };
                    cpu.group_reg_w(cpu.group_1, val);
                }),
                0x10..=0x15 | 0x17 => Some(|cpu| {
                    let val = match cpu.prefix {
                        Prefix::NONE => cpu.rl(cpu.prefixed_group_reg_r(cpu.group_1)),
                        _ => cpu.low_byte,
                    };
                    cpu.group_reg_w(cpu.group_1, val);
                }),
                0x18..=0x1d | 0x1f => Some(|cpu| {
                    let val = match cpu.prefix {
                        Prefix::NONE => cpu.rr(cpu.prefixed_group_reg_r(cpu.group_1)),
                        _ => cpu.low_byte,
                    };
                    cpu.group_reg_w(cpu.group_1, val);
                }),
                0x20..=0x25 | 0x27 => Some(|cpu| {
                    let val = match cpu.prefix {
                        Prefix::NONE => cpu.sla(cpu.prefixed_group_reg_r(cpu.group_1)),
                        _ => cpu.low_byte,
                    };
                    cpu.group_reg_w(cpu.group_1, val);
                }),
                0x28..=0x2d | 0x2f => Some(|cpu| {
                    let val = match cpu.prefix {
                        Prefix::NONE => cpu.sra(cpu.prefixed_group_reg_r(cpu.group_1)),
                        _ => cpu.low_byte,
                    };
                    cpu.group_reg_w(cpu.group_1, val);
                }),
                0x30..=0x35 | 0x37 => Some(|cpu| {
                    let val = match cpu.prefix {
                        Prefix::NONE => cpu.sll(cpu.prefixed_group_reg_r(cpu.group_1)),
                        _ => cpu.low_byte,
                    };
                    cpu.group_reg_w(cpu.group_1, val);
                }),
                0x38..=0x3d | 0x3f => Some(|cpu| {
                    let val = match cpu.prefix {
                        Prefix::NONE => cpu.srl(cpu.prefixed_group_reg_r(cpu.group_1)),
                        _ => cpu.low_byte,
                    };
                    cpu.group_reg_w(cpu.group_1, val);
                }),
                0x40..=0x7f => Some(bit),
                0x80..=0x85
                | 0x87..=0x8d
                | 0x8f..=0x95
                | 0x97..=0x9d
                | 0x9f..=0xa5
                | 0xa7..=0xad
                | 0xaf..=0xb5
                | 0xb7..=0xbd
                | 0xbf => Some(|cpu| {
                    let val = match cpu.prefix {
                        Prefix::NONE => cpu.prefixed_group_reg_r(cpu.group_1) & mask(cpu.group_8),
                        _ => cpu.low_byte,
                    };
                    cpu.group_reg_w(cpu.group_1, val);
                }),
                0xc0..=0xc5
                | 0xc7..=0xcd
                | 0xcf..=0xd5
                | 0xd7..=0xdd
                | 0xdf..=0xe5
                | 0xe7..=0xed
                | 0xef..=0xf5
                | 0xf7..=0xfd
                | 0xff => Some(|cpu| {
                    let val = match cpu.prefix {
                        Prefix::NONE => cpu.prefixed_group_reg_r(cpu.group_1) | flag(cpu.group_8),
                        _ => cpu.low_byte,
                    };
                    cpu.group_reg_w(cpu.group_1, val);
                }),
                _ => None,
            },
            0x400 => match self.curr_op & 0x00ff {
                0x40 | 0x48 | 0x50 | 0x58 | 0x60 | 0x68 | 0x70 | 0x78 => Some(in_reg_bc),
                0x41 | 0x49 | 0x51 | 0x59 | 0x61 | 0x69 | 0x79 => {
                    Some(|cpu| cpu.wz = cpu.bc().wrapping_add(1))
                }
                0x42 | 0x52 | 0x62 | 0x72 => Some(sbc_hl_rp),
                0x43 | 0x53 | 0x63 | 0x73 => {
                    Some(|cpu| cpu.wz = word(cpu.high_byte, cpu.low_byte).wrapping_add(1))
                }
                0x44 | 0x4c | 0x54 | 0x5c | 0x64 | 0x6c | 0x74 | 0x7c => Some(|cpu| {
                    let val = cpu.a;
                    cpu.a = 0;
                    cpu.sub_a_r(val);
                }),
                0x45 | 0x55 | 0x5d | 0x65 | 0x6d | 0x75 | 0x7d => Some(|cpu| {
                    cpu.pc = word(cpu.high_byte, cpu.low_byte);
                    cpu.wz = cpu.pc;
                    cpu.iff1 = cpu.iff2;
                }),
                0x46 | 0x4e | 0x66 | 0x6e => Some(|cpu| cpu.im = 0),
                0x47 => Some(|cpu| cpu.i = cpu.a),
                0x4a | 0x5a | 0x6a | 0x7a => Some(adc_hl_rp),
                0x4b => Some(set_bc),
                0x4d => Some(|cpu| {
                    cpu.pc = word(cpu.high_byte, cpu.low_byte);
                    cpu.wz = cpu.pc;
                    cpu.iff1 = cpu.iff2;
                    cpu.irq_req = false;
                }),
                0x4f => Some(|cpu| cpu.r = cpu.a),
                0x56 | 0x76 => Some(|cpu| cpu.im = 1),
                0x57 => Some(|cpu| cpu.ld_a_ir(cpu.i)),
                0x5b => Some(set_de),
                0x5e | 0x7e => Some(|cpu| cpu.im = 2),
                0x5f => Some(|cpu| cpu.ld_a_ir(cpu.r)),
                0x6b => Some(set_prefixed_hl),
                0x7b => Some(|cpu| cpu.sp = word(cpu.high_byte, cpu.low_byte)),
                0xa0 => Some(|cpu| cpu.ldid(true)),
                0xa1 => Some(|cpu| cpu.cpid(true)),
                0xa2 => Some(|cpu| cpu.inid(true)),
                0xa3 => Some(|cpu| cpu.outid(true)),
                0xa8 => Some(|cpu| cpu.ldid(false)),
                0xa9 => Some(|cpu| cpu.cpid(false)),
                0xaa => Some(|cpu| cpu.inid(false)),
                0xab => Some(|cpu| cpu.outid(false)),
                0xb0 => Some(|cpu| cpu.ldidr(true)),
                0xb1 => Some(|cpu| cpu.cpidr(true)),
                0xb2 => Some(|cpu| cpu.inidr(true)),
                0xb3 => Some(|cpu| cpu.otidr(true)),
                0xb8 => Some(|cpu| cpu.ldidr(false)),
                0xb9 => Some(|cpu| cpu.cpidr(false)),
                0xba => Some(|cpu| cpu.inidr(false)),
                0xbb => Some(|cpu| cpu.otidr(false)),
                _ => None,
            },
            0x700 => match self.curr_op {
                OP_CALL_COND => Some(|cpu| cpu.pc = cpu.wz),
                OP_CHECK_RET => Some(|cpu| {
                    cpu.pc = word(cpu.high_byte, cpu.low_byte);
                    cpu.wz = cpu.pc;
                }),
                OP_IM => match self.im {
                    0 => Some(|cpu| {
                        cpu.is_im0 = true;
                        cpu.phase = FDEPhase::Execute;
                    }),
                    1 => Some(|cpu| cpu.pc = 0x38),
                    2 => Some(|cpu| cpu.pc = word(cpu.high_byte, cpu.low_byte)),
                    _ => panic!("Invalid im for irq"),
                },
                _ => None,
            },
            _ => None,
        };
        match f {
            None => (),
            Some(func) => func(self),
        }
    }
}
