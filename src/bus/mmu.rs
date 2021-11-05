use std::rc::Rc;
use std::cell::RefCell;
use crate::bus::Device;
use crate::bus::dram::Dram;
use crate::cpu;
use crate::cpu::PrivilegedLevel;
use crate::cpu::csr::CSRname;

pub enum AddrTransMode {
    Bare,
    Sv32,
}

pub struct MMU {
    csrs: Rc<RefCell<cpu::csr::CSRs>>,
    priv_lv: Rc<RefCell<cpu::PrivilegedLevel>>
}

impl MMU {
    pub fn new(csrs: Rc<RefCell<cpu::csr::CSRs>>,
               priv_lv: Rc<RefCell<cpu::PrivilegedLevel>>) -> MMU {
        MMU {
            csrs,
            priv_lv,
        }
    }

    #[allow(non_snake_case)]
    pub fn trans_addr(&self, dram: &Dram, addr: usize) -> usize {
        const PTESIZE: usize = 4;
        const PAGESIZE: usize = 4096; // 2^12

        let satp = self.csrs.borrow().read(CSRname::satp.wrap());
        let ppn = (satp & 0xFFFFF3) as usize;
        let state = match satp >> 31 & 0x1 {
            1 => AddrTransMode::Sv32,
            _ => AddrTransMode::Bare,
        };

        match *(self.priv_lv.borrow()) {
            PrivilegedLevel::Supervisor |
            PrivilegedLevel::User => {
                match state {
                    AddrTransMode::Bare => addr,
                    AddrTransMode::Sv32 => {
                        let VPN1 = addr >> 22 & 0x3FF;
                        let VPN0 = addr >> 12 & 0x3FF;
                        let page_off = addr & 0xFFF;

                        // first table walk
                        let PTE_addr = ppn * PAGESIZE + VPN1 * PTESIZE;
                        println!("PTE_addr(1):{:x}\n", PTE_addr);
                        let PTE = dram.load32(PTE_addr) as usize;
                        let PPN1 = (PTE >> 20 & 0xFFF) as usize;

                        // second table walk
                        let PTE_addr = (PTE >> 10 & 0xFFFFF3) * PAGESIZE + VPN0 * PTESIZE;
                        println!("PTE_addr(2):{:x}\n", PTE_addr);
                        let PTE = dram.load32(PTE_addr) as usize;
                        let PPN0 = (PTE >> 10 & 0x3FF) as usize;

                        println!("raw address:{:x}\n\t=> transrated address:{:x}",
                                 addr, PPN1 << 22 | PPN0 << 12 | page_off);

                        // return transrated address
                        PPN1 << 22 | PPN0 << 12 | page_off
                    },
                }
            },
            // return raw address if privileged level is Machine
            _ => addr,
        }
    }
}
