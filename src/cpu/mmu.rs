mod pmp;
mod tlb;

use crate::bus::dram::Dram;
use crate::bus::Device;
use crate::cpu::csr::CSRs;
use crate::cpu::{CSRname, PrivilegedLevel, TransFor, TrapCause, Xstatus};
use crate::{log, Isa};
use std::rc::Rc;

pub enum AddrTransMode {
    Bare,
    Sv32,
    Sv39,
}

pub struct Mmu {
    ppn: u64,
    trans_mode: AddrTransMode,
    tlb: tlb::Tlb,
    isa: Rc<Isa>,
}

impl Mmu {
    pub fn new(isa: Rc<Isa>) -> Self {
        Mmu {
            ppn: 0,
            trans_mode: AddrTransMode::Bare,
            tlb: tlb::Tlb::new(),
            isa,
        }
    }

    pub fn flush_tlb(&mut self) {
        self.tlb.flush();
    }

    fn update_ppn_and_mode(&mut self, csrs: &CSRs) {
        let satp = csrs.read(CSRname::satp.wrap()).unwrap();
        self.ppn = match *self.isa {
            Isa::Rv32 => satp & 0x3FFFFF,
            Isa::Rv64 => satp & 0xFFFFFFFFFFF,
        };
        self.trans_mode = match *self.isa {
            Isa::Rv32 => match satp >> 31 & 0x1 {
                1 => AddrTransMode::Sv32,
                _ => AddrTransMode::Bare,
            },
            Isa::Rv64 => match satp >> 60 & 0xf {
                8 => AddrTransMode::Sv39,
                _ => AddrTransMode::Bare,
            },
        };
    }

    fn is_leaf_pte(&self, pte: u64) -> bool {
        let pte_r = pte >> 1 & 0x1;
        let pte_x = pte >> 3 & 0x1;

        pte_r == 1 || pte_x == 1
    }

    fn check_pte_validity(&self, purpose: TransFor, pte: u64) -> Result<u64, TrapCause> {
        let pte_v = pte & 0x1;
        let pte_r = pte >> 1 & 0x1;
        let pte_w = pte >> 2 & 0x1;

        // check the PTE validity
        if pte_v == 0 || (pte_r == 0 && pte_w == 1) {
            log::debugln!("invalid pte: {:x}", pte);
            return Err(self.trap_cause(purpose));
        }

        Ok(pte)
    }

    fn trap_cause(&self, purpose: TransFor) -> TrapCause {
        match purpose {
            TransFor::Fetch => TrapCause::InstPageFault,
            TransFor::Load => TrapCause::LoadPageFault,
            TransFor::StoreAMO => TrapCause::StoreAMOPageFault,
            TransFor::Deleg => TrapCause::InstPageFault,
        }
    }

    fn check_leaf_pte(
        &self,
        purpose: TransFor,
        priv_lv: PrivilegedLevel,
        csrs: &CSRs,
        pte: u64,
    ) -> Result<u64, TrapCause> {
        let pte_r = pte >> 1 & 0x1;
        let pte_w = pte >> 2 & 0x1;
        let pte_x = pte >> 3 & 0x1;
        let pte_u = pte >> 4 & 0x1;
        let pte_a = pte >> 6 & 0x1;
        let pte_d = pte >> 7 & 0x1;

        self.check_pte_validity(purpose, pte)?;

        // check the U bit
        if pte_u == 0 && priv_lv == PrivilegedLevel::User {
            log::debugln!("invalid pte_u: {:x}", pte);
            return Err(self.trap_cause(purpose));
        }

        // check the A bit
        if pte_a == 0 {
            log::debugln!("invalid pte_a: {:x}", pte);
            return Err(self.trap_cause(purpose));
        }

        // check the PTE field according to translate purpose
        match purpose {
            TransFor::Fetch | TransFor::Deleg => {
                if pte_x == 0 {
                    log::debugln!("invalid pte_x: {:x}", pte);
                    return Err(TrapCause::InstPageFault);
                }
            }
            TransFor::Load => {
                // check sum bit
                let sum = csrs.read_xstatus(PrivilegedLevel::Machine, Xstatus::SUM);
                if sum == 0 && pte_u == 1 && priv_lv == PrivilegedLevel::Supervisor {
                    log::debugln!("[SUM] invalid pte_u: {:x}", pte);
                    return Err(self.trap_cause(purpose));
                }

                // check the X and R bit
                let mxr = csrs.read_xstatus(PrivilegedLevel::Machine, Xstatus::MXR);
                if pte_r == 0 && (mxr == 0 || pte_x == 0) {
                    log::debugln!("[MXR == {}] invalid pte_r or pte_x: {:x}", mxr, pte);
                    return Err(TrapCause::LoadPageFault);
                }
            }
            TransFor::StoreAMO => {
                // check sum bit
                let sum = csrs.read_xstatus(PrivilegedLevel::Machine, Xstatus::SUM);
                if sum == 0 && pte_u == 1 && priv_lv == PrivilegedLevel::Supervisor {
                    log::debugln!("[SUM] invalid pte_u: {:x}", pte);
                    return Err(self.trap_cause(purpose));
                }

                if pte_w == 0 || pte_d == 0 {
                    log::debugln!("invalid pte_w: {:x}", pte);
                    return Err(TrapCause::StoreAMOPageFault);
                }
            }
        }

        log::debugln!("PPN0: 0x{:x}", pte >> 10 & 0x3FF);
        Ok(pte)
    }

