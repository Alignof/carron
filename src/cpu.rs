pub mod fetch;
pub mod decode;
pub mod execution;
pub mod csr;
mod reg;
mod mmu;
mod instruction;

use std::collections::HashSet;
use crate::bus;
use crate::elfload;
use csr::{CSRname, Xstatus};

#[derive(Copy, Clone, Debug)]
pub enum TrapCause {
    IllegalInst = 2,
    Breakpoint = 3,
    UmodeEcall = 8,
    SmodeEcall = 9,
    MmodeEcall = 11,
    InstPageFault = 12,
    LoadPageFault = 13,
    StoreAMOPageFault = 15,
    MachineSoftwareInterrupt = (1 << 31) + 3,
    SupervisorSoftwareInterrupt = (1 << 31) + 1,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PrivilegedLevel {
    User = 0b00,
    Supervisor = 0b01,
    Reserved = 0b10,
    Machine = 0b11,
}

#[derive(Debug, PartialEq)]
pub enum TransFor {
    Fetch,
    Load,
    StoreAMO,
    Deleg,
}

pub struct CPU {
    pub pc: u32,
        bus: bus::Bus,
    pub regs: reg::Register,
        csrs: csr::CSRs,
        mmu: mmu::MMU,
    pub reservation_set: HashSet<(usize, i32)>,
    pub priv_lv: PrivilegedLevel,
}

impl CPU {
    pub fn new(loader: elfload::ElfLoader, pk_load: Option<elfload::ElfLoader>, pc_from_cli: Option<u32>) -> CPU {
        // initialize bus and get the entry point
        let (init_pc, bus) = bus::Bus::new(loader, pk_load);

        CPU {
            pc: pc_from_cli.unwrap_or(init_pc),
            bus,
            regs: reg::Register::new(),
            csrs: csr::CSRs::new().init(),
            mmu: mmu::MMU::new(),
            reservation_set: HashSet::new(),
            priv_lv: PrivilegedLevel::Machine, 
        }
    }

    pub fn add2pc(&mut self, addval: i32) {
        self.pc += addval as u32;
    }

    pub fn update_pc(&mut self, newpc: i32) {
        self.pc = newpc as u32;
    }

    pub fn check_interrupt(&self) -> Result<(), (Option<i32>, TrapCause, String)> {
        const MSIP: u32 = 3;
        const SSIP: u32 = 1;
        let mie = self.csrs.read(CSRname::mie.wrap()).unwrap();
        let mip = self.csrs.read(CSRname::mip.wrap()).unwrap();
        let mideleg = self.csrs.read(CSRname::mideleg.wrap()).unwrap();
        let is_interrupt_enabled = |bit: u32| {
            (mie >> bit) & 0b1 == 1 && (mip >> bit) & 0b1 == 1 && (mideleg >> bit) & 0b1 == 0
        };

        match self.priv_lv {
            PrivilegedLevel::Machine => {
                if self.csrs.read_xstatus(PrivilegedLevel::Machine, Xstatus::MIE) == 1 {
                    if is_interrupt_enabled(MSIP) {
                        return Err((
                            None,
                            TrapCause::MachineSoftwareInterrupt,
                            "machine software interrupt".to_string()
                        ));
                    }
                    if is_interrupt_enabled(SSIP) {
                        return Err((
                            None,
                            TrapCause::SupervisorSoftwareInterrupt,
                            "supervisor software interrupt".to_string()
                        ));
                    }
                }
            },
            PrivilegedLevel::Supervisor => {
                if is_interrupt_enabled(MSIP) {
                    return Err((
                        None,
                        TrapCause::MachineSoftwareInterrupt,
                        "machine software interrupt".to_string()
                    ));
                }
                if self.csrs.read_xstatus(PrivilegedLevel::Supervisor, Xstatus::MIE) == 1 && is_interrupt_enabled(SSIP) {
                    return Err((
                        None,
                        TrapCause::SupervisorSoftwareInterrupt,
                        "supervisor software interrupt".to_string()
                    ));
                }
            },
            _ => (),
        }

        Ok(())
    }

