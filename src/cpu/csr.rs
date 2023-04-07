mod breakpoint;

use super::{CrossIsaUtil, PrivilegedLevel, TrapCause};
use crate::Isa;
use breakpoint::Triggers;
use std::cell::RefCell;
use std::rc::Rc;

const MISA: usize = CSRname::misa as usize;
const USTATUS: usize = CSRname::ustatus as usize;
const SSTATUS: usize = CSRname::sstatus as usize;
const MSTATUS: usize = CSRname::mstatus as usize;
const SIE: usize = CSRname::sie as usize;
const SIP: usize = CSRname::sip as usize;
const SIESIPMASK: u64 = 0x0333;
const MHPMCOUNTER3: usize = CSRname::mhpmcounter3 as usize;

pub struct CSRs {
    csrs: [u64; 4096],
    triggers: Triggers,
    pc: Rc<RefCell<u64>>,
    isa: Rc<Isa>,
    priv_lv: Rc<RefCell<PrivilegedLevel>>,
}

#[allow(clippy::identity_op)]
impl CSRs {
    pub fn new(isa: Rc<Isa>, pc: Rc<RefCell<u64>>, priv_lv: Rc<RefCell<PrivilegedLevel>>) -> Self {
        CSRs {
            csrs: [0; 4096],
            triggers: Triggers {
                tselect: 0,
                tdata1: [0; 8],
                tdata2: [0; 8],
            },
            pc,
            isa,
            priv_lv,
        }
    }

    fn priv_lv(&self) -> PrivilegedLevel {
        *self.priv_lv.borrow()
    }

    pub fn init(mut self) -> Self {
        self.write(CSRname::marchid.wrap(), 0x5).unwrap();
        match *self.isa {
            Isa::Rv32 => self.write(CSRname::misa.wrap(), 0x40141105).unwrap(),
            Isa::Rv64 => {
                self.write(CSRname::misa.wrap(), 0x8000000000141105)
                    .unwrap();
                self.write(CSRname::mstatus.wrap(), 0x0000000a00000000)
                    .unwrap();
            }
        }
        self
    }

    fn umask(&self) -> u64 {
        match *self.isa {
            Isa::Rv32 => 0b10000000000011010111100100110011,
            Isa::Rv64 => 0b100000000000000000000000000001100000000000011010111100100110011,
        }
    }

    fn smask(&self) -> u64 {
        match *self.isa {
            Isa::Rv32 => 0b10000000000011010111100100110011,
            Isa::Rv64 => 0b100000000000000000000000000001100000000000011010111100100110011,
        }
    }

    fn mmask(&self) -> u64 {
        match *self.isa {
            Isa::Rv32 => 0b10000000011111111111100110111011,
            Isa::Rv64 => 0b100000000000000000000000000111100000000011111111111100110111011,
        }
    }

    fn mask_warl(&mut self, dst: usize, mask: u64) -> u64 {
        match dst {
            MISA => {
                if *self.pc.borrow() % 4 != 0 {
                    mask & !0b100 // clear C extension flag
                } else {
                    mask
                }
            }
            MSTATUS => match *self.isa {
                Isa::Rv32 => mask,
                Isa::Rv64 => mask & !(0b1111 << 32),
            },
            MHPMCOUNTER3 => 0,
            _ => mask,
        }
    }

