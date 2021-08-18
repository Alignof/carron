mod instruction;
pub mod decode;
pub mod execution;

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

pub fn fetch(dram: &dram::Dram, index_pc: usize) -> u32 {
    let is_cinst: bool = self.bus.dram.raw_byte(self.cpu.pc) & 0x3 != 0x3;

    (Dram::raw_byte(dram, index_pc + 3) as u32) << 24 |
    (Dram::raw_byte(dram, index_pc + 2) as u32) << 16 |
    (Dram::raw_byte(dram, index_pc + 1) as u32) <<  8 |
    (Dram::raw_byte(dram, index_pc + 0) as u32)
}
