pub mod fetch;
pub mod decode;
pub mod execution;
pub mod csr;
mod reg;
mod mmu;
mod breakpoint;
mod instruction;

use std::collections::HashSet;
use crate::bus;
use crate::elfload;
use csr::CSRname;

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
}

#[derive(Debug, PartialEq)]
pub enum PrivilegedLevel {
    User = 0b00,
    Supervisor = 0b01,
    Reserved = 0b10,
    Machine = 0b11,
}

#[derive(Debug)]
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
    pub fn new(loader: elfload::ElfLoader, pk_load: Option<elfload::ElfLoader>, 
               pc_from_cli: Option<u32>) -> CPU {
        // initialize bus and get the entry point
        let (init_pc, bus) = bus::Bus::new(loader, pk_load);

        CPU {
            pc: match pc_from_cli {
                Some(cli_pc) => cli_pc,
                None => init_pc,
            },
            bus,
            regs: reg::Register::new(),
            csrs: csr::CSRs::new().init(),
            mmu: mmu::MMU::new(),
            reservation_set: HashSet::new(),
            priv_lv: PrivilegedLevel::Machine, 
        }
    }

    pub fn add2pc(&mut self, addval: i32) {
        self.pc = (self.pc as i32 + addval) as u32;
    }

    pub fn update_pc(&mut self, newpc: i32) {
        self.pc = newpc as u32;
    }

    pub fn exception(&mut self, tval_addr: i32, cause_of_trap: TrapCause) {
        self.csrs.write(CSRname::mcause.wrap(), cause_of_trap as i32);
        self.csrs.write(CSRname::mepc.wrap(), self.pc as i32);

        // check Machine Trap Delegation Registers
        let mcause = self.csrs.read(CSRname::mcause.wrap());
        let medeleg = self.csrs.read(CSRname::medeleg.wrap());
        if self.priv_lv != PrivilegedLevel::Machine && (medeleg & 1 << mcause) != 0 {
            // https://msyksphinz.hatenablog.com/entry/2018/04/03/040000
            dbg!("delegated");
            self.csrs.write(CSRname::scause.wrap(), cause_of_trap as i32);
            self.csrs.write(CSRname::sepc.wrap(), self.pc as i32);
            self.csrs.write(CSRname::stval.wrap(), tval_addr);
            self.priv_lv = PrivilegedLevel::Supervisor;

            let new_pc = self.csrs.read(CSRname::stvec.wrap()) as i32;
            self.update_pc(new_pc as i32);
        } else {
            self.csrs.write(CSRname::mtval.wrap(), tval_addr);
            self.priv_lv = PrivilegedLevel::Machine;

            let new_pc = self.csrs.read(CSRname::mtvec.wrap()) as i32;
            self.update_pc(new_pc as i32);
        }

        println!("new pc:0x{:x}", self.pc);
    }

    pub fn trans_addr(&mut self, purpose: TransFor, addr: i32) -> Result<u32, String> {
        let addr = self.check_breakpoint(&purpose, addr as u32)?;

        match self.mmu.trans_addr(
            purpose, addr, &self.csrs, &self.bus.dram, &self.priv_lv) {

            Ok(addr) => {
                Ok(addr)
            },
            Err(cause) => {
                dbg!(cause);
                self.exception(addr as i32, cause);
                Err(format!("address transration failed: {:?}", cause))
            },
        }
    }
}

