pub mod dram;

use dram::Dram;
use crate::elfload;

pub struct Bus {
    pub dram: dram::Dram,
}

impl Bus {
    pub fn new(loader: elfload::ElfLoader) -> Bus {
        Bus {
            dram: Dram::new(loader),
        }
    }
}

