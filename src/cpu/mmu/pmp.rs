use super::Mmu;
use crate::cpu::csr::CSRs;
use crate::cpu::{PrivilegedLevel, TransFor, TrapCause};
use crate::log;

impl Mmu {
    fn check_pmp(
        &self,
        purpose: TransFor,
        addr: u64,
        pmpcfg: u64,
        pmp_r: u64,
        pmp_w: u64,
        pmp_x: u64,
    ) -> Result<u64, TrapCause> {
        match purpose {
            TransFor::Fetch | TransFor::Deleg => {
                if pmp_x != 1 {
                    log::debugln!("invalid pmp_x: {:x}", pmpcfg);
                    return Err(TrapCause::InstPageFault);
                }
            }
            TransFor::Load => {
                if pmp_r != 1 {
                    log::debugln!("invalid pmp_r: {:x}", pmpcfg);
                    return Err(TrapCause::LoadPageFault);
                }
            }
            TransFor::StoreAMO => {
                if pmp_w != 1 {
                    log::debugln!("invalid pmp_w: {:x}", pmpcfg);
                    return Err(TrapCause::StoreAMOPageFault);
                }
            }
        }

        Ok(addr)
    }

    pub fn pmp(
        &self,
        purpose: TransFor,
        addr: u64,
        priv_lv: PrivilegedLevel,
        csrs: &CSRs,
    ) -> Result<u64, TrapCause> {
        match priv_lv {
            PrivilegedLevel::Machine => Ok(addr),
            PrivilegedLevel::Reserved => panic!("PrivilegedLevel 0x3 is Reserved."),
            PrivilegedLevel::Supervisor | PrivilegedLevel::User => {
                const PMPADDRS: [usize; 16] = [
                    0x3B0, 0x3B1, 0x3B2, 0x3B3, 0x3B4, 0x3B5, 0x3B6, 0x3B7, 0x3B8, 0x3B9, 0x3BA,
                    0x3BB, 0x3BC, 0x3BD, 0x3BE, 0x3BF,
                ];
                let get_pmpcfg = |pmpnum| {
                    let cfgnum = pmpnum / 4;
                    let cfgoff = pmpnum % 4;
                    csrs.read(Some(0x3A0 + cfgnum)).unwrap() >> (4 * cfgoff)
                };

                for index in 0..PMPADDRS.len() {
                    // pmpaddr0 ~ pmpaddr15
                    let pmpcfg = get_pmpcfg(index);
                    let pmp_r = pmpcfg & 0x1;
                    let pmp_w = pmpcfg >> 1 & 0x1;
                    let pmp_x = pmpcfg >> 2 & 0x1;
                    let pmp_a = pmpcfg >> 3 & 0x3;
                    match pmp_a {
                        0b00 => return Ok(addr),
                        0b01 => {
                            // TOR
                            let addr_aligned = addr >> 2; // addr[:2]
                            if (index == 0
                                && addr_aligned < csrs.read(Some(PMPADDRS[index])).unwrap())
                                || (index != 0
                                    && csrs.read(Some(PMPADDRS[index - 1])).unwrap()
                                        <= addr_aligned
                                    && addr_aligned < csrs.read(Some(PMPADDRS[index])).unwrap())
                            {
                                return self.check_pmp(purpose, addr, pmpcfg, pmp_r, pmp_w, pmp_x);
                            }
                        }
                        0b10 => {
                            // NA4
                            let addr_aligned = addr >> 2; // addr[:2]
                            if addr_aligned == csrs.read(Some(PMPADDRS[index])).unwrap() {
                                return self.check_pmp(purpose, addr, pmpcfg, pmp_r, pmp_w, pmp_x);
                            }
                        }
                        0b11 => {
                            // NAPOT
                            let mut addr_aligned = addr >> 2; // addr[:2]
                            let mut pmpaddr = csrs.read(Some(PMPADDRS[index])).unwrap();
                            while pmpaddr & 0x1 == 1 {
                                pmpaddr >>= 1;
                                addr_aligned >>= 1;
                            }
                            pmpaddr >>= 1;
                            addr_aligned >>= 1;

                            if addr_aligned == pmpaddr {
                                return self.check_pmp(purpose, addr, pmpcfg, pmp_r, pmp_w, pmp_x);
                            }
                        }
                        _ => panic!("pmp_a does not matched"),
                    }
                }

                Err(match purpose {
                    TransFor::Fetch | TransFor::Deleg => TrapCause::InstPageFault,
                    TransFor::Load => TrapCause::LoadPageFault,
                    TransFor::StoreAMO => TrapCause::StoreAMOPageFault,
                })
            }
        }
    }
}
