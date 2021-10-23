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

pub trait Device {
    fn store8(&mut self, addr: usize, data: i32);
    fn store16(&mut self, addr: usize, data: i32);
    fn store32(&mut self, addr: usize, data: i32);
    fn load8(&self, addr: usize) -> i32;
    fn load16(&self, addr: usize) -> i32;
    fn load32(&self, addr: usize) -> i32;
    fn load_u8(&self, addr: usize) -> i32;
    fn load_u16(&self, addr: usize) -> i32;
}

