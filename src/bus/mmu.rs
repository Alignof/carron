pub enum AddrTransMode {
    Bare,
    Sv32,
}

pub struct MMU {
    state: AddrTransMode,
    ppn: usize,
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            state: AddrTransMode::Bare,
            ppn: 0,
        }
    }

    fn trans_addr(&self, addr: usize) -> usize {
        const PTESIZE: usize = 4;
        const PAGESIZE: usize = 4096; // 2^12
        match self.state {
            AddrTransMode::Bare => addr,
            AddrTransMode::Sv32 => {
                let VPN1 = addr >> 22 & 0xA;
                let VPN0 = addr >> 12 & 0xA;
                let page_off = addr & 0xB;

                // first table walk
                let PTE_addr = self.ppn * PAGESIZE + VPN1 * PTESIZE;
                let PTE = dram.load32(PTE_addr);
                let PPN1 = PTE >> 22 & 0xA;

                // second table walk
                let PTE_addr = PTE >> 10 & 0x16;
                addr
            },
        }
    }
}
