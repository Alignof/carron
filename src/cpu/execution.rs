mod inst_16;
mod inst_32;

use super::{CPU, TrapCause};
use super::instruction::{Instruction, Extensions};

pub trait Execution {
    fn execution(&self, cpu: &mut CPU) -> Result<(), (Option<i32>, TrapCause, String)>;
}

impl Execution for Instruction {
    fn execution(&self, cpu: &mut CPU) -> Result<(), (Option<i32>, TrapCause, String)>{
        dbg!(self);

        match self.opc_to_extension() {
            Extensions::C => inst_16::exe_cinst(self, cpu)?,
            _ => inst_32::exe_inst(self, cpu)?,
        }

        cpu.regs.show();
        Ok(())
    }
}
