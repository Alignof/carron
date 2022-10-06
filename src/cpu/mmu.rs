use crate::log;
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
        self.ppn = (csrs.read(CSRname::satp.wrap()).unwrap() & 0x3FFFFF) as u32;
        self.trans_mode = match csrs.read(CSRname::satp.wrap()).unwrap() >> 31 & 0x1 {
            1 => AddrTransMode::Sv32,
            _ => AddrTransMode::Bare,
        };
    }

    fn check_pmp(&self, purpose: TransFor, addr: u32, pmpcfg: u32, pmp_r: u32, pmp_w: u32, pmp_x: u32) -> Result<u32, TrapCause> {
        match purpose {
            TransFor::Fetch | TransFor::Deleg => {
                if pmp_x != 1 {
                    log::debugln!("invalid pmp_x: {:x}", pmpcfg);
                    return Err(TrapCause::InstPageFault);
                }
            },
            TransFor::Load => {
                if pmp_r != 1 {
                    log::debugln!("invalid pmp_r: {:x}", pmpcfg);
                    return Err(TrapCause::LoadPageFault);
                }
            },
            TransFor::StoreAMO => {
                if pmp_w != 1 {
                    log::debugln!("invalid pmp_w: {:x}", pmpcfg);
                    return Err(TrapCause::StoreAMOPageFault);
                }
            },
        }

        Ok(addr)
    }

    fn pmp(&self, purpose: TransFor, addr: u32, priv_lv: PrivilegedLevel, csrs: &CSRs) -> Result<u32, TrapCause> {
        let pmpaddrs: Vec<usize> = (0x3B0..0x3BF).collect();
        let get_pmpcfg = |pmpnum| {
            let cfgnum = pmpnum / 4;
            let cfgoff = pmpnum % 4;
            csrs.read(Some(0x3A0 + cfgnum)).unwrap() >> (4 * cfgoff)
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
                    if (index == 0 && addr_aligned < csrs.read(Some(pmpaddrs[index])).unwrap()) ||
                       (index != 0 && csrs.read(Some(pmpaddrs[index-1])).unwrap() <= addr_aligned && addr_aligned < csrs.read(Some(pmpaddrs[index])).unwrap()) {
                           return self.check_pmp(purpose, addr, pmpcfg, pmp_r, pmp_w, pmp_x);
                    }
                },
                0b10 => { // NA4
                    let addr_aligned = addr >> 2; // addr[:2]
                    if addr_aligned == csrs.read(Some(pmpaddrs[index])).unwrap() {
                        return self.check_pmp(purpose, addr, pmpcfg, pmp_r, pmp_w, pmp_x);
                    }
                },
                0b11 => { // NAPOT
                    let mut addr_aligned = addr >> 2; // addr[:2]
                    let mut pmpaddr = csrs.read(Some(pmpaddrs[index])).unwrap();
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

        if priv_lv == PrivilegedLevel::Machine {
            Ok(addr) 
        } else {
            Err(match purpose {
                TransFor::Fetch | TransFor::Deleg => {
                    TrapCause::InstPageFault
                },
                TransFor::Load => {
                    TrapCause::LoadPageFault
                },
                TransFor::StoreAMO => {
                    TrapCause::StoreAMOPageFault
                },
            })
        }
    }

    fn is_leaf_pte(&self, pte: u32) -> bool {
        let pte_r = pte >> 1 & 0x1;
        let pte_x = pte >> 3 & 0x1;

        pte_r == 1 || pte_x == 1
    }

    fn check_pte_validity(&self, purpose: TransFor, pte: u32) -> Result<u32, TrapCause>{
        let pte_v = pte & 0x1;
        let pte_r = pte >> 1 & 0x1;
        let pte_w = pte >> 2 & 0x1;
        let trap_cause = |purpose: TransFor| {
            match purpose {
                TransFor::Fetch => TrapCause::InstPageFault,
                TransFor::Load => TrapCause::LoadPageFault,
                TransFor::StoreAMO => TrapCause::StoreAMOPageFault,
                TransFor::Deleg => TrapCause::InstPageFault,
            }
        };


        // check the PTE validity
        if pte_v == 0 || (pte_r == 0 && pte_w == 1) {
            log::debugln!("invalid pte: {:x}", pte);
            return Err(trap_cause(purpose));
        }

        Ok(pte)
    }

    fn check_leaf_pte(&self, purpose: TransFor, priv_lv: PrivilegedLevel, csrs: &CSRs, pte: u32) -> Result<u32, TrapCause> {
        let pte_r = pte >> 1 & 0x1;
        let pte_w = pte >> 2 & 0x1;
        let pte_x = pte >> 3 & 0x1;
        let pte_u = pte >> 4 & 0x1;
        let pte_a = pte >> 6 & 0x1;
        let pte_d = pte >> 7 & 0x1;
        let trap_cause = |purpose: TransFor| {
            match purpose {
                TransFor::Fetch => TrapCause::InstPageFault,
                TransFor::Load => TrapCause::LoadPageFault,
                TransFor::StoreAMO => TrapCause::StoreAMOPageFault,
                TransFor::Deleg => TrapCause::InstPageFault,
            }
        };

        if let Err(e) = self.check_pte_validity(purpose, pte) {
            return Err(e);
        }

        // check the U bit
        if pte_u == 0 && priv_lv == PrivilegedLevel::User {
            log::debugln!("invalid pte_u: {:x}", pte);
            return Err(trap_cause(purpose));
        }
        match purpose {
            TransFor::Load | TransFor::StoreAMO => {
                let sum = csrs.read_xstatus(PrivilegedLevel::Supervisor, Xstatus::SUM);
                if sum == 0 && pte_u == 1 && priv_lv == PrivilegedLevel::Supervisor {
                    log::debugln!("invalid pte_u: {:x}", pte);
                    return Err(trap_cause(purpose));
                }
            },
            _ => (),
        }
        
        // check the X and R bit
        let mxr = csrs.read_xstatus(PrivilegedLevel::Supervisor, Xstatus::MXR);
        if (mxr == 0 && pte_r == 0) || (mxr == 1 && pte_r == 1 && pte_x == 1) {
            log::debugln!("invalid pte_r or pte_x: {:x}", pte);
            return Err(trap_cause(purpose));
        }
        
        if pte_a == 0 {
            log::debugln!("invalid pte_a: {:x}", pte);
            return Err(trap_cause(purpose));
        }

        // check the PTE field according to translate purpose 
        match purpose {
            TransFor::Fetch | TransFor::Deleg => {
                if pte_x != 1 {
                    log::debugln!("invalid pte_x: {:x}", pte);
                    return Err(TrapCause::InstPageFault);
                }
            },
            TransFor::Load => {
                if pte_r != 1 {
                    log::debugln!("invalid pte_r: {:x}", pte);
                    return Err(TrapCause::LoadPageFault);
                }
            },
            TransFor::StoreAMO => {
                if pte_w != 1 || pte_d == 0 {
                    log::debugln!("invalid pte_w: {:x}", pte);
                    return Err(TrapCause::StoreAMOPageFault);
                }
            },
        }

        log::debugln!("PPN0: 0x{:x}", pte >> 10 & 0x3FF);
        Ok(pte)
    }

    #[allow(non_snake_case)]
    pub fn trans_addr(&mut self, purpose: TransFor, addr: u32, csrs: &CSRs, 
                      dram: &Dram, priv_lv: PrivilegedLevel) -> Result<u32, TrapCause> {

        let trap_cause = |purpose: TransFor| {
            match purpose {
                TransFor::Fetch => TrapCause::InstPageFault,
                TransFor::Load => TrapCause::LoadPageFault,
                TransFor::StoreAMO => TrapCause::StoreAMOPageFault,
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
                        log::debugln!("PTE_addr(1): 0x{:x}", PTE_addr);
                        let PTE = match self.check_pte_validity(purpose, dram.load32(PTE_addr).unwrap() as u32) {
                            Ok(pte) => pte,
                            Err(cause) => {
                                return Err(cause) // exception
                            },
                        };
                        log::debugln!("PTE(1): 0x{:x}", PTE);
                        let PPN1 = PTE >> 20 & 0xFFF;
                        log::debugln!("PPN1: 0x{:x}", PPN1);

                        // complete the trans addr if PTE is the leaf
                        let PPN0 = if self.is_leaf_pte(PTE) {
                                // check misaligned superpage
                                if (PTE >> 10 & 0x1FF) != 0 {
                                    return Err(trap_cause(purpose)) // exception
                                }

                                // check leaf pte and return PPN0
                                match self.check_leaf_pte(purpose, priv_lv, csrs, PTE) {
                                    Ok(_) => VPN0,
                                    Err(cause) => return Err(cause),
                                }
                            } else {
                                // second table walk
                                let PTE_addr = (PTE >> 10 & 0x3FFFFF) * PAGESIZE + VPN0 * PTESIZE;
                                log::debugln!("PTE_addr = (PTE >> 10 & 0x3FFFFF) * PAGESIZE + VPN0 * PTESIZE");
                                log::debugln!("0x{:x} = 0x{:x} * 0x{:x} + 0x{:x} * 0x{:x}",
                                         PTE_addr, (PTE >> 10 & 0x3FFFFF), PAGESIZE, VPN0, PTESIZE);
                                let PTE = match self.check_pte_validity(purpose, dram.load32(PTE_addr).unwrap() as u32) {
                                    Ok(pte) => pte,
                                    Err(cause) => {
                                        return Err(cause) // exception
                                    },
                                };

                                log::debugln!("PTE(2): 0x{:x}", PTE);

                                // check PTE to be leaf
                                if !self.is_leaf_pte(PTE) {
                                    return Err(trap_cause(purpose)) // misaligned superpage
                                }

                                // check leaf pte and return PPN0
                                match self.check_leaf_pte(purpose, priv_lv, csrs, PTE) {
                                    Ok(PTE) => PTE >> 10 & 0x3FF,
                                    Err(cause) => return Err(cause),
                                }
                            };

                        log::debugln!("raw address:{:x}\n\t=> transrated address:{:x}",
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
