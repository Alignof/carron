pub mod fetch;
pub mod decode;
pub mod execution;
pub mod csr;
mod reg;
mod mmu;
mod instruction;

use crate::bus;
use crate::elfload;
use csr::CSRname;

pub enum TrapCause {
    UmodeEcall = 8,
    SmodeEcall = 9,
    MmodeEcall = 11,
    InstPageFault = 12,
    LoadPageFault = 13,
}

#[derive(Debug)]
pub enum PrivilegedLevel {
    User = 0b00,
    Supervisor = 0b01,
    Reserved = 0b10,
    Machine = 0b11,
}

pub struct CPU {
    pub pc: usize,
    pub regs: reg::Register,
        csrs: csr::CSRs,
        bus: bus::Bus,
        mmu: mmu::MMU,
    pub priv_lv: PrivilegedLevel,
}

impl CPU {
    pub fn new(loader: elfload::ElfLoader) -> CPU {
        CPU {
            pc: 0,
            regs: reg::Register::new(),
            csrs: csr::CSRs::new(),
            bus: bus::Bus::new(loader),
            mmu: mmu::MMU::new(),
            priv_lv: PrivilegedLevel::Machine, 
        }
    }

    pub fn add2pc(&mut self, addval: i32) {
        self.pc = (self.pc as i32 + addval) as usize;
    }

    pub fn update_pc(&mut self, newval: i32) {
        self.pc = newval as usize;
    }

    pub fn exception(&mut self, cause_of_trap: TrapCause) {
        self.csrs.bitset(CSRname::mcause.wrap(), 1 << (cause_of_trap as i32));
        self.csrs.write(CSRname::mepc.wrap(), self.pc as i32);
        self.csrs.bitclr(CSRname::mstatus.wrap(), 0x3 << 11);
        self.priv_lv = PrivilegedLevel::Machine;

        // check Machine Trap Delegation Registers
        let mcause = self.csrs.read(CSRname::mcause.wrap());
        let medeleg = self.csrs.read(CSRname::medeleg.wrap());
        if (medeleg & mcause) == 0 {
            let new_pc = self.trans_addr(self.csrs.read(CSRname::sepc.wrap()) as i32).unwrap();
            self.update_pc(new_pc as i32);
        } else {
            // https://msyksphinz.hatenablog.com/entry/2018/04/03/040000
            dbg!("delegated");
            self.priv_lv = PrivilegedLevel::Supervisor;
        }

        println!("new epc:0x{:x}", self.pc);
    }

    pub fn trans_addr(&mut self, addr: i32) -> Option<usize> {
        match self.mmu.trans_addr(addr as usize, 
                                  self.csrs.read(CSRname::satp.wrap()), 
                                  &self.bus.dram, &self.priv_lv) {
            Ok(addr) => Some(addr),
            Err(()) => {
                //panic!("page fault");
                self.exception(TrapCause::LoadPageFault);
                None
            },
        }
    }
}

