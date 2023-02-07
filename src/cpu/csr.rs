mod breakpoint;

use super::{CrossIsaUtil, PrivilegedLevel, TrapCause};
use crate::Isa;
use breakpoint::Triggers;
use std::rc::Rc;

pub struct CSRs {
    csrs: [u64; 4096],
    triggers: Triggers,
    isa: Rc<Isa>,
}

#[allow(clippy::identity_op)]
impl CSRs {
    pub fn new(isa: Rc<Isa>) -> Self {
        CSRs {
            csrs: [0; 4096],
            triggers: Triggers {
                tselect: 0,
                tdata1: [0; 8],
                tdata2: [0; 8],
            },
            isa,
        }
    }

    pub fn init(mut self) -> Self {
        match *self.isa {
            Isa::Rv32 => self.write(CSRname::misa.wrap(), 0x40141105),
            Isa::Rv64 => {
                self.write(CSRname::misa.wrap(), 0x8000000000141105);
                self.write(CSRname::mstatus.wrap(), 0x0000000a00000000);
            }
        }
        self
    }

    fn umask(&self) -> u64 {
        match *self.isa {
            Isa::Rv32 => 0b10000000000011010111100100110011,
            Isa::Rv64 => 0b100000000000000000000000000111100000000000011010111100100110011,
        }
    }

    fn smask(&self) -> u64 {
        match *self.isa {
            Isa::Rv32 => 0b10000000000011010111100100110011,
            Isa::Rv64 => 0b100000000000000000000000000111100000000000011010111100100110011,
        }
    }

    fn mmask(&self) -> u64 {
        match *self.isa {
            Isa::Rv32 => 0b10000000011111111111100110111011,
            Isa::Rv64 => 0b100000000000000000000000000111100000000011111111111100110111011,
        }
    }

    pub fn bitset(&mut self, dist: Option<usize>, src: u64) {
        let mask = src.fix2regsz(&self.isa);
        if mask != 0 {
            match dist.unwrap() {
                0x000 => self.csrs[0x300] |= mask & self.umask(),
                0x100 => self.csrs[0x300] |= mask & self.smask(),
                _ => self.csrs[dist.unwrap()] |= mask,
            }
        }
    }

    pub fn bitclr(&mut self, dist: Option<usize>, src: u64) {
        let mask = src.fix2regsz(&self.isa);
        if mask != 0 {
            match dist.unwrap() {
                0x000 => self.csrs[0x300] &= !(mask & self.umask()),
                0x100 => self.csrs[0x300] &= !(mask & self.smask()),
                _ => self.csrs[dist.unwrap()] &= !mask,
            }
        }
    }

