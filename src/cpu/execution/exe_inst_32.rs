use crate::cpu::CPU;
use crate::cpu::instruction::{Instruction, OpecodeKind};

pub fn exe_inst(inst: &Instruction, cpu: &mut CPU) {
    use OpecodeKind::*;

    cpu.pc += 4;
    match inst.opc {
        OP_LUI    => {},
        OP_AUIPC  => {},
        OP_JAL    => {},
        OP_JALR   => {},
        OP_BEQ    => {},
        OP_BNE    => {},
        OP_BLT    => {},
        OP_BGE    => {},
        OP_BLTU   => {},
        OP_BGEU   => {},
        OP_LB     => {},
        OP_LH     => {},
        OP_LW     => {},
        OP_LBU    => {},
        OP_LHU    => {},
        OP_SB     => {},
        OP_SH     => {},
        OP_SW     => {},
        OP_ADDI   => {
            cpu.reg[inst.rd.unwrap() as usize] += inst.rs1.unwrap() as u32;
        },
        OP_SLTI   => {},
        OP_SLTIU  => {},
        OP_XORI   => {},
        OP_ORI    => {},
        OP_ANDI   => {},
        OP_SLLI   => {},
        OP_SRLI   => {},
        OP_ADD    => {
            cpu.reg[inst.rd.unwrap() as usize] += cpu.reg[inst.rs1.unwrap() as usize];
        },
        OP_SUB    => {
            cpu.reg[inst.rd.unwrap() as usize] -= cpu.reg[inst.rs1.unwrap() as usize];
        },
        OP_SLL    => {},
        OP_SLT    => {},
        OP_SLTU   => {},
        OP_XOR    => {},
        OP_SRL    => {},
        OP_SRA    => {},
        OP_OR     => {},
        OP_AND    => {},
        OP_FENCE  => {},
        OP_ECALL  => {},
        OP_EBREAK => {},
        _         => panic!("not a full instruction"),
    }
}
