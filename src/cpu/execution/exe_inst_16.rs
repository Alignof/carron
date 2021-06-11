use crate::cpu::CPU;
use crate::cpu::instruction::{Instruction, OpecodeKind};

pub fn exe_cinst(inst: &Instruction, cpu: &mut CPU) {
    use OpecodeKind::*;
    match inst.opc {
        OP_C_ADDI4SPN => {cpu.pc += 1},
        OP_C_FLD      => {cpu.pc += 1},
        OP_C_LW       => {cpu.pc += 1},
        OP_C_FLW      => {cpu.pc += 1},
        OP_C_FSD      => {cpu.pc += 1},
        OP_C_SW       => {cpu.pc += 1},
        OP_C_FSW      => {cpu.pc += 1},
        OP_C_NOP      => {cpu.pc += 1},
        OP_C_ADDI     => {cpu.pc += 1},
        OP_C_JAL      => {cpu.pc += 1},
        OP_C_LI       => {cpu.pc += 1},
        OP_C_ADDI16SP => {cpu.pc += 1},
        OP_C_LUI      => {cpu.pc += 1},
        OP_C_SRLI     => {cpu.pc += 1},
        OP_C_SRAI     => {cpu.pc += 1},
        OP_C_ANDI     => {cpu.pc += 1},
        OP_C_SUB      => {cpu.pc += 1},
        OP_C_XOR      => {cpu.pc += 1},
        OP_C_OR       => {cpu.pc += 1},
        OP_C_AND      => {cpu.pc += 1},
        OP_C_J        => {cpu.pc += 1},
        OP_C_BEQZ     => {cpu.pc += 1},
        OP_C_BNEZ     => {cpu.pc += 1},
        OP_C_SLLI     => {cpu.pc += 1},
        OP_C_FLDSP    => {cpu.pc += 1},
        OP_C_LWSP     => {cpu.pc += 1},
        OP_C_FLWSP    => {cpu.pc += 1},
        OP_C_JR       => {cpu.pc += 1},
        OP_C_MV       => {cpu.pc += 1},
        OP_C_EBREAK   => {cpu.pc += 1},
        OP_C_JALR     => {cpu.pc += 1},
        OP_C_ADD      => {cpu.pc += 1},
        OP_C_FSDSP    => {cpu.pc += 1},
        OP_C_SWSP     => {cpu.pc += 1},
        OP_C_FSWSP    => {cpu.pc += 1},
        _             => panic!("not a compressed Instruction"),
    }
}
