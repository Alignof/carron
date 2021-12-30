use crate::bus::Device;
use crate::bus::dram::Dram;
use crate::cpu::{PrivilegedLevel, TransFor};

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

        // check the PTE validity
        if pte_v == 0 || (pte_r == 0 && pte_w == 1) {
            println!("invalid pte: {:x}", pte);
            return Err(());
        }

        Ok(pte as u32)
    }

    fn is_leaf_pte(&self, pte: u32) -> bool {
        let pte_r = pte >> 1 & 0x1;
        let pte_w = pte >> 2 & 0x1;
        let pte_x = pte >> 3 & 0x1;

        pte_r == 1 || pte_w == 1 || pte_x == 1
    }

    fn check_leaf_pte(&self, purpose: &TransFor, priv_lv: &PrivilegedLevel, pte: u32) -> Result<u32, ()> {
        let pte_r = pte >> 1 & 0x1;
        let pte_w = pte >> 2 & 0x1;
        let pte_x = pte >> 3 & 0x1;
        let pte_u = pte >> 4 & 0x1;

        // check the U bit
        if priv_lv == &PrivilegedLevel::User && pte_u != 1 {
            println!("invalid pte_u: {:x}", pte);
            return Err(());
        }

        // check the PTE field according to translate purpose 
        match purpose {
            TransFor::Fetch => {
                if pte_x != 1 {
                    println!("invalid pte_x: {:x}", pte);
                    return Err(());
                }
            },
            TransFor::Load => {
                if pte_r != 1 {
                    println!("invalid pte_r: {:x}", pte);
                    return Err(());
                }
            },
            TransFor::Store => {
                if pte_w != 1 {
                    println!("invalid pte_w: {:x}", pte);
                    return Err(());
                }
            },
            _ => (),
        }

        println!("PPN0: 0x{:x}", pte >> 10 & 0x3FF);
        Ok(pte)
    }

    #[allow(non_snake_case)]
    pub fn trans_addr(&mut self, purpose: TransFor, addr: u32, satp: u32, 
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
                        let PTE_addr = self.ppn * PAGESIZE + VPN1 * PTESIZE;
                        println!("PTE_addr(1): 0x{:x}", PTE_addr);
                        let PTE = match self.check_pte_validity(dram.load32(PTE_addr)) {
                            Ok(pte) => pte,
                            Err(()) => {
                                return Err(()) // exception
                            },
                        };
                        println!("PTE(1): 0x{:x}", PTE);
                        let PPN1 = PTE >> 20 & 0xFFF;
                        println!("PPN1: 0x{:x}", PPN1);

                        // complete the trans addr if PTE is the leaf
                        let PPN0 = if self.is_leaf_pte(PTE) {
                                // check misaligned superpage
                                if (PTE >> 10 & 0x3FF) != 0 {
                                    return Err(()) // exception
                                }

                                // check leaf pte and return PPN0
                                match self.check_leaf_pte(&purpose, priv_lv, PTE) {
                                    Ok(PTE) => VPN0,
                                    Err(()) => return Err(()),
                                }
                            } else {
                                // second table walk
                                let PTE_addr = (PTE >> 10 & 0x3FFFFF) * PAGESIZE + VPN0 * PTESIZE;
                                println!("PTE_addr = (PTE >> 10 & 0x3FFFFF) * PAGESIZE + VPN0 * PTESIZE");
                                println!("0x{:x} = 0x{:x} * 0x{:x} + 0x{:x} * 0x{:x}",
                                         PTE_addr, (PTE >> 10 & 0x3FFFFF), PAGESIZE, VPN0, PTESIZE);
                                let PTE = match self.check_pte_validity(dram.load32(PTE_addr)) {
                                    Ok(pte) => pte,
                                    Err(()) => {
                                        return Err(()) // exception
                                    },
                                };

                                println!("PTE(2): 0x{:x}", PTE);

                                // check PTE to be leaf
                                if !self.is_leaf_pte(PTE) {
                                    return Err(()) // exception
                                }

                                // check leaf pte and return PPN0
                                match self.check_leaf_pte(&purpose, priv_lv, PTE) {
                                    Ok(PTE) => PTE >> 10 & 0x3FF,
                                    Err(()) => return Err(()),
                                }
                            };

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
