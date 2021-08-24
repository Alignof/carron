pub mod decode;
pub mod execution;
mod instruction;

use crate::cpu;
use crate::bus;
use crate::elfload;
use crate::bus::Bus;
use crate::cpu::decode::Decode;

pub struct CPU {
    pub pc: usize,
        reg: [i32; 32],
        csrs: [u32; 4096],
        bus: bus::Bus,
}

impl CPU {
    pub fn new(entry_address: usize, loader: elfload::ElfLoader) -> CPU {
        CPU {
            pc: entry_address,
            reg: [0; 32],
            csrs: [0; 4096],
            bus: Bus::new(loader),
        }
    }
    
    pub fn read_reg(&self, src: Option<usize>) -> i32 {
        let src = src.unwrap();
        if src == 0 {
            0
        } else {
            self.reg[src]
        }
    }

    pub fn write_reg(&mut self, dist: Option<usize>, src: i32) {
        let dist = dist.unwrap();
        if dist != 0 {
            self.reg[dist] = src;
        }
    }

    pub fn read_csr(&self, src: Option<usize>) -> u32 {
        self.csrs[src.unwrap()]
    }

    pub fn write_csr(&mut self, dist: Option<usize>, src: i32) {
        self.csrs[dist.unwrap()] = src as u32;
    }

    pub fn bitset_csr(&mut self, dist: Option<usize>, src: i32) {
        self.csrs[dist.unwrap()] |= src as u32;
    }
}

pub fn fetch(cpu: &cpu::CPU) -> Box<dyn Decode> {
    let dram = &cpu.bus.dram;
    let index_pc : usize = cpu.pc;
    let is_cinst: bool = dram.raw_byte(index_pc) & 0x3 != 0x3;

    if is_cinst {
        let new_inst: u16 = 
            (dram.raw_byte(index_pc + 1) as u16) <<  8 |
            (dram.raw_byte(index_pc + 0) as u16);
        Box::new(new_inst)
    } else {
        let new_inst: u32 =
            (dram.raw_byte(index_pc + 3) as u32) << 24 |
            (dram.raw_byte(index_pc + 2) as u32) << 16 |
            (dram.raw_byte(index_pc + 1) as u32) <<  8 |
            (dram.raw_byte(index_pc + 0) as u32);
        Box::new(new_inst)
    }
}
