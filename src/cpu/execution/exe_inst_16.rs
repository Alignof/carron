use crate::cpu::CPU;
use crate::cpu::instruction::{Instruction, OpecodeKind};

pub fn exe_cinst(inst: &Instruction, cpu: &mut CPU) {
    use OpecodeKind::*;
    match inst.opc {
        OP_C_ADDI4SPN => {},
        OP_C_FLD      => {},
        OP_C_LW       => {},
        OP_C_FLW      => {},
        OP_C_FSD      => {},
        OP_C_SW       => {},
        OP_C_FSW      => {},
        OP_C_NOP      => {},
        OP_C_ADDI     => {},
        OP_C_JAL      => {},
        OP_C_LI       => {},
        OP_C_ADDI16SP => {},
        OP_C_LUI      => {},
        OP_C_SRLI     => {},
        OP_C_SRAI     => {},
        OP_C_ANDI     => {},
        OP_C_SUB      => {},
        OP_C_XOR      => {},
        OP_C_OR       => {},
        OP_C_AND      => {},
        OP_C_J        => {},
        OP_C_BEQZ     => {},
        OP_C_BNEZ     => {},
        OP_C_SLLI     => {},
        OP_C_FLDSP    => {},
        OP_C_LWSP     => {},
        OP_C_FLWSP    => {},
        OP_C_JR       => {},
        OP_C_MV       => {},
        OP_C_EBREAK   => {},
        OP_C_JALR     => {},
        OP_C_ADD      => {
            cpu.reg[inst.rd.unwrap() as usize] =
                cpu.reg[inst.rs1.unwrap() as usize] + cpu.reg[inst.rs2.unwrap() as usize];
        },
        OP_C_FSDSP    => {},
        OP_C_SWSP     => {},
        OP_C_FSWSP    => {},
        _             => panic!("not a compressed Instruction"),
    }

    cpu.pc += 2;
}
