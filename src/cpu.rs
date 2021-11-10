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

    pub fn exception(&mut self) {
        self.csrs.write(CSRname::mcause.wrap(),
        match self.priv_lv {
            PrivilegedLevel::User => 8,
            PrivilegedLevel::Supervisor => 9,
            _ => panic!("cannot enviroment call in current privileged mode."),
        });
        self.csrs.write(CSRname::mepc.wrap(), self.pc as i32);
        self.csrs.bitclr(CSRname::mstatus.wrap(), 0x3 << 11);
        self.priv_lv = PrivilegedLevel::Machine;
        let new_pc = self.trans_addr(self.csrs.read(CSRname::mtvec.wrap()) as i32).unwrap();
        self.update_pc(new_pc as i32);
    }

    pub fn trans_addr(&mut self, addr: i32) -> Option<usize> {
        match self.mmu.trans_addr(addr as usize, 
                                  self.csrs.read(CSRname::satp.wrap()), 
                                  &self.bus.dram, &self.priv_lv) {
            Ok(addr) => Some(addr),
            Err(()) => {
                //panic!("page fault");
                self.exception();
                None
            },
        }
    }
}

