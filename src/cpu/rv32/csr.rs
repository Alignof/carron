mod breakpoint;

use super::{CSRname, PrivilegedLevel, TrapCause, Xstatus};
use breakpoint::Triggers;

const UMASK: u32 = 0b10000000000011010111100100110011;
const SMASK: u32 = 0b10000000000011010111100100110011;
const MMASK: u32 = 0b10000000011111111111100110111011;

pub struct CSRs {
    csrs: [u32; 4096],
    triggers: Triggers,
}

#[allow(clippy::identity_op)]
impl CSRs {
    pub fn new() -> Self {
        CSRs {
            csrs: [0; 4096],
            triggers: Triggers {
                tselect: 0,
                tdata1: [0; 8],
                tdata2: [0; 8],
            },
        }
    }

    pub fn init(mut self) -> Self {
        self.write(CSRname::misa.wrap(), 0x40141105);
        self
    }

    pub fn bitset(&mut self, dist: Option<usize>, src: u32) {
        let mask = src;
        if mask != 0 {
            match dist.unwrap() {
                0x000 => self.csrs[0x300] |= mask & UMASK,
                0x100 => self.csrs[0x300] |= mask & SMASK,
                _ => self.csrs[dist.unwrap()] |= mask,
            }
        }
    }

    pub fn bitclr(&mut self, dist: Option<usize>, src: u32) {
        let mask = src;
        if mask != 0 {
            match dist.unwrap() {
                0x000 => self.csrs[0x300] &= !(mask & UMASK),
                0x100 => self.csrs[0x300] &= !(mask & SMASK),
                _ => self.csrs[dist.unwrap()] &= !mask,
            }
        }
    }

    pub fn write(&mut self, dist: Option<usize>, src: u32) {
        match dist.unwrap() {
            0x000 => self.csrs[0x300] = src & UMASK,
            0x100 => self.csrs[0x300] = src & SMASK,
            _ => self.csrs[dist.unwrap()] = src,
        }
        self.update_triggers(dist.unwrap(), src);
    }

    fn read_xepc(&self, dist: usize) -> Result<u32, (Option<u32>, TrapCause, String)> {
        if self.csrs[CSRname::misa as usize] >> 2 & 0x1 == 1 {
            // C extension enabled (IALIGN = 16)
            Ok(self.csrs[dist] & !0b01)
        } else {
            // C extension disabled (IALIGN = 32)
            Ok(self.csrs[dist] & !0b11)
        }
    }

    pub fn read(&self, src: Option<usize>) -> Result<u32, (Option<u32>, TrapCause, String)> {
        let dist = src.unwrap();
        match dist {
            0x000 => Ok(self.csrs[0x300] & UMASK),
            0x100 => Ok(self.csrs[0x300] & SMASK),
            0x341 | 0x141 => self.read_xepc(dist),
            _ => Ok(self.csrs[dist]),
        }
    }

    pub fn read_xstatus(&self, priv_lv: PrivilegedLevel, xfield: Xstatus) -> u32 {
        let xstatus = CSRname::mstatus as usize;
        let mask: u32 = match priv_lv {
            PrivilegedLevel::Machine => MMASK,
            PrivilegedLevel::Supervisor => SMASK,
            PrivilegedLevel::User => UMASK,
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
            Xstatus::SD => (self.csrs[xstatus] & mask) >> 31 & 0x1,
        }
    }

    pub fn write_xstatus(&mut self, priv_lv: PrivilegedLevel, xfield: Xstatus, data: u32) {
        let xstatus = CSRname::mstatus as usize;
        let mask: u32 = match priv_lv {
            PrivilegedLevel::Machine => MMASK,
            PrivilegedLevel::Supervisor => SMASK,
            PrivilegedLevel::User => UMASK,
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
                self.csrs[xstatus] =
                    ((self.csrs[xstatus] & !(0x1 << 31)) | ((data & 0x1) << 31)) & mask
            }
        }
    }
}

impl Default for CSRs {
    fn default() -> Self {
        Self::new()
    }
}