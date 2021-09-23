pub mod dram;
pub mod mmu;

use dram::Dram;
use mmu::MMU;
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

