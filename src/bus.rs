pub mod dram;
use dram::Dram;
use crate::elfload;

pub struct Bus {
    pub dram: dram::Dram,
    // todo: add mmu
}

impl Bus {
    pub fn new(loader: elfload::ElfLoader) -> Bus {
        Bus {
            dram: Dram::new(loader),
        }
    }
}