    fn interrupt(&mut self, tval_addr: i32, cause_of_trap: TrapCause) {
        self.csrs.write(CSRname::mcause.wrap(), cause_of_trap as i32);
        self.csrs.write(CSRname::mepc.wrap(), self.pc as i32);

        // check Machine Trap Delegation Registers
        let mcause = self.csrs.read(CSRname::mcause.wrap()).unwrap();
        let mideleg = self.csrs.read(CSRname::mideleg.wrap()).unwrap();
        if self.priv_lv != PrivilegedLevel::Machine && (mideleg & 1 << mcause) != 0 {
            dbg!("delegated");
            self.csrs.write(CSRname::scause.wrap(), cause_of_trap as i32);
            self.csrs.write(CSRname::sepc.wrap(), self.pc as i32);
            self.csrs.write(CSRname::stval.wrap(), tval_addr);
            self.priv_lv = PrivilegedLevel::Supervisor;

            let new_pc = self.csrs.read(CSRname::stvec.wrap()).unwrap() as i32;
            self.update_pc(new_pc as i32);
        } else {
            self.csrs.write(CSRname::mtval.wrap(), tval_addr);
            self.csrs.write_xstatus( // sstatus.MPIE = sstatus.MIE
                PrivilegedLevel::Machine,
                Xstatus::MPIE,
                self.csrs.read_xstatus(PrivilegedLevel::Machine, Xstatus::MIE)
            );
            self.csrs.write_xstatus(PrivilegedLevel::Machine, Xstatus::MIE, 0b0); // msatus.MIE = 0
            self.csrs.write_xstatus(PrivilegedLevel::Machine, Xstatus::MPP, self.priv_lv as u32); // set prev_priv to MPP
            self.priv_lv = PrivilegedLevel::Machine;

            let mtvec = self.csrs.read(CSRname::mtvec.wrap()).unwrap() as i32;
            let new_pc = if mtvec & 0b1 == 1 {
                (mtvec - 1) + 4 * cause_of_trap as i32
            } else {
                mtvec
            };
            self.update_pc(new_pc as i32);
        }
    }

    pub fn exception(&mut self, tval_addr: i32, cause_of_trap: TrapCause) {
        self.csrs.write(CSRname::mcause.wrap(), cause_of_trap as i32);
        self.csrs.write(CSRname::mepc.wrap(), self.pc as i32);

        // check Machine Trap Delegation Registers
        let mcause = self.csrs.read(CSRname::mcause.wrap()).unwrap();
        let medeleg = self.csrs.read(CSRname::medeleg.wrap()).unwrap();
        if self.priv_lv != PrivilegedLevel::Machine && (medeleg & 1 << mcause) != 0 {
            // https://msyksphinz.hatenablog.com/entry/2018/04/03/040000
            dbg!("delegated");
            self.csrs.write(CSRname::scause.wrap(), cause_of_trap as i32);
            self.csrs.write(CSRname::sepc.wrap(), self.pc as i32);
            self.csrs.write(CSRname::stval.wrap(), tval_addr);
            self.priv_lv = PrivilegedLevel::Supervisor;

            let new_pc = self.csrs.read(CSRname::stvec.wrap()).unwrap() as i32;
            self.update_pc(new_pc as i32);
        } else {
            self.csrs.write(CSRname::mtval.wrap(), tval_addr);
            self.csrs.write_xstatus( // sstatus.MPIE = sstatus.MIE
                PrivilegedLevel::Machine,
                Xstatus::MPIE,
                self.csrs.read_xstatus(PrivilegedLevel::Machine, Xstatus::MIE)
            );
            self.csrs.write_xstatus(PrivilegedLevel::Machine, Xstatus::MIE, 0b0); // msatus.MIE = 0
            self.csrs.write_xstatus(PrivilegedLevel::Machine, Xstatus::MPP, self.priv_lv as u32); // set prev_priv to MPP
            self.priv_lv = PrivilegedLevel::Machine;

            let new_pc = self.csrs.read(CSRname::mtvec.wrap()).unwrap() as i32;
            self.update_pc(new_pc as i32);
        }
    }

    pub fn trap(&mut self, tval_addr: i32, cause_of_trap: TrapCause) {
        match cause_of_trap {
            TrapCause::IllegalInst |
            TrapCause::Breakpoint |
            TrapCause::UmodeEcall |
            TrapCause::SmodeEcall |
            TrapCause::MmodeEcall |
            TrapCause::InstPageFault |
            TrapCause::LoadPageFault |
            TrapCause::StoreAMOPageFault => {
                self.exception(tval_addr, cause_of_trap);
            },
            TrapCause::MachineSoftwareInterrupt |
            TrapCause::SupervisorSoftwareInterrupt => {
                self.interrupt(tval_addr, cause_of_trap);
            },
        }

        eprintln!("new pc:0x{:x}", self.pc);
    }

    pub fn trans_addr(&mut self, purpose: TransFor, addr: i32) -> Result<u32, (Option<i32>, TrapCause, String)> {
        let addr = self.check_breakpoint(&purpose, addr as u32)?;
        let mut trans_priv = self.priv_lv;

        if (purpose == TransFor::Load || purpose == TransFor::StoreAMO) &&
           self.csrs.read_xstatus(PrivilegedLevel::Machine, Xstatus::MPRV) == 1 {
                trans_priv = match self.csrs.read_xstatus(PrivilegedLevel::Machine, Xstatus::MPP) {
                    0b00 => PrivilegedLevel::User,
                    0b01 => PrivilegedLevel::Supervisor,
                    0b10 => panic!("PrivilegedLevel 0x3 is Reserved."),
                    0b11 => PrivilegedLevel::Machine,
                    _ => panic!("invalid PrivilegedLevel"),
               }
        }

        match self.mmu.trans_addr(
            purpose, addr, &self.csrs, &self.bus.dram, trans_priv) {

            Ok(addr) => Ok(addr),
            Err(cause) => {
                dbg!(cause);
                Err((Some(addr), cause, format!("address transration failed: {:?}", cause)))
            },
        }
    }
}

