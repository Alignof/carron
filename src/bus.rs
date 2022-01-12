pub mod dram;
pub mod mrom;

use crate::elfload;
use dram::Dram;
use mrom::Mrom;

pub struct Bus {
    pub dram: dram::Dram,
    pub mrom: mrom::Mrom,
}

impl Bus {
    pub fn new(loader: elfload::ElfLoader, pk_load: Option<elfload::ElfLoader>) -> Bus {
        Bus {
            // load proxy kernel before user program when it's given
            dram: if let Some(pk_load) = pk_load {
                Dram::new_with_pk(loader, pk_load)
            } else {
                Dram::new(loader)
            },
            mrom: Mrom::new(),
        }
    }

    // get 1 byte
    pub fn raw_byte(&self, addr: u32) -> u8 {
        self.dram.raw_byte(addr)
    }

    // store
    pub fn store8(&mut self, addr: u32, data: i32) {
        self.dram.store8(addr, data)
    }

    pub fn store16(&mut self, addr: u32, data: i32) {
        self.dram.store16(addr, data)
    }

    pub fn store32(&mut self, addr: u32, data: i32) {
        self.dram.store32(addr, data)
    }


    // load
    pub fn load8(&self, addr: u32) -> i32 {
        self.dram.load8(addr)
    }

    pub fn load16(&self, addr: u32) -> i32 {
        self.dram.load16(addr)
    }

    pub fn load32(&self, addr: u32) -> i32 {
        self.dram.load32(addr)
    }

    pub fn load_u8(&self, addr: u32) -> i32 {
        self.dram.load_u8(addr)
    }

    pub fn load_u16(&self, addr: u32) -> i32 {
        self.dram.load_u16(addr)
    }
}

pub trait Device {
    fn addr2index(&self, addr: u32) -> usize;
    fn raw_byte(&self, addr: u32) -> u8;
    fn store8(&mut self, addr: u32, data: i32);
    fn store16(&mut self, addr: u32, data: i32);
    fn store32(&mut self, addr: u32, data: i32);
    fn load8(&self, addr: u32) -> i32;
    fn load16(&self, addr: u32) -> i32;
    fn load32(&self, addr: u32) -> i32;
    fn load_u8(&self, addr: u32) -> i32;
    fn load_u16(&self, addr: u32) -> i32;
}

