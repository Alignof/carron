pub mod decode;
pub mod execution;
mod instruction;

use crate::cpu;
use crate::bus;
use crate::elfload;
use crate::bus::{Bus, dram};
use crate::bus::dram::Dram;
use crate::cpu::decode::Decode;

pub struct CPU {
    pub pc: usize,
        reg: [i32; 32],
        csrs: [i32; 4096],
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
}

pub fn fetch(cpu: &cpu::CPU) -> Box<dyn Decode> {
    let dram: &dram::Dram = &cpu.bus.dram;
    let index_pc : usize = cpu.pc;
    let is_cinst: bool = Dram::raw_byte(dram, index_pc) & 0x3 != 0x3;

    if is_cinst {
        let new_inst: u16 = 
            (Dram::raw_byte(dram, index_pc + 1) as u16) <<  8 |
            (Dram::raw_byte(dram, index_pc + 0) as u16);
        Box::new(new_inst)
    } else {
        let new_inst: u32 =
            (Dram::raw_byte(dram, index_pc + 3) as u32) << 24 |
            (Dram::raw_byte(dram, index_pc + 2) as u32) << 16 |
            (Dram::raw_byte(dram, index_pc + 1) as u32) <<  8 |
            (Dram::raw_byte(dram, index_pc + 0) as u32);
        Box::new(new_inst)
    }
}
