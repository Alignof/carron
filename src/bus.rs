pub mod mmu;
pub mod dram;

use mmu::MMU;
use dram::Dram;
use crate::elfload;

pub struct Bus {
    pub dram: dram::Dram,
    pub mmu: mmu::MMU,
}

impl Bus {
    pub fn new(loader: elfload::ElfLoader) -> Bus {
        Bus {
            dram: Dram::new(loader),
            mmu: MMU::new(),
        }
    }
}

