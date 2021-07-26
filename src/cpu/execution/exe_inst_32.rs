use crate::cpu::CPU;
use crate::cpu::instruction::{Instruction, OpecodeKind};

pub fn exe_inst(inst: &Instruction, cpu: &mut CPU) {
    use OpecodeKind::*;
    const INST_SIZE: u32 = 4;

    match inst.opc {
        OP_LUI    => {
            cpu.reg[inst.rd.unwrap() as usize] = inst.imm.unwrap() << 12;
        },
        OP_AUIPC  => {
            cpu.pc += (inst.imm.unwrap() << 12) as u32;
        },
        OP_JAL    => {
            cpu.reg[inst.rd.unwrap() as usize] = (cpu.pc + INST_SIZE) as i32; 
            cpu.pc += inst.imm.unwrap() as u32;
        },
        OP_JALR   => {
            cpu.reg[inst.rd.unwrap() as usize] = (cpu.pc + INST_SIZE) as i32; 
            cpu.pc += (cpu.reg[inst.rs1.unwrap() as usize]  + inst.imm.unwrap()) as u32;
        },
        OP_BEQ    => {
            if cpu.reg[inst.rs1.unwrap() as usize] == cpu.reg[inst.rs1.unwrap() as usize] {
                cpu.pc += inst.imm.unwrap() as u32;
            } 
        },
        OP_BNE    => {
            if cpu.reg[inst.rs1.unwrap() as usize] != cpu.reg[inst.rs1.unwrap() as usize] {
                cpu.pc += inst.imm.unwrap() as u32;
            } 
        },
        OP_BLT    => {
            if cpu.reg[inst.rs1.unwrap() as usize] < cpu.reg[inst.rs1.unwrap() as usize] {
                cpu.pc += inst.imm.unwrap() as u32;
            } 
        },
        OP_BGE    => {
            if cpu.reg[inst.rs1.unwrap() as usize] >= cpu.reg[inst.rs1.unwrap() as usize] {
                cpu.pc += inst.imm.unwrap() as u32;
            } 
        },
        OP_BLTU   => {
            if (cpu.reg[inst.rs1.unwrap() as usize] as u32) < (cpu.reg[inst.rs1.unwrap() as usize] as u32) {
                cpu.pc += inst.imm.unwrap() as u32;
            } 
        },
        OP_BGEU   => {
            if (cpu.reg[inst.rs1.unwrap() as usize] as u32) >= (cpu.reg[inst.rs1.unwrap() as usize] as u32) {
                cpu.pc += inst.imm.unwrap() as u32;
            } 
        },
        OP_LB     => {},
        OP_LH     => {},
        OP_LW     => {},
        OP_LBU    => {},
        OP_LHU    => {},
        OP_SB     => {},
        OP_SH     => {},
        OP_SW     => {},
        OP_ADDI   => {
            cpu.reg[inst.rd.unwrap() as usize] += inst.rs1.unwrap() as i32;
        },
        OP_SLTI   => {
            cpu.reg[inst.rd.unwrap() as usize] =
                (cpu.reg[inst.rs1.unwrap() as usize] < inst.imm.unwrap()) as i32;
        },
        OP_SLTIU  => {
            cpu.reg[inst.rd.unwrap() as usize] =
                ((cpu.reg[inst.rs1.unwrap() as usize] as u32) < inst.imm.unwrap() as u32) as i32;
        },
        OP_XORI   => {
            cpu.reg[inst.rd.unwrap() as usize] ^= inst.rs1.unwrap() as i32;
        },
        OP_ORI    => {
            cpu.reg[inst.rd.unwrap() as usize] |= inst.rs1.unwrap() as i32;
        },
        OP_ANDI   => {
            cpu.reg[inst.rd.unwrap() as usize] &= inst.rs1.unwrap() as i32;
        },
        OP_SLLI   => {
            cpu.reg[inst.rd.unwrap() as usize] =
                cpu.reg[inst.rs1.unwrap() as usize] << inst.imm.unwrap() as i32;
        },
        OP_SRLI   => {
            cpu.reg[inst.rd.unwrap() as usize] =
                cpu.reg[inst.rs1.unwrap() as usize] >> inst.imm.unwrap() as i32;
        },
        OP_ADD    => {
            cpu.reg[inst.rd.unwrap() as usize] =
                cpu.reg[inst.rs1.unwrap() as usize] + cpu.reg[inst.rs2.unwrap() as usize];
        },
        OP_SUB    => {
            cpu.reg[inst.rd.unwrap() as usize] =
                cpu.reg[inst.rs1.unwrap() as usize] - cpu.reg[inst.rs2.unwrap() as usize];
        },
        OP_SLL    => {
            cpu.reg[inst.rd.unwrap() as usize] =
                cpu.reg[inst.rs1.unwrap() as usize] << cpu.reg[inst.rs2.unwrap() as usize];
        },
        OP_SLT    => {
            cpu.reg[inst.rd.unwrap() as usize] =
                (cpu.reg[inst.rs1.unwrap() as usize] < cpu.reg[inst.rs2.unwrap() as usize]) as i32;
        },
        OP_SLTU   => {
            cpu.reg[inst.rd.unwrap() as usize] =
                ((cpu.reg[inst.rs1.unwrap() as usize] as u32) < (cpu.reg[inst.rs2.unwrap() as usize] as u32)) as i32;
        },
        OP_XOR    => {
            cpu.reg[inst.rd.unwrap() as usize] =
                cpu.reg[inst.rs1.unwrap() as usize] ^ cpu.reg[inst.rs2.unwrap() as usize];
        },
        OP_SRL    => {
            cpu.reg[inst.rd.unwrap() as usize] =
                ((cpu.reg[inst.rs1.unwrap() as usize] as u32)  >> cpu.reg[inst.rs2.unwrap() as usize]) as i32;
        },
        OP_SRA    => {
            cpu.reg[inst.rd.unwrap() as usize] =
                (cpu.reg[inst.rs1.unwrap() as usize] as i32)  >> cpu.reg[inst.rs2.unwrap() as usize];
        },
        OP_OR     => {
            cpu.reg[inst.rd.unwrap() as usize] =
                cpu.reg[inst.rs1.unwrap() as usize] | cpu.reg[inst.rs2.unwrap() as usize];
        },
        OP_AND    => {
            cpu.reg[inst.rd.unwrap() as usize] =
                cpu.reg[inst.rs1.unwrap() as usize] & cpu.reg[inst.rs2.unwrap() as usize];
        },
        OP_FENCE  => {},
        OP_ECALL  => {},
        OP_EBREAK => {},
        _         => panic!("not a full instruction"),
    }

    cpu.pc += 4;
}