    #[allow(non_snake_case)]
    pub fn trans_addr(
        &mut self,
        purpose: TransFor,
        vaddr: u64,
        csrs: &CSRs,
        dram: &mut Dram,
        priv_lv: PrivilegedLevel,
    ) -> Result<u64, TrapCause> {
        const PAGESIZE: u64 = 4096; // 2^12

        // update trans_mode and ppn
        self.update_ppn_and_mode(csrs);

        match priv_lv {
            PrivilegedLevel::Supervisor | PrivilegedLevel::User => match self.trans_mode {
                AddrTransMode::Bare => Ok(vaddr),
                AddrTransMode::Sv32 | AddrTransMode::Sv39 => {
                    if let Some(paddr) = self.tlb.lookup(vaddr) {
                        return Ok(paddr);
                    }

                    let page_off = vaddr & 0xFFF;
                    let mut ppn = self.ppn;
                    let vpn = match *self.isa {
                        Isa::Rv32 => [vaddr >> 12 & 0x3FF, vaddr >> 22 & 0x3FF, 0],
                        Isa::Rv64 => [
                            vaddr >> 12 & 0x1FF,
                            vaddr >> 21 & 0x1FF,
                            vaddr >> 30 & 0x1FF,
                        ],
                    };
                    let pte_size: u64 = match *self.isa {
                        Isa::Rv32 => 4,
                        Isa::Rv64 => 8,
                    };
                    let mut level: i32 = match *self.isa {
                        Isa::Rv32 => 1, // 2 - 1
                        Isa::Rv64 => 2, // 3 - 1
                    };

                    let pte = loop {
                        let pte_vaddr = ppn * PAGESIZE + vpn[level as usize] * pte_size;
                        log::debugln!("pte_vaddr({}): 0x{:x}", level, pte_vaddr);
                        let pte =
                            match *self.isa {
                                Isa::Rv32 => self
                                    .check_pte_validity(purpose, dram.load32(pte_vaddr).unwrap())?,
                                Isa::Rv64 => self
                                    .check_pte_validity(purpose, dram.load64(pte_vaddr).unwrap())?,
                            };
                        log::debugln!("pte({}): 0x{:x}", level, pte);

                        if self.is_leaf_pte(pte) {
                            break pte;
                        }

                        level -= 1;
                        if level < 0 {
                            return Err(self.trap_cause(purpose));
                        }

                        ppn = match *self.isa {
                            Isa::Rv32 => pte >> 10 & 0x3fff_ffff,
                            Isa::Rv64 => pte >> 10 & 0xfff_ffff_ffff,
                        };
                        log::debugln!("PPN{}: 0x{:x}", level, ppn);
                    };

                    self.check_leaf_pte(purpose, priv_lv, csrs, pte)?;
                    let ppn = match *self.isa {
                        Isa::Rv32 => [pte >> 10 & 0x3FF, pte >> 20 & 0xFFF, 0],
                        Isa::Rv64 => [pte >> 10 & 0x1FF, pte >> 19 & 0x1FF, pte >> 28 & 0x3FF_FFFF],
                    };

                    let paddr = match *self.isa {
                        Isa::Rv32 => match level {
                            0 => ppn[1] << 22 | ppn[0] << 12 | page_off,
                            1 => {
                                if ppn[0] != 0 {
                                    return Err(self.trap_cause(purpose));
                                }
                                ppn[1] << 22 | vpn[0] << 12 | page_off
                            }
                            _ => return Err(self.trap_cause(purpose)),
                        },
                        Isa::Rv64 => match level {
                            0 => ppn[2] << 30 | ppn[1] << 21 | ppn[0] << 12 | page_off,
                            1 => {
                                if ppn[0] != 0 {
                                    return Err(self.trap_cause(purpose));
                                }
                                ppn[2] << 30 | ppn[1] << 21 | vpn[0] << 12 | page_off
                            }
                            2 => {
                                if ppn[0] != 0 || ppn[1] != 0 {
                                    return Err(self.trap_cause(purpose));
                                }
                                ppn[2] << 30 | vpn[1] << 21 | vpn[0] << 12 | page_off
                            }
                            _ => return Err(self.trap_cause(purpose)),
                        },
                    };

                    log::debugln!(
                        "raw vaddress:{:x}\n\t=> transrated vaddress:{:x}",
                        vaddr,
                        paddr,
                    );

                    self.tlb.refill_tlb(vaddr, paddr);
                    self.pmp(purpose, paddr, priv_lv, csrs)
                }
            },
            // return raw vaddress if privileged level is Machine
            _ => self.pmp(purpose, vaddr, priv_lv, csrs),
        }
    }
}
