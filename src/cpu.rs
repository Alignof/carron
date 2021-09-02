pub mod fetch;
pub mod decode;
pub mod execution;
mod instruction;

use crate::bus;
use crate::elfload;
use crate::bus::Bus;

pub struct CPU {
    pub pc: usize,
        regs: [i32; 32],
        csrs: [u32; 4096],
        bus: bus::Bus,
}

impl CPU {
    pub fn new(entry_address: usize, loader: elfload::ElfLoader) -> CPU {
        CPU {
            pc: entry_address,
            regs: [0; 32],
            csrs: [0; 4096],
            bus: Bus::new(loader),
        }
    }

    pub fn show_regs(&self) {
        println!("=========================================== dump ============================================");
        println!("pc:\t0x{:x}", self.pc);
        for (num, reg) in self.regs.iter().enumerate() {
            print!("reg{}:\t0x{:08x}\t", num, reg);
            if (num + 1) % 4 == 0 { println!() }
        }
        println!("=============================================================================================");
    }
    
    pub fn read_reg(&self, src: Option<usize>) -> i32 {
        let src = src.unwrap();
        if src == 0 {
            0
        } else {
            self.regs[src]
        }
    }

    pub fn write_reg(&mut self, dist: Option<usize>, src: i32) {
        let dist = dist.unwrap();
        if dist != 0 {
            self.regs[dist] = src;
        }
    }

    pub fn read_csr(&self, src: Option<usize>) -> u32 {
        self.csrs[src.unwrap()]
    }

    pub fn write_csr(&mut self, dist: Option<usize>, src: i32) {
        self.csrs[dist.unwrap()] = src as u32;
    }

    pub fn bitset_csr(&mut self, dist: Option<usize>, src: i32) {
        let mask = src as u32;
        if mask != 0 {
            self.csrs[dist.unwrap()] |= mask;
        }
    }

    pub fn bitclr_csr(&mut self, dist: Option<usize>, src: i32) {
        let mask = src as u32;
        if mask != 0 {
            self.csrs[dist.unwrap()] &= !mask;
        }
    }
}

