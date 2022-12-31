mod c_extension;
use crate::cpu::instruction::Instruction;
use crate::cpu::{TrapCause, CPU};

pub fn exe_cinst(
    inst: &Instruction,
    cpu: &mut CPU,
) -> Result<(), (Option<u32>, TrapCause, String)> {
    const INST_SIZE: u32 = 2;

    // store previous program counter for excluding branch case
    let prev_pc = cpu.pc;

    c_extension::exec(inst, cpu)?;

    // add the program counter when it isn't a branch instruction
    if cpu.pc == prev_pc {
        cpu.pc += INST_SIZE;
    }

    Ok(())
}
