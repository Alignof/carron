mod instruction;
pub mod decode;
pub mod execution;

use crate::bus::dram;
use crate::bus::dram::Dram;

pub struct CPU {
    pub pc: u32,
    pub reg: [i32; 32],
}

impl CPU {
    pub fn new(entry_address: u32) -> CPU {
        CPU {
            pc: entry_address as u32,
            reg: [0; 32],
        }
    }
}

pub fn fetch(dram: dram::Dram, index_pc: usize) -> u32 {
    // return instruction data
    (Dram::raw_byte(&dram, index_pc + 4) as u32) << 24 |
    (Dram::raw_byte(&dram, index_pc + 3) as u32) << 16 |
    (Dram::raw_byte(&dram, index_pc + 2) as u32) <<  8 |
    (Dram::raw_byte(&dram, index_pc + 1) as u32)
}

pub fn fetch_compressed(dram: dram::Dram, index_pc: usize) -> u16 {
    // return compressed instruction data
    (Dram::raw_byte(&dram, index_pc + 1) as u16) << 8 |
    (Dram::raw_byte(&dram, index_pc + 0) as u16)
}
