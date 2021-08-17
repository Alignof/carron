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

