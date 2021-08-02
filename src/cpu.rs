mod instruction;
pub mod decode;
pub mod execution;

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

pub fn get_u16(mmap: &[u8], index: usize) -> u16 {
    (mmap[index + 1] as u16) << 8 |
    (mmap[index + 0] as u16)
}

pub fn get_u32(mmap: &[u8], index: usize) -> u32 {
    (mmap[index + 3] as u32) << 24 |
    (mmap[index + 2] as u32) << 16 |
    (mmap[index + 1] as u32) <<  8 |
    (mmap[index + 0] as u32)
}

pub fn is_cinst(mmap: &[u8], index: usize) -> bool {
    mmap[index] & 0x3 != 0x3
}

