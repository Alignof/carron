mod exe_inst_16;
mod exe_inst_32;

use super::CPU;
use super::instruction::Instruction;
use exe_inst_16::exe_cinst;
use exe_inst_32::exe_inst;

pub trait Execution {
    fn execution(&self, cpu: &CPU);
}

impl Execution for Instruction {
    fn execution(&self, cpu: &CPU) {
        if self.is_compressed {
            exe_cinst(self, cpu);
        } else {
            exe_inst(self, cpu);
        }
    }
}
