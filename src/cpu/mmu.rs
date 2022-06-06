use crate::bus::Device;
use crate::bus::dram::Dram;
use crate::cpu::{PrivilegedLevel, TransFor, TrapCause};
use crate::cpu::csr::{CSRs, CSRname, Xstatus};

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

    fn update_ppn_and_mode(&mut self, csrs: &CSRs) {
        self.ppn = (csrs.read(CSRname::satp.wrap()) & 0x3FFFFF) as u32;
        self.trans_mode = match csrs.read(CSRname::satp.wrap()) >> 31 & 0x1 {
            1 => AddrTransMode::Sv32,
            _ => AddrTransMode::Bare,
        };
    }

    fn check_pmp(&self, purpose: TransFor, addr: u32, pmpcfg: u32, pmp_r: u32, pmp_w: u32, pmp_x: u32) -> Result<u32, TrapCause> {
        match purpose {
            TransFor::Fetch | TransFor::Deleg => {
                if pmp_x != 1 {
                    println!("invalid pmp_x: {:x}", pmpcfg);
                    return Err(TrapCause::InstPageFault);
                }
            },
            TransFor::Load => {
                if pmp_r != 1 {
                    println!("invalid pmp_r: {:x}", pmpcfg);
                    return Err(TrapCause::LoadPageFault);
                }
            },
            TransFor::Store => {
                if pmp_w != 1 {
                    println!("invalid pmp_w: {:x}", pmpcfg);
                    return Err(TrapCause::StorePageFault);
                }
            },
        }

        Ok(addr)
    }

    fn pmp(&self, purpose: TransFor, addr: u32, priv_lv: &PrivilegedLevel, csrs: &CSRs) -> Result<u32, TrapCause> {
        let pmpaddrs = [0x3B0, 0x3B1, 0x3B2, 0x3B3, 0x3B4, 0x3B5, 0x3B6, 0x3B7, 0x3B8, 0x3B9, 0x3BA, 0x3BB, 0x3BC, 0x3BD, 0x3BE, 0x3BF];
        let get_pmpcfg = |pmpnum| {
            let cfgnum = pmpnum / 4;
            let cfgoff = pmpnum % 4;
            csrs.read(Some(0x3A0 + cfgnum)) >> (4 * cfgoff)
        };

        for index in 0 .. pmpaddrs.len() { // pmpaddr0 ~ pmpaddr15
            let pmpcfg = get_pmpcfg(index);
            let pmp_r = pmpcfg & 0x1;
            let pmp_w = pmpcfg >> 1 & 0x1;
            let pmp_x = pmpcfg >> 2 & 0x1;
            let pmp_a = pmpcfg >> 3 & 0x3;
            match pmp_a {
                0b00 => return Ok(addr),
                0b01 => { // TOR
                    let addr_aligned = addr >> 2; // addr[:2]
                    if (index == 0 && addr_aligned < csrs.read(Some(pmpaddrs[index]))) ||
                       (index != 0 && csrs.read(Some(pmpaddrs[index-1])) <= addr_aligned && addr_aligned < csrs.read(Some(pmpaddrs[index]))) {
                           return self.check_pmp(purpose, addr, pmpcfg, pmp_r, pmp_w, pmp_x);
                    }
                },
                0b10 => { // NA4
                    let addr_aligned = addr >> 2; // addr[:2]
                    if addr_aligned == csrs.read(Some(pmpaddrs[index])) {
                        return self.check_pmp(purpose, addr, pmpcfg, pmp_r, pmp_w, pmp_x);
                    }
                },
                0b11 => { // NAPOT
                    let mut addr_aligned = addr >> 2; // addr[:2]
                    let mut pmpaddr = csrs.read(Some(pmpaddrs[index]));
                    while pmpaddr & 0x1 == 1 {
                        pmpaddr >>= 1;
                        addr_aligned >>= 1;
                    }
                    pmpaddr >>= 1;
                    addr_aligned >>= 1;

                    if addr_aligned == pmpaddr {
                        return self.check_pmp(purpose, addr, pmpcfg, pmp_r, pmp_w, pmp_x);
                    }
                },
                _ => panic!("pmp_a does not matched"),
            }
        }

        if priv_lv == &PrivilegedLevel::Machine {
            Ok(addr) 
        } else {
            Err(match purpose {
                TransFor::Fetch | TransFor::Deleg => {
                    TrapCause::InstPageFault
                },
                TransFor::Load => {
                    TrapCause::LoadPageFault
                },
                TransFor::Store => {
                    TrapCause::StorePageFault
                },
            })
        }
    }

    fn is_leaf_pte(&self, pte: u32) -> bool {
        let pte_r = pte >> 1 & 0x1;
        let pte_x = pte >> 3 & 0x1;

        pte_r == 1 || pte_x == 1
    }

    fn check_pte_validity(&self, purpose: &TransFor, pte: u32) -> Result<u32, TrapCause>{
        let pte_v = pte & 0x1;
        let pte_r = pte >> 1 & 0x1;
        let pte_w = pte >> 2 & 0x1;
        let trap_cause = |purpose: &TransFor| {
            match purpose {
                TransFor::Fetch => TrapCause::InstPageFault,
                TransFor::Load => TrapCause::LoadPageFault,
                TransFor::Store => TrapCause::StorePageFault,
                TransFor::Deleg => TrapCause::InstPageFault,
            }
        };


        // check the PTE validity
        if pte_v == 0 || (pte_r == 0 && pte_w == 1) {
            println!("invalid pte: {:x}", pte);
            return Err(trap_cause(purpose));
        }

        Ok(pte)
    }

    fn check_leaf_pte(&self, purpose: &TransFor, priv_lv: &PrivilegedLevel, csrs: &CSRs, pte: u32) -> Result<u32, TrapCause> {
        let pte_r = pte >> 1 & 0x1;
        let pte_w = pte >> 2 & 0x1;
        let pte_x = pte >> 3 & 0x1;
        let pte_u = pte >> 4 & 0x1;
        let pte_a = pte >> 6 & 0x1;
        let pte_d = pte >> 7 & 0x1;
        let trap_cause = |purpose: &TransFor| {
            match purpose {
                TransFor::Fetch => TrapCause::InstPageFault,
                TransFor::Load => TrapCause::LoadPageFault,
                TransFor::Store => TrapCause::StorePageFault,
                TransFor::Deleg => TrapCause::InstPageFault,
            }
        };

        if let Err(e) = self.check_pte_validity(purpose, pte) {
            return Err(e);
        }

        // check the U bit
        if pte_u == 0 && priv_lv == &PrivilegedLevel::User {
            println!("invalid pte_u: {:x}", pte);
            return Err(trap_cause(purpose));
        }
        match purpose {
            TransFor::Load | TransFor::Store => {
                let sum = csrs.read_xstatus(priv_lv, Xstatus::SUM);
                if sum == 0 && pte_u == 1 && priv_lv == &PrivilegedLevel::Supervisor {
                    dbg!(priv_lv);
                    println!("invalid pte_u: {:x}", pte);
                    return Err(trap_cause(purpose));
                }
            },
            _ => (),
        }
        
        // check the X and R bit
        let mxr = csrs.read_xstatus(priv_lv, Xstatus::MXR);
        if (mxr == 0 && pte_r == 0) || (mxr == 1 && pte_r == 1 && pte_x == 1) {
            println!("invalid pte_r or pte_x: {:x}", pte);
            return Err(trap_cause(purpose));
        }
        
        if pte_a == 0 {
            println!("invalid pte_a: {:x}", pte);
            return Err(trap_cause(purpose));
        }

        // check the PTE field according to translate purpose 
        match purpose {
            TransFor::Fetch | TransFor::Deleg => {
                if pte_x != 1 {
                    println!("invalid pte_x: {:x}", pte);
                    return Err(TrapCause::InstPageFault);
                }
            },
            TransFor::Load => {
                if pte_r != 1 {
                    println!("invalid pte_r: {:x}", pte);
                    return Err(TrapCause::LoadPageFault);
                }
            },
            TransFor::Store => {
                if pte_w != 1 || pte_d == 0 {
                    println!("invalid pte_w: {:x}", pte);
                    return Err(TrapCause::StorePageFault);
                }
            },
        }

        println!("PPN0: 0x{:x}", pte >> 10 & 0x3FF);
        Ok(pte)
    }

    #[allow(non_snake_case)]
    pub fn trans_addr(&mut self, purpose: TransFor, addr: u32, csrs: &CSRs, 
                      dram: &Dram, priv_lv: &PrivilegedLevel) -> Result<u32, TrapCause> {

        let trap_cause = |purpose: &TransFor| {
            match purpose {
                TransFor::Fetch => TrapCause::InstPageFault,
                TransFor::Load => TrapCause::LoadPageFault,
                TransFor::Store => TrapCause::StorePageFault,
                TransFor::Deleg => TrapCause::InstPageFault,
            }
        };
        // update trans_mode and ppn
        self.update_ppn_and_mode(csrs);

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
                        let PTE = match self.check_pte_validity(&purpose, dram.load32(PTE_addr) as u32) {
                            Ok(pte) => pte,
                            Err(cause) => {
                                return Err(cause) // exception
                            },
                        };
                        println!("PTE(1): 0x{:x}", PTE);
                        let PPN1 = PTE >> 20 & 0xFFF;
                        println!("PPN1: 0x{:x}", PPN1);

                        // complete the trans addr if PTE is the leaf
                        let PPN0 = if self.is_leaf_pte(PTE) {
                                // check misaligned superpage
                                if (PTE >> 10 & 0x1FF) != 0 {
                                    return Err(trap_cause(&purpose)) // exception
                                }

                                // check leaf pte and return PPN0
                                match self.check_leaf_pte(&purpose, priv_lv, csrs, PTE) {
                                    Ok(_) => VPN0,
                                    Err(cause) => return Err(cause),
                                }
                            } else {
                                // second table walk
                                let PTE_addr = (PTE >> 10 & 0x3FFFFF) * PAGESIZE + VPN0 * PTESIZE;
                                println!("PTE_addr = (PTE >> 10 & 0x3FFFFF) * PAGESIZE + VPN0 * PTESIZE");
                                println!("0x{:x} = 0x{:x} * 0x{:x} + 0x{:x} * 0x{:x}",
                                         PTE_addr, (PTE >> 10 & 0x3FFFFF), PAGESIZE, VPN0, PTESIZE);
                                let PTE = match self.check_pte_validity(&purpose, dram.load32(PTE_addr) as u32) {
                                    Ok(pte) => pte,
                                    Err(cause) => {
                                        return Err(cause) // exception
                                    },
                                };

                                println!("PTE(2): 0x{:x}", PTE);

                                // check PTE to be leaf
                                if !self.is_leaf_pte(PTE) {
                                    return Err(trap_cause(&purpose)) // misaligned superpage
                                }

                                // check leaf pte and return PPN0
                                match self.check_leaf_pte(&purpose, priv_lv, csrs, PTE) {
                                    Ok(PTE) => PTE >> 10 & 0x3FF,
                                    Err(cause) => return Err(cause),
                                }
                            };

                        println!("raw address:{:x}\n\t=> transrated address:{:x}",
                                 addr, PPN1 << 22 | PPN0 << 12 | page_off);

                        // check pmp and return transrated address
                        self.pmp(purpose, PPN1 << 22 | PPN0 << 12 | page_off, priv_lv, csrs)
                    },
                }
            },
            // return raw address if privileged level is Machine
            _ => self.pmp(purpose, addr, priv_lv, csrs),
        }
    }
}