    fn check_accessible(&self, dist: usize) -> Result<(), (Option<u64>, TrapCause, String)> {
        if dist >= 4096 {
            return Err((
                None,
                TrapCause::IllegalInst,
                format!("csr size is 4096, but you accessed {dist}"),
            ));
        }

        match self.priv_lv() {
            PrivilegedLevel::User => {
                if (0x100..=0x180).contains(&dist) || (0x300..=0x344).contains(&dist) {
                    return Err((
                        None,
                        TrapCause::IllegalInst,
                        format!("You are in User mode but accessed {dist}"),
                    ));
                }
            }
            PrivilegedLevel::Supervisor => {
                if (0x300..=0x344).contains(&dist) {
                    return Err((
                        None,
                        TrapCause::IllegalInst,
                        format!("You are in Supervisor mode but accessed {dist}"),
                    ));
                }

                if dist == CSRname::satp as usize && self.read_xstatus(Xstatus::TVM) == 1 {
                    return Err((
                        None,
                        TrapCause::IllegalInst,
                        "mstatus.TVM == 1 but accessed satp".to_string(),
                    ));
                }
            }
            _ => (),
        }

        if (0xc00..=0xc1f).contains(&dist) {
            match self.priv_lv() {
                PrivilegedLevel::Machine | PrivilegedLevel::Reserved => (),
                PrivilegedLevel::Supervisor => {
                    let mctren = self.read(CSRname::mcounteren.wrap())?;
                    if mctren >> (dist - 0xc00) & 0x1 == 0 {
                        return Err((
                            None,
                            TrapCause::IllegalInst,
                            "mcounteren bit is cleared, but attempt reading".to_string(),
                        ));
                    }
                }
                PrivilegedLevel::User => {
                    let mctren = self.read(CSRname::mcounteren.wrap())?;
                    if mctren >> (dist - 0xc00) & 0x1 == 0 {
                        return Err((
                            None,
                            TrapCause::IllegalInst,
                            "mcounteren bit is cleared, but attempt reading".to_string(),
                        ));
                    }
                    let sctren = self.read(CSRname::scounteren.wrap())?;
                    if sctren >> (dist - 0xc00) & 0x1 == 0 {
                        return Err((
                            None,
                            TrapCause::IllegalInst,
                            "mcounteren bit is cleared, but attempt reading".to_string(),
                        ));
                    }
                }
            }
        }

        // riscv-privileged-20190608.pdf p7
        let csrs_ranges = vec![
            0x000..=0x0ff,
            0x400..=0x4ff,
            0x800..=0x8ff,
            0xc00..=0xc7f,
            0xc80..=0xcbf,
            0xcc0..=0xcff,
            0x100..=0x1ff,
            0x500..=0x57f,
            0x580..=0x5bf,
            0x5c0..=0x5ff,
            0x900..=0x97f,
            0x980..=0x9bf,
            0x9c0..=0x9ff,
            0xd00..=0xd7f,
            //0xd80..=0xdbf,
            0xdc0..=0xdff,
            0x200..=0x2ff,
            0x600..=0x67f,
            0x680..=0x6bf,
            0x6c0..=0x6ff,
            0xa00..=0xa7f,
            0xa80..=0xabf,
            0xac0..=0xaff,
            0xe00..=0xe7f,
            0xe80..=0xebf,
            0xec0..=0xeff,
            //0x300..=0x3ff,
            0x300..=0x30b, // disable mstateen0(0x30c)
            0x30d..=0x3bf, // disable pmpaddr16~
            0x700..=0x77f,
            0x780..=0x79f,
            0x7a0..=0x7af,
            0x7b0..=0x7bf,
            0x7c0..=0x7ff,
            0xb00..=0xb7f,
            0xb80..=0xbbf,
            0xbc0..=0xbff,
            0xf00..=0xf7f,
            //0xf80..=0xfbf,
            0xfc0..=0xfff,
        ];

        if csrs_ranges.iter().any(|x| x.contains(&dist)) {
            match dist {
                // == depends on privilege ==
                // scounteren(0x106) only allow higher
                0x106 => match self.priv_lv() {
                    PrivilegedLevel::User => Err((
                        None,
                        TrapCause::IllegalInst,
                        format!("unknown CSR number: {dist}"),
                    )),
                    _ => Ok(()),
                },
                // stimecmp(0x14d) only supervisor
                0x14d => match self.priv_lv() {
                    PrivilegedLevel::Supervisor => Ok(()),
                    _ => Err((
                        None,
                        TrapCause::IllegalInst,
                        format!("unknown CSR number: {dist}"),
                    )),
                },
                _ => Ok(()),
            }
        } else {
            // out of range
            Err((
                None,
                TrapCause::IllegalInst,
                format!("unknown CSR number: {dist}"),
            ))
        }
    }

    pub fn bitset(
        &mut self,
        dist: Option<usize>,
        src: u64,
    ) -> Result<(), (Option<u64>, TrapCause, String)> {
        let dist = dist.unwrap();
        self.check_accessible(dist)?;

        let mask = self.mask_warl(dist, src.fix2regsz(&self.isa));
        if mask != 0 {
            match dist {
                USTATUS => self.csrs[MSTATUS] |= mask & self.umask(),
                SSTATUS => self.csrs[MSTATUS] |= mask & self.smask(),
                SIE => self.csrs[CSRname::mie as usize] |= mask & SIESIPMASK,
                SIP => self.csrs[CSRname::mip as usize] |= mask & SIESIPMASK,
                _ => self.csrs[dist] |= mask,
            }
        }

        Ok(())
    }

