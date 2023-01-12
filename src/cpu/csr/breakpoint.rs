use crate::cpu::csr::{CSRname, CSRs};
use crate::cpu::{Cpu, PrivilegedLevel, TransFor, TrapCause};

pub struct Triggers {
    pub tselect: usize,
    pub tdata1: [u64; 8],
    pub tdata2: [u64; 8],
}

impl CSRs {
    pub fn update_triggers(&mut self, dist: usize, src: u64) {
        match dist {
            0x7a0 => {
                // tselect
                let tgr_index = src as usize;
                self.triggers.tselect = tgr_index;
                self.csrs[CSRname::tdata1 as usize] = self.triggers.tdata1[tgr_index];
                self.csrs[CSRname::tdata2 as usize] = self.triggers.tdata2[tgr_index];
            }
            0x7a1 => {
                // tdata1
                self.triggers.tdata1[self.triggers.tselect] = src;
            }
            0x7a2 => {
                // tdata2
                self.triggers.tdata2[self.triggers.tselect] = src;
            }
            _ => (),
        }
    }
}

impl Cpu {
    pub fn check_breakpoint(
        &mut self,
        purpose: TransFor,
        addr: u64,
    ) -> Result<(), (Option<u64>, TrapCause, String)> {
        for trigger_num in 0..self.csrs.triggers.tselect + 1 {
            let tdata1 = self.csrs.triggers.tdata1[trigger_num];
            let trigger_type = tdata1 >> 28 & 0xF;

            match trigger_type {
                0x0 => (),
                0x1 => panic!("SiFive address match trigger is not implemented."),
                0x2 => {
                    let tdata2 = self.csrs.triggers.tdata2[trigger_num];
                    let match_mode = tdata1 >> 7 & 0xF;

                    let mode_m = tdata1 >> 6 & 0x1;
                    let mode_s = tdata1 >> 4 & 0x1;
                    let mode_u = tdata1 >> 3 & 0x1;

                    if self.priv_lv == PrivilegedLevel::Machine && mode_m == 0x0
                        || self.priv_lv == PrivilegedLevel::Supervisor && mode_s == 0x0
                        || self.priv_lv == PrivilegedLevel::User && mode_u == 0x0
                    {
                        return Ok(());
                    }

                    if match_mode != 0x0 {
                        panic!("this match mode is not supported");
                    }

                    match purpose {
                        TransFor::Fetch | TransFor::Deleg => {
                            let fetch_bit = tdata1 >> 2 & 0x1;
                            if addr == tdata2 && fetch_bit == 1 {
                                return Err((
                                    Some(addr),
                                    TrapCause::Breakpoint,
                                    "Breakpoint exception (fetch)".to_string(),
                                ));
                            }
                        }
                        TransFor::Load => {
                            let load_bit = tdata1 & 0x1;
                            if addr == tdata2 && load_bit == 1 {
                                return Err((
                                    Some(addr),
                                    TrapCause::Breakpoint,
                                    "Breakpoint exception (load)".to_string(),
                                ));
                            }
                        }
                        TransFor::StoreAMO => {
                            let store_bit = tdata1 >> 1 & 0x1;
                            if addr == tdata2 && store_bit == 1 {
                                return Err((
                                    Some(addr),
                                    TrapCause::Breakpoint,
                                    "Breakpoint exception (store/AMO)".to_string(),
                                ));
                            }
                        }
                    }
                }
                0x3 => panic!("Instruction count trigger is not implemented."),
                0x4 => panic!("Interrupt trigger is not implemented."),
                0x5 => panic!("Exception trigger is not implemented."),
                _ => panic!("this trigger is not supported: {}", trigger_type),
            }
        }

        Ok(())
    }
}
