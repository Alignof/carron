mod inst_16;
mod inst_32;

use super::{Cpu, TrapCause};
use crate::cpu::instruction::{Extensions, Instruction};
use crate::log;

pub trait Execution {
    fn execution(&self, cpu: &mut Cpu) -> Result<(), (Option<u64>, TrapCause, String)>;
}

impl Execution for Instruction {
    fn execution(&self, cpu: &mut Cpu) -> Result<(), (Option<u64>, TrapCause, String)> {
        log::debugln!("{:#?}", self);

        match self.opc_to_extension() {
            Extensions::C => inst_16::exe_cinst(self, cpu)?,
            _ => inst_32::exe_inst(self, cpu)?,
        }

        //cpu.regs.show();
        Ok(())
    }
}
