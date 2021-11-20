use crate::bus::Device;
use crate::bus::dram::Dram;
use crate::cpu::PrivilegedLevel;
use dbg_hex::dbg_hex;

pub enum AddrTransMode {
    Bare,
    Sv32,
}

pub struct MMU {
    ppn: u32,
    trans_mode: AddrTransMode,
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            ppn: 0,
            trans_mode: AddrTransMode::Bare,
        }
    }

    fn update_data(&mut self, satp: u32) {
        self.ppn = (satp & 0x3FFFFF) as u32;
        self.trans_mode = match satp >> 31 & 0x1 {
            1 => AddrTransMode::Sv32,
            _ => AddrTransMode::Bare,
        };
    }

    fn check_pte_validity(&self, pte: i32) -> Result<u32, ()>{
        let pte_v = pte & 0x1;
        let pte_r = pte >> 1 & 0x1;
        let pte_w = pte >> 2 & 0x1;

        if pte_v == 0 || (pte_r == 0 && pte_w == 1) {
            Err(())
        } else {
            Ok(pte as u32)
        }
    }

    fn check_leaf_pte(&self, pte: u32) -> bool {
        let pte_r = pte >> 1 & 0x1;
        let pte_x = pte >> 3 & 0x1;

        pte_r == 1 || pte_x == 1
    }

    #[allow(non_snake_case)]
    pub fn trans_addr(&mut self, addr: u32, satp: u32, 
                      dram: &Dram, priv_lv: &PrivilegedLevel) -> Result<u32, ()> {

        // update trans_mode and ppn
        self.update_data(satp);

        match priv_lv {
            PrivilegedLevel::Supervisor |
            PrivilegedLevel::User => {
                match self.trans_mode {
                    AddrTransMode::Bare => Ok(addr),
                    AddrTransMode::Sv32 => {
                        const PTESIZE: u32 = 4;
                        const PAGESIZE: u32 = 4096; // 2^12

                        let VPN1 = addr >> 22 & 0x3FF;
                        let VPN0 = addr >> 12 & 0x3FF;
                        let page_off = addr & 0xFFF;

                        // first table walk
                        dbg_hex!(satp);
                        dbg_hex!(self.ppn);
                        dbg_hex!(PAGESIZE);
                        dbg_hex!(VPN1);
                        dbg_hex!(PTESIZE);
                        let PTE_addr = self.ppn * PAGESIZE + VPN1 * PTESIZE;
                        println!("PTE_addr(1): 0x{:x}", PTE_addr);
                        let PTE = match self.check_pte_validity(dram.load32(PTE_addr - dram.base_addr)) {
                            Ok(pte) => pte,
                            Err(()) => {
                                return Err(()) // exception
                            },
                        };
                        println!("PTE(1): 0x{:x}", PTE);
                        let PPN1 = PTE >> 20 & 0xFFF;

                        // second table walk
                        let PTE_addr = (PTE >> 10 & 0x3FFFFF) * PAGESIZE + VPN0 * PTESIZE;
                        println!("PTE_addr(2): 0x{:x}", PTE_addr);
                        let PTE = match self.check_pte_validity(dram.load32(PTE_addr - dram.base_addr)) {
                            Ok(pte) => pte,
                            Err(()) => {
                                return Err(()) // exception
                            },
                        };
                        println!("PTE(2): 0x{:x}", PTE);
                        let PPN0 = PTE >> 10 & 0x3FF;

                        // check PTE to be leaf
                        if !self.check_leaf_pte(PTE) {
                            return Err(()) // exception
                        }

                        println!("raw address:{:x}\n\t=> transrated address:{:x}",
                                 addr, PPN1 << 22 | PPN0 << 12 | page_off);

                        // return transrated address
                        Ok(PPN1 << 22 | PPN0 << 12 | page_off)
                    },
                }
            },
            // return raw address if privileged level is Machine
            _ => Ok(addr),
        }
    }
}
