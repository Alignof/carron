pub mod dram;
mod mmu;

use std::rc::Rc;
use std::cell::RefCell;
use crate::elfload;
use crate::cpu::csr;
use dram::Dram;

pub struct Bus {
    pub dram: dram::Dram,
        mmu: mmu::MMU,
}

impl Bus {
    pub fn new(loader: elfload::ElfLoader, new_csrs_ref: Rc<RefCell<csr::CSRs>>) -> Bus {
        Bus {
            dram: Dram::new(loader),
            mmu: mmu::MMU::new(new_csrs_ref),
        }
    }

    // get 1 byte
    pub fn raw_byte(&self, addr: usize) -> u8 {
        self.dram.raw_byte(addr)
    }

    // store
    pub fn store8(&mut self, addr: usize, data: i32) {
        self.dram.store8(addr, data)
    }

    pub fn store16(&mut self, addr: usize, data: i32) {
        self.dram.store16(addr, data)
    }

    pub fn store32(&mut self, addr: usize, data: i32) {
        self.dram.store32(addr, data)
    }


    // load
    pub fn load8(&self, addr: usize) -> i32 {
        self.dram.load8(addr)
    }

    pub fn load16(&self, addr: usize) -> i32 {
        self.dram.load16(addr)
    }

    pub fn load32(&self, addr: usize) -> i32 {
        self.dram.load32(addr)
    }

    pub fn load_u8(&self, addr: usize) -> i32 {
        self.dram.load_u8(addr)
    }

    pub fn load_u16(&self, addr: usize) -> i32 {
        self.dram.load_u16(addr)
    }
}

pub trait Device {
    fn raw_byte(&self, addr: usize) -> u8;
    fn store8(&mut self, addr: usize, data: i32);
    fn store16(&mut self, addr: usize, data: i32);
    fn store32(&mut self, addr: usize, data: i32);
    fn load8(&self, addr: usize) -> i32;
    fn load16(&self, addr: usize) -> i32;
    fn load32(&self, addr: usize) -> i32;
    fn load_u8(&self, addr: usize) -> i32;
    fn load_u16(&self, addr: usize) -> i32;
}

