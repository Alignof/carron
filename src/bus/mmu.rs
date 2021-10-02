pub enum AddrTransMode {
    Bare,
    Sv32,
}

pub struct MMU {
    state: AddrTransMode,
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            state: AddrTransMode::Bare,
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
                addr
            },
        }
    }
}
