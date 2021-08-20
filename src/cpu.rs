pub mod decode;
pub mod execution;
mod instruction;

use crate::bus::dram;
use crate::bus::dram::Dram;
use crate::cpu::decode::Decode;

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
    
    pub fn read_reg(&self, dist: Option<usize>) -> i32 {
        let dist = dist.unwrap();
        if dist == 0 {
            0
        } else {
            self.reg[dist]
        }
    }
}

pub fn fetch(dram: &dram::Dram, index_pc: usize) -> Box<dyn Decode> {
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
