mod inst_16;
mod inst_32;

use super::CPU;
use super::instruction::{Instruction, Extensions};
use inst_16::exe_cinst;
use inst_32::exe_inst;

pub trait Execution {
    fn execution(&self, cpu: &mut CPU);
}

impl Execution for Instruction {
    fn execution(&self, cpu: &mut CPU) {
        dbg!(self);

        match self.opc_to_extension() {
            Extensions::C => exe_cinst(self, cpu),
            _ => exe_inst(self, cpu),
        }

        cpu.regs.show();
    }
}