    pub fn bitclr(
        &mut self,
        dist: Option<usize>,
        src: u64,
    ) -> Result<(), (Option<u64>, TrapCause, String)> {
        let dist = dist.unwrap();
        self.check_accessible(dist)?;

        let mask = self.mask_warl(dist, src.fix2regsz(&self.isa));
        if mask != 0 {
            match dist {
                USTATUS => self.csrs[MSTATUS] &= !(mask & self.umask()),
                SSTATUS => self.csrs[MSTATUS] &= !(mask & self.smask()),
                SIE => self.csrs[CSRname::mie as usize] &= !(mask & SIESIPMASK),
                SIP => self.csrs[CSRname::mip as usize] &= !(mask & SIESIPMASK),
                _ => self.csrs[dist] &= !mask,
            }
        }

        Ok(())
    }

    pub fn write(
        &mut self,
        dist: Option<usize>,
        src: u64,
    ) -> Result<(), (Option<u64>, TrapCause, String)> {
        let dist = dist.unwrap();
        self.check_accessible(dist)?;

        let src = src.fix2regsz(&self.isa);
        match dist {
            USTATUS => self.csrs[MSTATUS] |= src & self.umask(),
            SSTATUS => self.csrs[MSTATUS] |= src & self.smask(),
            SIE => self.csrs[CSRname::mie as usize] = src & SIESIPMASK,
            SIP => self.csrs[CSRname::mip as usize] = src & SIESIPMASK,
            MISA => {
                if *self.pc.borrow() % 4 != 0 {
                    let c_ext_bit = (self.csrs[MISA] >> 2) & 1;
                    self.csrs[MISA] = (src & !0b100) | c_ext_bit
                } else {
                    self.csrs[MISA] = src
                }
            }
            MSTATUS => match *self.isa {
                Isa::Rv32 => self.csrs[dist] = src,
                Isa::Rv64 => self.csrs[dist] = (src & !(0b1111 << 32)) | 0b1010 << 32,
            },
            MHPMCOUNTER3 => (), // protect from any value
            other => self.csrs[other] = src,
        }
        self.update_triggers(dist, src);

        Ok(())
    }

    fn read_xepc(&self, dist: usize) -> Result<u64, (Option<u64>, TrapCause, String)> {
        if self.csrs[CSRname::misa as usize] >> 2 & 0x1 == 1 {
            // C extension enabled (IALIGN = 16)
            Ok(self.csrs[dist].fix2regsz(&self.isa) & !0b01)
        } else {
            // C extension disabled (IALIGN = 32)
            Ok(self.csrs[dist].fix2regsz(&self.isa) & !0b11)
        }
    }

    pub fn read(&self, src: Option<usize>) -> Result<u64, (Option<u64>, TrapCause, String)> {
        let dist = src.unwrap();

        match dist {
            0x000 => Ok(self.csrs[0x300].fix2regsz(&self.isa) & self.umask()),
            0x100 => Ok(self.csrs[0x300].fix2regsz(&self.isa) & self.smask()),
            SIE => Ok(self.csrs[CSRname::mie as usize].fix2regsz(&self.isa) & SIESIPMASK),
            SIP => Ok(self.csrs[CSRname::mip as usize].fix2regsz(&self.isa) & SIESIPMASK),
            0x341 | 0x141 => self.read_xepc(dist),
            _ => Ok(self.csrs[dist].fix2regsz(&self.isa)),
        }
    }

