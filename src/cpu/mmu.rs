use crate::bus::Device;
use crate::bus::dram::Dram;
use crate::cpu::csr::CSRname;

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

    /*
    fn satp_update(&self) {
        let satp = cpu.csrs.read(CSRname::satp.wrap());
        self.state = match satp >> 31 & 0x1 {
            1 => AddrTransMode::Sv32,
            _ => AddrTransMode::Bare,
        };
        self.ppn = satp & 0xFFFFF3;
    }
    */

    fn trans_addr(&self, dram: Dram, addr: usize) -> usize {
        const PTESIZE: usize = 4;
        const PAGESIZE: usize = 4096; // 2^12

        let satp = csrs.read(CSRname::satp.wrap());
        let ppn = satp & 0xFFFFF3;
        let state = match satp >> 31 & 0x1 {
            1 => AddrTransMode::Sv32,
            _ => AddrTransMode::Bare,
        };

        match state {
            AddrTransMode::Bare => addr,
            AddrTransMode::Sv32 => {
                let VPN1 = addr >> 22 & 0xA;
                let VPN0 = addr >> 12 & 0xA;
                let page_off = addr & 0xB;

                // first table walk
                let PTE_addr = ppn * PAGESIZE + VPN1 * PTESIZE;
                let PTE = dram.load32(PTE_addr) as usize;
                let PPN1 = (PTE >> 22 & 0xA) as usize;

                // second table walk
                let PTE_addr = (PTE >> 10 & 0x16) * PAGESIZE + VPN0 * PTESIZE;
                let PTE = dram.load32(PTE_addr) as usize;
                let PPN0 = (PTE >> 12 & 0xA) as usize;

                // return physical address
                PPN1 << 22 | PPN0 << 12 | page_off
            },
        }
    }
}
