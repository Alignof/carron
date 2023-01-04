mod a_extension;
mod base_i;
mod m_extension;
mod priv_extension;
mod zicsr_extension;

use crate::cpu::instruction::{Extensions, Instruction};
use crate::cpu::{Cpu, TrapCause};

pub fn exe_inst(inst: &Instruction, cpu: &mut Cpu) -> Result<(), (Option<u64>, TrapCause, String)> {
    const INST_SIZE: u64 = 4;

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