    pub fn read_xstatus(&self, xfield: Xstatus) -> u64 {
        let xstatus = CSRname::mstatus as usize;
        let mask: u64 = match self.priv_lv() {
            PrivilegedLevel::Machine => self.mmask(),
            PrivilegedLevel::Supervisor => self.smask(),
            PrivilegedLevel::User => self.umask(),
            _ => panic!("PrivilegedLevel 0x3 is Reserved."),
        };

        match xfield {
            Xstatus::UIE => (self.csrs[xstatus] & mask) >> 0 & 0x1,
            Xstatus::SIE => (self.csrs[xstatus] & mask) >> 1 & 0x1,
            Xstatus::MIE => (self.csrs[xstatus] & mask) >> 3 & 0x1,
            Xstatus::UPIE => (self.csrs[xstatus] & mask) >> 4 & 0x1,
            Xstatus::SPIE => (self.csrs[xstatus] & mask) >> 5 & 0x1,
            Xstatus::MPIE => (self.csrs[xstatus] & mask) >> 7 & 0x1,
            Xstatus::SPP => (self.csrs[xstatus] & mask) >> 8 & 0x1,
            Xstatus::MPP => (self.csrs[xstatus] & mask) >> 11 & 0x3,
            Xstatus::FS => (self.csrs[xstatus] & mask) >> 13 & 0x3,
            Xstatus::XS => (self.csrs[xstatus] & mask) >> 15 & 0x3,
            Xstatus::MPRV => (self.csrs[xstatus] & mask) >> 17 & 0x1,
            Xstatus::SUM => (self.csrs[xstatus] & mask) >> 18 & 0x1,
            Xstatus::MXR => (self.csrs[xstatus] & mask) >> 19 & 0x1,
            Xstatus::TVM => (self.csrs[xstatus] & mask) >> 20 & 0x1,
            Xstatus::TW => (self.csrs[xstatus] & mask) >> 21 & 0x1,
            Xstatus::TSR => (self.csrs[xstatus] & mask) >> 22 & 0x1,
            Xstatus::UXL => match *self.isa {
                Isa::Rv32 => panic!("attempting read to UXL in rv32"),
                Isa::Rv64 => (self.csrs[xstatus] & mask) >> 32 & 0x3,
            },
            Xstatus::SXL => match *self.isa {
                Isa::Rv32 => panic!("attempting read to SXL in rv32"),
                Isa::Rv64 => (self.csrs[xstatus] & mask) >> 34 & 0x3,
            },
            Xstatus::SD => match *self.isa {
                Isa::Rv32 => (self.csrs[xstatus] & mask) >> 31 & 0x1,
                Isa::Rv64 => (self.csrs[xstatus] & mask) >> 63 & 0x1,
            },
        }
        .fix2regsz(&self.isa)
    }

    pub fn write_xstatus(&mut self, xfield: Xstatus, data: u64) {
        let data = data.fix2regsz(&self.isa);
        let xstatus = CSRname::mstatus as usize;
        let mask: u64 = match self.priv_lv() {
            PrivilegedLevel::Machine => self.mmask(),
            PrivilegedLevel::Supervisor => self.smask(),
            PrivilegedLevel::User => self.umask(),
            _ => panic!("PrivilegedLevel 0x3 is Reserved."),
        };

        match xfield {
            Xstatus::UIE => {
                self.csrs[xstatus] =
                    (self.csrs[xstatus] & !(0x1 << 0)) | (((data & 0x1) << 0) & mask)
            }
            Xstatus::SIE => {
                self.csrs[xstatus] =
                    (self.csrs[xstatus] & !(0x1 << 1)) | (((data & 0x1) << 1) & mask)
            }
            Xstatus::MIE => {
                self.csrs[xstatus] =
                    (self.csrs[xstatus] & !(0x1 << 3)) | (((data & 0x1) << 3) & mask)
            }
            Xstatus::UPIE => {
                self.csrs[xstatus] =
                    (self.csrs[xstatus] & !(0x1 << 4)) | (((data & 0x1) << 4) & mask)
            }
            Xstatus::SPIE => {
                self.csrs[xstatus] =
                    (self.csrs[xstatus] & !(0x1 << 5)) | (((data & 0x1) << 5) & mask)
            }
            Xstatus::MPIE => {
                self.csrs[xstatus] =
                    (self.csrs[xstatus] & !(0x1 << 7)) | (((data & 0x1) << 7) & mask)
            }
            Xstatus::SPP => {
                self.csrs[xstatus] =
                    (self.csrs[xstatus] & !(0x1 << 8)) | (((data & 0x1) << 8) & mask)
            }
            Xstatus::MPP => {
                self.csrs[xstatus] =
                    (self.csrs[xstatus] & !(0x3 << 11)) | (((data & 0x3) << 11) & mask)
            }
            Xstatus::FS => {
                self.csrs[xstatus] =
                    (self.csrs[xstatus] & !(0x3 << 13)) | (((data & 0x3) << 13) & mask)
            }
            Xstatus::XS => {
                self.csrs[xstatus] =
                    (self.csrs[xstatus] & !(0x3 << 15)) | (((data & 0x3) << 15) & mask)
            }
            Xstatus::MPRV => {
                self.csrs[xstatus] =
                    (self.csrs[xstatus] & !(0x1 << 17)) | (((data & 0x1) << 17) & mask)
            }
            Xstatus::SUM => {
                self.csrs[xstatus] =
                    (self.csrs[xstatus] & !(0x1 << 18)) | (((data & 0x1) << 18) & mask)
            }
            Xstatus::MXR => {
                self.csrs[xstatus] =
                    (self.csrs[xstatus] & !(0x1 << 19)) | (((data & 0x1) << 19) & mask)
            }
            Xstatus::TVM => {
                self.csrs[xstatus] =
                    (self.csrs[xstatus] & !(0x1 << 20)) | (((data & 0x1) << 20) & mask)
            }
            Xstatus::TW => {
                self.csrs[xstatus] =
                    (self.csrs[xstatus] & !(0x1 << 21)) | (((data & 0x1) << 21) & mask)
            }
            Xstatus::TSR => {
                self.csrs[xstatus] =
                    (self.csrs[xstatus] & !(0x1 << 22)) | (((data & 0x1) << 22) & mask)
            }
            Xstatus::SD => {
                self.csrs[xstatus] = match *self.isa {
                    Isa::Rv32 => {
                        (self.csrs[xstatus] & !(0x1 << 31)) | (((data & 0x1) << 31) & mask)
                    }
                    Isa::Rv64 => {
                        (self.csrs[xstatus] & !(0x1 << 63)) | (((data & 0x1) << 63) & mask)
                    }
                }
            }
            Xstatus::UXL => {
                self.csrs[xstatus] = match *self.isa {
                    Isa::Rv32 => panic!("attempting write to UXL in rv32"),
                    Isa::Rv64 => {
                        (self.csrs[xstatus] & !(0x3 << 32)) | (((data & 0x3) << 32) & mask)
                    }
                }
            }
            Xstatus::SXL => {
                self.csrs[xstatus] = match *self.isa {
                    Isa::Rv32 => panic!("attempting write to SXL in rv32"),
                    Isa::Rv64 => {
                        (self.csrs[xstatus] & !(0x3 << 34)) | (((data & 0x3) << 34) & mask)
                    }
                }
            }
        }
    }

