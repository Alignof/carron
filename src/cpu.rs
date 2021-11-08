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

    pub fn trans_addr(&mut self, addr: i32) -> usize {
        match self.mmu.trans_addr(addr as usize, 
                                  self.csrs.read(CSRname::satp.wrap()), 
                                  &self.bus.dram, &self.priv_lv) {
            Ok(addr) => addr,
            Err(()) => 0,
        }
    }
}

