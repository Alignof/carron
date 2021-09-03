pub mod fetch;
pub mod decode;
pub mod execution;
mod csr;
mod instruction;

use crate::bus;
use crate::elfload;
use crate::bus::Bus;

pub enum PrivilegedLevel {
    User = 0b00,
    Supervisor = 0b01,
    Reserved = 0b10,
    Machine = 0b11,
}

pub struct CPU {
    pub pc: usize,
        regs: [i32; 32],
        csrs: [u32; 4096],
        bus: bus::Bus,
    pub priv_lv: PrivilegedLevel,
}

impl CPU {
    pub fn new(entry_address: usize, loader: elfload::ElfLoader) -> CPU {
        CPU {
            pc: entry_address,
            regs: [0; 32],
            csrs: [0; 4096],
            bus: Bus::new(loader),
            priv_lv: PrivilegedLevel::Machine, 
        }
    }

    pub fn add2pc(&mut self, addval: i32) {
        self.pc = (self.pc as i32 + addval) as usize;
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
}

