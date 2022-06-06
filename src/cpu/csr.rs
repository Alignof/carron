use crate::cpu::PrivilegedLevel;

pub struct CSRs {
    csrs: [u32; 4096],
}

impl CSRs {
    pub fn new() -> CSRs {
        CSRs {
            csrs: [0; 4096],
        }
    }

    pub fn bitset(&mut self, dist: Option<usize>, src: i32) {
        let mask = src as u32;
        if mask != 0 {
            self.csrs[dist.unwrap()] |= mask;
        }
    }

    pub fn bitclr(&mut self, dist: Option<usize>, src: i32) {
        let mask = src as u32;
        if mask != 0 {
            self.csrs[dist.unwrap()] &= !mask;
        }
    }

    pub fn write(&mut self, dist: Option<usize>, src: i32) {
        self.csrs[dist.unwrap()] = src as u32;
    }

    pub fn read(&self, src: Option<usize>) -> u32 {
        self.csrs[src.unwrap()]
    }

    pub fn read_xstatus(&self, priv_lv: &PrivilegedLevel, xfield: Xstatus) -> u32 {
        let xstatus: usize = match priv_lv {
            PrivilegedLevel::Machine => CSRname::mstatus as usize,
            PrivilegedLevel::Supervisor => CSRname::sstatus as usize,
            PrivilegedLevel::User => CSRname::ustatus as usize,
            _ => panic!("PrivilegedLevel 0x3 is Reserved."),
        };

        match xfield {
            Xstatus::UIE    => self.csrs[xstatus] >>  0 & 0x1,
            Xstatus::SIE    => self.csrs[xstatus] >>  1 & 0x1,
            Xstatus::MIE    => self.csrs[xstatus] >>  3 & 0x1,
            Xstatus::UPIE   => self.csrs[xstatus] >>  4 & 0x1,
            Xstatus::SPIE   => self.csrs[xstatus] >>  5 & 0x1,
            Xstatus::MPIE   => self.csrs[xstatus] >>  7 & 0x1,
            Xstatus::SPP    => self.csrs[xstatus] >>  8 & 0x1,
            Xstatus::MPP    => self.csrs[xstatus] >> 11 & 0x3,
            Xstatus::FS     => self.csrs[xstatus] >> 13 & 0x3,
            Xstatus::XS     => self.csrs[xstatus] >> 15 & 0x3,
            Xstatus::MPRV   => self.csrs[xstatus] >> 17 & 0x1,
            Xstatus::SUM    => self.csrs[xstatus] >> 18 & 0x1,
            Xstatus::MXR    => self.csrs[xstatus] >> 19 & 0x1,
            Xstatus::TVM    => self.csrs[xstatus] >> 20 & 0x1,
            Xstatus::TW     => self.csrs[xstatus] >> 21 & 0x1,
            Xstatus::TSR    => self.csrs[xstatus] >> 22 & 0x1,
            Xstatus::SD     => self.csrs[xstatus] >> 31 & 0x1,
        }
    } 
}

#[allow(non_camel_case_types)]
pub enum CSRname {
    ustatus  = 0x000,
    utvec    = 0x005,
    uepc     = 0x041,
    ucause   = 0x042,
    sstatus  = 0x100,
    stvec    = 0x105,
    sscratch = 0x140, 
    sepc     = 0x141, 
    scause   = 0x142,
    stval    = 0x143,
    satp     = 0x180,
    mstatus  = 0x300,
    medeleg  = 0x302,
    mtvec    = 0x305,
    mscratch = 0x340, 
    mepc     = 0x341, 
    mcause   = 0x342,
    mtval    = 0x343,
}

pub enum Xstatus {
    UIE,	// 0
    SIE,	// 1
    MIE,	// 3
    UPIE,	// 4
    SPIE,	// 5
    MPIE,	// 7
    SPP,	// 8
    MPP,	// 11-12
    FS,		// 13-14
    XS,		// 15-16
    MPRV,	// 17
    SUM,	// 18
    MXR,	// 19
    TVM,	// 20
    TW,		// 21
    TSR,	// 22
    SD,		// 31
}

impl CSRname {
    pub fn wrap(self) -> Option<usize> {
        Some(self as usize)
    }
}

