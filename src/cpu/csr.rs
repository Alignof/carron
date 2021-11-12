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

    pub fn read_mstatus(&self, mstat: Mstatus) -> u32 {
        let mstatus: usize = CSRname::mstatus as usize;
        match mstat {
            Mstatus::UIE    => self.csrs[mstatus] >>  0 & 0x1,
            Mstatus::SIE    => self.csrs[mstatus] >>  1 & 0x1,
            Mstatus::MIE    => self.csrs[mstatus] >>  3 & 0x1,
            Mstatus::UPIE   => self.csrs[mstatus] >>  4 & 0x1,
            Mstatus::SPIE   => self.csrs[mstatus] >>  5 & 0x1,
            Mstatus::MPIE   => self.csrs[mstatus] >>  7 & 0x1,
            Mstatus::SPP    => self.csrs[mstatus] >>  8 & 0x1,
            Mstatus::MPP    => self.csrs[mstatus] >> 11 & 0x3,
            Mstatus::FS     => self.csrs[mstatus] >> 13 & 0x3,
            Mstatus::XS     => self.csrs[mstatus] >> 15 & 0x3,
            Mstatus::MPRV   => self.csrs[mstatus] >> 17 & 0x1,
            Mstatus::SUM    => self.csrs[mstatus] >> 18 & 0x1,
            Mstatus::MXR    => self.csrs[mstatus] >> 19 & 0x1,
            Mstatus::TVM    => self.csrs[mstatus] >> 20 & 0x1,
            Mstatus::TW     => self.csrs[mstatus] >> 21 & 0x1,
            Mstatus::TSR    => self.csrs[mstatus] >> 22 & 0x1,
            Mstatus::SD     => self.csrs[mstatus] >> 31 & 0x1,
        }
    } 
}

#[allow(non_camel_case_types)]
pub enum CSRname {
    sstatus = 0x100,
    stvec   = 0x105,
    sepc    = 0x141, 
    scause  = 0x142,
    stval   = 0x143,
    satp    = 0x180,
    mstatus = 0x300,
    medeleg = 0x302,
    mtvec   = 0x305,
    mepc    = 0x341, 
    mcause  = 0x342,
    mtval   = 0x343,
}

pub enum Mstatus {
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

