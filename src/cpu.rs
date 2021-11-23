pub mod fetch;
pub mod decode;
pub mod execution;
pub mod csr;
mod reg;
mod mmu;
mod instruction;

use crate::bus;
use crate::elfload;
use csr::{CSRname, Xstatus};

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

    pub fn exception(&mut self, cause_of_trap: TrapCause) {
        self.csrs.bitset(CSRname::mcause.wrap(), 1 << (cause_of_trap as i32));

        // check Machine Trap Delegation Registers
        let mcause = self.csrs.read(CSRname::mcause.wrap());
        let medeleg = self.csrs.read(CSRname::medeleg.wrap());
        if (medeleg & mcause) == 0 {
            self.csrs.write(CSRname::mepc.wrap(), self.pc as i32);
            self.csrs.bitclr(CSRname::mstatus.wrap(), 0x3 << 11);
            self.priv_lv = PrivilegedLevel::Machine;

            let new_pc = self.trans_addr(
                self.csrs.read(
                    match self.csrs.read_xstatus(&self.priv_lv, Xstatus::MPP) {
                        0b00 => CSRname::mepc.wrap(),
                        0b01 => CSRname::sepc.wrap(),
                        _ => panic!("PrivilegedLevel 0x3 is Reserved."),
                    }
                ) as i32
            ).unwrap();
            self.update_pc(new_pc as i32);
        } else {
            // https://msyksphinz.hatenablog.com/entry/2018/04/03/040000
            dbg!("delegated");
            self.csrs.bitset(CSRname::scause.wrap(), 1 << (TrapCause::InstPageFault as i32));
            self.csrs.bitclr(CSRname::sstatus.wrap(), 0x3 << 11);
            self.priv_lv = PrivilegedLevel::Supervisor;
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
                //panic!("page fault");
                self.exception(TrapCause::InstPageFault);
                None
            },
        }
    }
}

