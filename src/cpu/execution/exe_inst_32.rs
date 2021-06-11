use crate::cpu::CPU;
use crate::cpu::instruction::{Instruction, OpecodeKind};

pub fn exe_inst(inst: &Instruction, cpu: &mut CPU) {
    use OpecodeKind::*;
    match inst.opc {
        OP_LUI    => {cpu.pc += 1},
        OP_AUIPC  => {cpu.pc += 1},
        OP_JAL    => {cpu.pc += 1},
        OP_JALR   => {cpu.pc += 1},
        OP_BEQ    => {cpu.pc += 1},
        OP_BNE    => {cpu.pc += 1},
        OP_BLT    => {cpu.pc += 1},
        OP_BGE    => {cpu.pc += 1},
        OP_BLTU   => {cpu.pc += 1},
        OP_BGEU   => {cpu.pc += 1},
        OP_LB     => {cpu.pc += 1},
        OP_LH     => {cpu.pc += 1},
        OP_LW     => {cpu.pc += 1},
        OP_LBU    => {cpu.pc += 1},
        OP_LHU    => {cpu.pc += 1},
        OP_SB     => {cpu.pc += 1},
        OP_SH     => {cpu.pc += 1},
        OP_SW     => {cpu.pc += 1},
        OP_ADDI   => {cpu.pc += 1},
        OP_SLTI   => {cpu.pc += 1},
        OP_SLTIU  => {cpu.pc += 1},
        OP_XORI   => {cpu.pc += 1},
        OP_ORI    => {cpu.pc += 1},
        OP_ANDI   => {cpu.pc += 1},
        OP_SLLI   => {cpu.pc += 1},
        OP_SRLI   => {cpu.pc += 1},
        OP_ADD    => {cpu.pc += 1},
        OP_SUB    => {cpu.pc += 1},
        OP_SLL    => {cpu.pc += 1},
        OP_SLT    => {cpu.pc += 1},
        OP_SLTU   => {cpu.pc += 1},
        OP_XOR    => {cpu.pc += 1},
        OP_SRL    => {cpu.pc += 1},
        OP_SRA    => {cpu.pc += 1},
        OP_OR     => {cpu.pc += 1},
        OP_AND    => {cpu.pc += 1},
        OP_FENCE  => {cpu.pc += 1},
        OP_ECALL  => {cpu.pc += 1},
        OP_EBREAK => {cpu.pc += 1},
        _             => panic!("not a full instruction"),
    }
}
