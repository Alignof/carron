pub mod fetch;
pub mod decode;
pub mod execution;
mod reg;
mod mmu;
mod csr;
mod instruction;

use crate::bus;
use crate::elfload;

pub enum PrivilegedLevel {
    User = 0b00,
    Supervisor = 0b01,
    Reserved = 0b10,
    Machine = 0b11,
}

pub struct CPU<'a> {
    pub pc: usize,
    pub regs: reg::Register,
        csrs: csr::CSRs,
        mmu: mmu::MMU<'a>,
        bus: bus::Bus,
    pub priv_lv: PrivilegedLevel,
}

impl CPU<'_> {
    pub fn new(entry_address: usize, loader: elfload::ElfLoader) -> CPU<'static> {
        let csrs = csr::CSRs::new();
        CPU {
            pc: entry_address,
            regs: reg::Register::new(),
            csrs: csrs,
            mmu: mmu::MMU::new(&csrs),
            bus: bus::Bus::new(loader),
            priv_lv: PrivilegedLevel::Machine, 
        }
    }

    pub fn add2pc(&mut self, addval: i32) {
        self.pc = (self.pc as i32 + addval) as usize;
    }

    pub fn update_pc(&mut self, newval: i32) {
        self.pc = newval as usize;
    }
}

