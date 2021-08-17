pub mod dram;
use dram::Dram;

pub struct Bus {
    pub dram: dram::Dram,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            dram: Dram::new(),
        }
    }
}


