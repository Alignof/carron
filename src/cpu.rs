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

#[derive(Copy, Clone)]
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
    pub pc: u32,
    pub regs: reg::Register,
        csrs: csr::CSRs,
        bus: bus::Bus,
        mmu: mmu::MMU,
    pub priv_lv: PrivilegedLevel,
}

impl CPU {
    pub fn new(loader: elfload::ElfLoader) -> CPU {
        CPU {
            pc: loader.elf_header.e_entry,
            regs: reg::Register::new(),
            csrs: csr::CSRs::new(),
            bus: bus::Bus::new(loader),
            mmu: mmu::MMU::new(),
            priv_lv: PrivilegedLevel::Machine, 
        }
    }

    pub fn add2pc(&mut self, addval: i32) {
        self.pc = (self.pc as i32 + addval) as u32;
    }

    pub fn update_pc(&mut self, newval: i32) {
        self.pc = newval as u32;
    }

    pub fn exception(&mut self, tval_addr: i32, cause_of_trap: TrapCause) {
        self.csrs.write(CSRname::mcause.wrap(), cause_of_trap as i32);
        self.csrs.write(CSRname::mepc.wrap(), self.pc as i32);
        self.csrs.bitclr(CSRname::mstatus.wrap(), 0x3 << 11);

        // check Machine Trap Delegation Registers
        let mcause = self.csrs.read(CSRname::mcause.wrap());
        let medeleg = self.csrs.read(CSRname::medeleg.wrap());
        if (medeleg & 1 << mcause) == 0 {
            self.csrs.write(CSRname::mtval.wrap(), tval_addr);
            self.priv_lv = PrivilegedLevel::Machine;

            let new_pc = self.trans_addr(self.csrs.read(CSRname::mtvec.wrap()) as i32).unwrap();
            self.update_pc(new_pc as i32);
        } else {
            // https://msyksphinz.hatenablog.com/entry/2018/04/03/040000
            dbg!("delegated");
            self.csrs.write(CSRname::scause.wrap(), cause_of_trap as i32);
            self.csrs.write(CSRname::stval.wrap(), tval_addr);
            self.priv_lv = PrivilegedLevel::Supervisor;

            let new_pc = self.trans_addr(self.csrs.read(CSRname::stvec.wrap()) as i32).unwrap();
            self.update_pc(new_pc as i32);
        }

        println!("new epc:0x{:x}", self.pc);
    }

    pub fn trans_addr(&mut self, addr: i32) -> Option<u32> {
        let base_addr = self.bus.dram.base_addr;
        match self.mmu.trans_addr(addr as u32, 
                                  self.csrs.read(CSRname::satp.wrap()), 
                                  &self.bus.dram, &self.priv_lv) {
            Ok(addr) => {
                Some(addr - base_addr)
            },
            Err(()) => {
                self.exception(addr, TrapCause::InstPageFault);
                None
            },
        }
    }
}

