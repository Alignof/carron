use crate::cpu::CPU;
use crate::cpu::instruction::{Instruction, OpecodeKind};
use crate::bus::dram::Dram;

pub fn exe_cinst(inst: &Instruction, cpu: &mut CPU, dram: &mut Dram) {
    use OpecodeKind::*;
    const INST_SIZE: u32 = 2;
    const REG_SP: usize = 2 as usize;
    const LINK_REG: usize = 1 as usize;

    // add program counter
    cpu.pc += 2;

    match inst.opc {
        OP_C_LI => {
            cpu.reg[inst.rd.unwrap()] = inst.imm.unwrap();
        },
        OP_C_LW => {
            cpu.reg[inst.rd.unwrap()] =
                Dram::load32(dram, (inst.rs1.unwrap() as i32 + inst.imm.unwrap()));
        },
        OP_C_LWSP => {
            cpu.reg[inst.rd.unwrap()] =
                Dram::load32(dram, (cpu.reg[REG_SP] as i32 + inst.imm.unwrap()));
        },
        OP_C_LUI => {
            cpu.reg[inst.rd.unwrap()] = inst.imm.unwrap() << 12;
        },
        OP_C_SW => {},
        OP_C_SLLI => {},
        OP_C_SWSP => {},
        OP_C_SRLI => {
            cpu.reg[inst.rd.unwrap()] =
                cpu.reg[inst.rs1.unwrap()] >> inst.imm.unwrap() as i32;
        },
        OP_C_SRAI => {
            cpu.reg[inst.rd.unwrap()] =
                (cpu.reg[inst.rs1.unwrap()] as i32) >> inst.imm.unwrap() as i32;
        },
        OP_C_ADD => {
            cpu.reg[inst.rd.unwrap()] =
                cpu.reg[inst.rs1.unwrap()] + cpu.reg[inst.rs2.unwrap()];
        },
        OP_C_ADDI4SPN => {
            cpu.reg[REG_SP] = 
                cpu.reg[REG_SP] + ((cpu.reg[inst.imm.unwrap()] >> 2) & 0x1FF);
        },
        OP_C_ADDI => {
            cpu.reg[inst.rd.unwrap()] += inst.rs1.unwrap() as i32;
        },
        OP_C_ADDI16SP => {
            cpu.reg[REG_SP] = 
                cpu.reg[REG_SP] + ((cpu.reg[inst.imm.unwrap()] >> 4) & 0x1FF);
        },
        OP_C_ANDI => {
            cpu.reg[inst.rd.unwrap()] &= inst.rs1.unwrap() as i32;
        },
        OP_C_SUB => {
            cpu.reg[inst.rd.unwrap()] =
                cpu.reg[inst.rs1.unwrap()] - cpu.reg[inst.rs2.unwrap()];
        },
        OP_C_XOR => {
            cpu.reg[inst.rd.unwrap()] =
                cpu.reg[inst.rs1.unwrap()] ^ cpu.reg[inst.rs2.unwrap()];
        },
        OP_C_OR => {
            cpu.reg[inst.rd.unwrap()] =
                cpu.reg[inst.rs1.unwrap()] | cpu.reg[inst.rs2.unwrap()];
        },
        OP_C_AND => {
            cpu.reg[inst.rd.unwrap()] =
                cpu.reg[inst.rs1.unwrap()] & cpu.reg[inst.rs2.unwrap()];
        },
        OP_C_J => {
            cpu.pc += inst.imm.unwrap() as u32;
        },
        OP_C_JAL => {
            cpu.reg[1] = (cpu.pc + INST_SIZE) as i32; 
            cpu.pc += inst.imm.unwrap() as u32;
        },
        OP_C_JALR => {
            cpu.reg[LINK_REG] = (cpu.pc + INST_SIZE) as i32; 
            cpu.pc += (cpu.reg[inst.rs1.unwrap()]  + inst.imm.unwrap()) as u32;
        },
        OP_C_BEQZ => {
            if cpu.reg[inst.rs1.unwrap()] == 0 {
                cpu.pc += inst.imm.unwrap() as u32;
            } 
        },
        OP_C_BNEZ => {
            if cpu.reg[inst.rs1.unwrap()] != 0 {
                cpu.pc += inst.imm.unwrap() as u32;
            } 
        },
        OP_C_JR => {
            cpu.pc += cpu.reg[inst.rs1.unwrap()] as u32;
        },
        OP_C_MV => {
            cpu.reg[inst.rd.unwrap()] =
                cpu.reg[inst.rs2.unwrap()];
        },
        OP_C_EBREAK => {},
        OP_C_NOP => {/* NOP */},
        _ => panic!("not a compressed Instruction"),
    }
}
