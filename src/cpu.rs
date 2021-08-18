mod instruction;
pub mod decode;
pub mod execution;

use bus::dram;
use either::*;

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

pub fn fetch(dram: dram::Dram, index_pc: &mut usize) -> either<u16, u32> {
    let is_cinst: bool = |index_pc: usize| {
        Dram::get_byte(index_pc) & 0x3 != 0x3
    };

    if is_cinst(index_pc) {
        (mmap[index + 1] as u16) << 8 |
        (mmap[index + 0] as u16)
    } else {
        (mmap[index + 3] as u32) << 24 |
        (mmap[index + 2] as u32) << 16 |
        (mmap[index + 1] as u32) <<  8 |
        (mmap[index + 0] as u32)
    }
}
