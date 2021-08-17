pub mod dram;
use dram::Dram;

pub struct Bus {
    pub dram: dram::Dram,
}

impl Bus {
    pub fn new(new_dram: dram::Dram) -> Bus {
        Bus {
            dram: new_dram,
        }
    }
}


