pub mod dram;
pub mod mrom;
pub mod device_tree;

use crate::elfload;
use dram::Dram;
use mrom::Mrom;

pub struct Bus {
    pub dram: dram::Dram,
    pub mrom: mrom::Mrom,
}

impl Bus {
    pub fn new(loader: elfload::ElfLoader, pk_load: Option<elfload::ElfLoader>) -> (u32, Bus) {
        // load proxy kernel before user program when it's given
        let (entry, dram) = if let Some(ref pk_load) = pk_load {
            Dram::new_with_pk(loader, pk_load)
        } else {
            Dram::new(loader)
        };
        let mut mrom = Mrom::new(entry);

        // create and load DTB
        mrom.load_dtb(dram.base_addr);

        // set initial pc to reset vector if proxy kernel loaded 
        let init_pc = if pk_load.is_some() {
            mrom.base_addr
        } else {
            entry
        };

        // return initial pc and Bus
        (init_pc, Bus{dram, mrom})
    }

    // get 1 byte
    pub fn load_byte(&self, addr: u32) -> u8 {
        if addr < self.dram.base_addr {
            self.mrom.load_byte(addr)
        } else {
            self.dram.load_byte(addr)
        }
    }

    // store
    pub fn store8(&mut self, addr: u32, data: i32) {
        if addr < self.dram.base_addr {
            self.mrom.store8(addr, data)
        } else {
            self.dram.store8(addr, data)
        }
    }

    pub fn store16(&mut self, addr: u32, data: i32) {
        if addr < self.dram.base_addr {
            self.mrom.store16(addr, data)
        } else {
            self.dram.store16(addr, data)
        }
    }

    pub fn store32(&mut self, addr: u32, data: i32) {
        if addr < self.dram.base_addr {
            self.mrom.store32(addr, data)
        } else {
            self.dram.store32(addr, data)
        }
    }


    // load
    pub fn load8(&self, addr: u32) -> i32 {
        if addr < self.dram.base_addr {
            self.mrom.load8(addr)
        } else {
            self.dram.load8(addr)
        }
    }

    pub fn load16(&self, addr: u32) -> i32 {
        if addr < self.dram.base_addr {
            self.mrom.load16(addr)
        } else {
            self.dram.load16(addr)
        }
    }

    pub fn load32(&self, addr: u32) -> i32 {
        if addr < self.dram.base_addr {
            self.mrom.load32(addr)
        } else {
            self.dram.load32(addr)
        }
    }

    pub fn load_u8(&self, addr: u32) -> i32 {
        if addr < self.dram.base_addr {
            self.mrom.load_u8(addr)
        } else {
            self.dram.load_u8(addr)
        }
    }

    pub fn load_u16(&self, addr: u32) -> i32 {
        if addr < self.dram.base_addr {
            self.mrom.load_u16(addr)
        } else {
            self.dram.load_u16(addr)
        }
    }
}

pub trait Device {
    fn store_byte(&self, addr: u32, data: u8);
    fn store8(&mut self, addr: u32, data: i32);
    fn store16(&mut self, addr: u32, data: i32);
    fn store32(&mut self, addr: u32, data: i32);
    fn load_byte(&self, addr: u32) -> u8;
    fn load8(&self, addr: u32) -> i32;
    fn load16(&self, addr: u32) -> i32;
    fn load32(&self, addr: u32) -> i32;
    fn load_u8(&self, addr: u32) -> i32;
    fn load_u16(&self, addr: u32) -> i32;
}

