pub mod dram;

use dram::Dram;
use crate::elfload;

pub struct Bus {
    pub dram: dram::Dram,
}

impl Bus {
    pub fn new(loader: elfload::ElfLoader) -> Bus {
        Bus {
            dram: Dram::new(loader),
        }
    }
}

trait Device {
    // get byte
    pub fn raw_byte(&self, addr: usize) -> u8 {
        self.data[addr]
    }

    // store
    pub fn store8(&mut self, addr: usize, data: i32) {
        self.data[addr + 0] = ((data >> 0) & 0xFF) as u8;
    }

    pub fn store16(&mut self, addr: usize, data: i32) {
        self.data[addr + 1] = ((data >> 8) & 0xFF) as u8;
        self.data[addr + 0] = ((data >> 0) & 0xFF) as u8;
    }

    pub fn store32(&mut self, addr: usize, data: i32) {
        self.data[addr + 3] = ((data >> 24) & 0xFF) as u8;
        self.data[addr + 2] = ((data >> 16) & 0xFF) as u8;
        self.data[addr + 1] = ((data >>  8) & 0xFF) as u8;
        self.data[addr + 0] = ((data >>  0) & 0xFF) as u8;
    }


    // load
    pub fn load8(&self, addr: usize) -> i32 {
        self.data[addr] as i8 as i32
    }

    pub fn load16(&self, addr: usize) -> i32 {
        ((self.data[addr + 1] as u16) << 8 |
         (self.data[addr + 0] as u16)) as i16 as i32
    }

    pub fn load32(&self, addr: usize) -> i32 {
        ((self.data[addr + 3] as u32) << 24 |
         (self.data[addr + 2] as u32) << 16 |
         (self.data[addr + 1] as u32) <<  8 |
         (self.data[addr + 0] as u32)) as i32
    }

    pub fn load_u8(&self, addr: usize) -> i32 {
        self.data[addr] as i32
    }

    pub fn load_u16(&self, addr: usize) -> i32 {
        ((self.data[addr + 1] as u32) << 8 |
         (self.data[addr + 0] as u32)) as i32
    }
}
