use crate::cpu::CPU;
use crate::cpu::instruction::Instruction;

pub fn exec(inst: &Instruction, _cpu: &mut CPU) {
    match inst.opc {
        _ => panic!("M extension is not implemented."),
    }
}

