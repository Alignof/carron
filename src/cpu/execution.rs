mod exe_inst_16;
mod exe_inst_32;

use memmap::Mmap;
use super::CPU;
use super::instruction::Instruction;
use exe_inst_16::exe_cinst;
use exe_inst_32::exe_inst;

pub trait Execution {
    fn execution(&self, cpu: &mut CPU, dram: &mut Vec<u8>);
}

impl Execution for Instruction {
    fn execution(&self, cpu: &mut CPU, dram: &mut Vec<u8>) {
        if self.is_compressed {
            exe_cinst(self, cpu, dram);
        } else {
            exe_inst(self, cpu, dram);
        }
    }
}