    pub fn timer_increment(&mut self, inc: u64) {
        self.csrs[CSRname::timer as usize] += inc;
    }
}

impl Default for CSRs {
    fn default() -> Self {
        Self::new(
            Isa::Rv64.into(),
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(PrivilegedLevel::Machine)),
        )
    }
}

#[allow(non_camel_case_types)]
pub enum CSRname {
    ustatus = 0x000,
    utvec = 0x005,
    uepc = 0x041,
    ucause = 0x042,
    sstatus = 0x100,
    sie = 0x104,
    stvec = 0x105,
    scounteren = 0x106,
    sscratch = 0x140,
    sepc = 0x141,
    scause = 0x142,
    stval = 0x143,
    sip = 0x144,
    satp = 0x180,
    mstatus = 0x300,
    misa = 0x301,
    medeleg = 0x302,
    mideleg = 0x303,
    mie = 0x304,
    mtvec = 0x305,
    mcounteren = 0x306,
    mscratch = 0x340,
    mepc = 0x341,
    mcause = 0x342,
    mtval = 0x343,
    mip = 0x344,
    tselect = 0x7a0,
    tdata1 = 0x7a1,
    tdata2 = 0x7a2,
    tdata3 = 0x7a3,
    tdata4 = 0x7a4,
    tdata5 = 0x7a5,
    mhpmcounter3 = 0xb03,
    timer = 0xc01,
    marchid = 0xf12,
}

impl CSRname {
    pub fn wrap(self) -> Option<usize> {
        Some(self as usize)
    }
}

pub enum Xstatus {
    UIE,  // 0
    SIE,  // 1
    MIE,  // 3
    UPIE, // 4
    SPIE, // 5
    MPIE, // 7
    SPP,  // 8
    MPP,  // 11-12
    FS,   // 13-14
    XS,   // 15-16
    MPRV, // 17
    SUM,  // 18
    MXR,  // 19
    TVM,  // 20
    TW,   // 21
    TSR,  // 22
    UXL,  // 32-33 (rv64 only)
    SXL,  // 33-34 (rv64 only)
    SD,   // XLEN - 1
}
