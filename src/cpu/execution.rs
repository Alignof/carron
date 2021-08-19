mod exe_inst_16;
mod exe_inst_32;

use super::CPU;
use super::instruction::Instruction;
use exe_inst_16::exe_cinst;
use exe_inst_32::exe_inst;
use crate::bus::dram::Dram;

pub trait Execution {
    fn execution(&self, cpu: &mut CPU, dram: &mut Dram);
}

impl Execution for Instruction {
    fn execution(&self, cpu: &mut CPU, dram: &mut Dram) {
        dbg!(self);
        if self.is_compressed {
            exe_cinst(self, cpu, dram);
        } else {
            exe_inst(self, cpu, dram);
        }
    }
}