    pub fn write(&mut self, dist: Option<usize>, src: u64) {
        let src = src.fix2regsz(&self.isa);
        match dist.unwrap() {
            0x000 => self.csrs[0x300] = src & self.umask(),
            0x100 => self.csrs[0x300] = src & self.smask(),
            other => self.csrs[other] = src,
        }
        self.update_triggers(dist.unwrap(), src);
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
            0x341 | 0x141 => self.read_xepc(dist),
            _ => Ok(self.csrs[dist].fix2regsz(&self.isa)),
        }
    }

    pub fn read_xstatus(&self, priv_lv: PrivilegedLevel, xfield: Xstatus) -> u64 {
        let xstatus = CSRname::mstatus as usize;
        let mask: u64 = match priv_lv {
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

    pub fn write_xstatus(&mut self, priv_lv: PrivilegedLevel, xfield: Xstatus, data: u64) {
        let data = data.fix2regsz(&self.isa);
        let xstatus = CSRname::mstatus as usize;
        let mask: u64 = match priv_lv {
            PrivilegedLevel::Machine => self.mmask(),
            PrivilegedLevel::Supervisor => self.smask(),
            PrivilegedLevel::User => self.umask(),
            _ => panic!("PrivilegedLevel 0x3 is Reserved."),
        };

        match xfield {
            Xstatus::UIE => {
                self.csrs[xstatus] =
                    ((self.csrs[xstatus] & !(0x1 << 0)) | ((data & 0x1) << 0)) & mask
            }
            Xstatus::SIE => {
                self.csrs[xstatus] =
                    ((self.csrs[xstatus] & !(0x1 << 1)) | ((data & 0x1) << 1)) & mask
            }
            Xstatus::MIE => {
                self.csrs[xstatus] =
                    ((self.csrs[xstatus] & !(0x1 << 3)) | ((data & 0x1) << 3)) & mask
            }
            Xstatus::UPIE => {
                self.csrs[xstatus] =
                    ((self.csrs[xstatus] & !(0x1 << 4)) | ((data & 0x1) << 4)) & mask
            }
            Xstatus::SPIE => {
                self.csrs[xstatus] =
                    ((self.csrs[xstatus] & !(0x1 << 5)) | ((data & 0x1) << 5)) & mask
            }
            Xstatus::MPIE => {
                self.csrs[xstatus] =
                    ((self.csrs[xstatus] & !(0x1 << 7)) | ((data & 0x1) << 7)) & mask
            }
            Xstatus::SPP => {
                self.csrs[xstatus] =
                    ((self.csrs[xstatus] & !(0x1 << 8)) | ((data & 0x1) << 8)) & mask
            }
            Xstatus::MPP => {
                self.csrs[xstatus] =
                    ((self.csrs[xstatus] & !(0x3 << 11)) | ((data & 0x3) << 11)) & mask
            }
            Xstatus::FS => {
                self.csrs[xstatus] =
                    ((self.csrs[xstatus] & !(0x3 << 13)) | ((data & 0x3) << 13)) & mask
            }
            Xstatus::XS => {
                self.csrs[xstatus] =
                    ((self.csrs[xstatus] & !(0x3 << 15)) | ((data & 0x3) << 15)) & mask
            }
            Xstatus::MPRV => {
                self.csrs[xstatus] =
                    ((self.csrs[xstatus] & !(0x1 << 17)) | ((data & 0x1) << 17)) & mask
            }
            Xstatus::SUM => {
                self.csrs[xstatus] =
                    ((self.csrs[xstatus] & !(0x1 << 18)) | ((data & 0x1) << 18)) & mask
            }
            Xstatus::MXR => {
                self.csrs[xstatus] =
                    ((self.csrs[xstatus] & !(0x1 << 19)) | ((data & 0x1) << 19)) & mask
            }
            Xstatus::TVM => {
                self.csrs[xstatus] =
                    ((self.csrs[xstatus] & !(0x1 << 20)) | ((data & 0x1) << 20)) & mask
            }
            Xstatus::TW => {
                self.csrs[xstatus] =
                    ((self.csrs[xstatus] & !(0x1 << 21)) | ((data & 0x1) << 21)) & mask
            }
            Xstatus::TSR => {
                self.csrs[xstatus] =
                    ((self.csrs[xstatus] & !(0x1 << 22)) | ((data & 0x1) << 22)) & mask
            }
            Xstatus::SD => {
                self.csrs[xstatus] = match *self.isa {
                    Isa::Rv32 => {
                        ((self.csrs[xstatus] & !(0x1 << 31)) | ((data & 0x1) << 31)) & mask
                    }
                    Isa::Rv64 => {
                        ((self.csrs[xstatus] & !(0x1 << 63)) | ((data & 0x1) << 63)) & mask
                    }
                }
            }
            Xstatus::UXL => {
                self.csrs[xstatus] = match *self.isa {
                    Isa::Rv32 => panic!("attempting write to UXL in rv32"),
                    Isa::Rv64 => {
                        ((self.csrs[xstatus] & !(0x3 << 32)) | ((data & 0x3) << 32)) & mask
                    }
                }
            }
            Xstatus::SXL => {
                self.csrs[xstatus] = match *self.isa {
                    Isa::Rv32 => panic!("attempting write to SXL in rv32"),
                    Isa::Rv64 => {
                        ((self.csrs[xstatus] & !(0x3 << 34)) | ((data & 0x3) << 34)) & mask
                    }
                }
            }
        }
    }
}

impl Default for CSRs {
    fn default() -> Self {
        Self::new(Isa::Rv64.into())
    }
}

#[allow(non_camel_case_types)]
pub enum CSRname {
    ustatus = 0x000,
    utvec = 0x005,
    uepc = 0x041,
    ucause = 0x042,
    sstatus = 0x100,
    stvec = 0x105,
    sscratch = 0x140,
    sepc = 0x141,
    scause = 0x142,
    stval = 0x143,
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
