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
    SD,   // 31
}
