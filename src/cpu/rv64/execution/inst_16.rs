mod c_extension;
use super::Cpu64;
use crate::cpu::instruction::Instruction;
use crate::cpu::TrapCause;

pub fn exe_cinst(
    inst: &Instruction,
    cpu: &mut Cpu64,
) -> Result<(), (Option<u64>, TrapCause, String)> {
    const INST_SIZE: u64 = 2;

    // store previous program counter for excluding branch case
    let prev_pc = cpu.pc;

    c_extension::exec(inst, cpu)?;

    // add the program counter when it isn't a branch instruction
    if cpu.pc == prev_pc {
        cpu.pc += INST_SIZE;
    }

    Ok(())
}
