mod base_i;
mod a_extension;
mod m_extension;
mod priv_extension;
mod zicsr_extension;

use crate::cpu::{CPU, TrapCause};
use crate::cpu::instruction::{Instruction, Extensions};

pub fn exe_inst(inst: &Instruction, cpu: &mut CPU) -> Result<(), (Option<u32>, TrapCause, String)> {
    const INST_SIZE: u32 = 4;

    // store previous program counter for excluding branch case
    let prev_pc = cpu.pc;

    match inst.opc_to_extension() {
        Extensions::BaseI => base_i::exec(inst, cpu)?,
        Extensions::A => a_extension::exec(inst, cpu)?,
        Extensions::M => m_extension::exec(inst, cpu)?,
        Extensions::Priv => priv_extension::exec(inst, cpu)?,
        Extensions::Zicsr => zicsr_extension::exec(inst, cpu)?,
        _ => panic!("not a full size instruction."),
    }

    // add the program counter when it isn't a branch instruction
    if cpu.pc == prev_pc {
        cpu.add2pc(INST_SIZE);
    }

    Ok(())
}
