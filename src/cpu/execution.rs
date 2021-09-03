mod exe_inst_16;
mod exe_inst_32;

use dbg_hex::dbg_hex;
use super::CPU;
use super::instruction::Instruction;
use exe_inst_16::exe_cinst;
use exe_inst_32::exe_inst;

pub trait Execution {
    fn execution(&self, cpu: &mut CPU);
}

impl Execution for Instruction {
    fn execution(&self, cpu: &mut CPU) {
        dbg_hex!(cpu.pc);
        dbg!(self);

        if self.is_compressed {
            exe_cinst(self, cpu);
        } else {
            exe_inst(self, cpu);
        }

        cpu.show_regs();
        if cpu.pc > 0x142c { panic!("out of range: 0x{:x}", cpu.pc); }
    }
}
