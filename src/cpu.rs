pub mod decode;
pub mod execution;
mod instruction;

use std::ops::Deref;
use crate::bus::dram;
use crate::bus::dram::Dram;

pub struct CPU {
    pub pc: usize,
    pub reg: [i32; 32],
}

impl CPU {
    pub fn new(entry_address: usize) -> CPU {
        CPU {
            pc: entry_address,
            reg: [0; 32],
        }
    }
}

pub struct InstWrapper {
    pub raw_inst: Box<dyn Decode>,
}

impl Deref for InstWrapper {
    type Target = Box<dyn Decode>;

    fn deref(&self) -> &Self::Target {
        &self.raw_inst
    }
}

pub fn fetch(dram: &dram::Dram, index_pc: usize) -> InstWrapper {
    let is_cinst: bool = self.bus.dram.raw_byte(self.cpu.pc) & 0x3 != 0x3;

    if is_cinst {
        let new_inst: u16 = 
            (Dram::raw_byte(dram, index_pc + 1) as u16) <<  8 |
            (Dram::raw_byte(dram, index_pc + 0) as u16)
        InstWrapper { raw_inst: Box::new(new_inst) }
    } else {
        let new_inst: u32 =
            (Dram::raw_byte(dram, index_pc + 3) as u32) << 24 |
            (Dram::raw_byte(dram, index_pc + 2) as u32) << 16 |
            (Dram::raw_byte(dram, index_pc + 1) as u32) <<  8 |
            (Dram::raw_byte(dram, index_pc + 0) as u32)
        InstWrapper { raw_inst: Box::new(new_inst) }
    }
}
