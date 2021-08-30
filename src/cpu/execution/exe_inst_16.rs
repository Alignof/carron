use crate::cpu::CPU;
use crate::cpu::instruction::{Instruction, OpecodeKind};

pub fn exe_cinst(inst: &Instruction, cpu: &mut CPU) {
    use OpecodeKind::*;
    const INST_SIZE: usize = 2;
    const REG_SP: usize = 2;
    const LINK_REG: usize = 1;

    // store previous program counter for excluding branch case
    let prev_pc = cpu.pc;

    match inst.opc {
        OP_C_LI => {
            cpu.write_reg(inst.rd, inst.imm.unwrap());
        },
        OP_C_LW => {
            cpu.write_reg(inst.rd,
                cpu.bus.dram.load32((cpu.read_reg(inst.rs1) + inst.imm.unwrap()) as usize));
        },
        OP_C_LWSP => {
            cpu.write_reg(inst.rd,
                cpu.bus.dram.load32((cpu.read_reg(Some(REG_SP)) + inst.imm.unwrap()) as usize));
        },
        OP_C_LUI => {
            cpu.write_reg(inst.rd, inst.imm.unwrap() << 12);
        },
        OP_C_SW => {
            cpu.bus.dram.store32((cpu.read_reg(inst.rs1) + inst.imm.unwrap()) as usize,
                         cpu.read_reg(inst.rs2));
        },
        OP_C_SLLI => {
            cpu.write_reg(inst.rd,
                ((cpu.read_reg(inst.rs1) as u32) << inst.imm.unwrap()) as i32);
        },
        OP_C_SWSP => {
            cpu.bus.dram.store32((cpu.read_reg(Some(REG_SP)) + inst.imm.unwrap()) as usize,
                         cpu.read_reg(inst.rs2));
        },
        OP_C_SRLI => {
            cpu.write_reg(inst.rd,
                cpu.read_reg(inst.rs1) >> inst.imm.unwrap());
        },
        OP_C_SRAI => {
            cpu.write_reg(inst.rd,
                cpu.read_reg(inst.rs1) >> inst.imm.unwrap());
        },
        OP_C_ADD => {
            cpu.write_reg(inst.rd,
                cpu.read_reg(inst.rs1) + cpu.read_reg(inst.rs2));
        },
        OP_C_ADDI4SPN => {
            cpu.write_reg(inst.rd, cpu.read_reg(inst.rd) + inst.imm.unwrap());
        },
        OP_C_ADDI => {
            cpu.write_reg(inst.rd, cpu.read_reg(inst.rd) + inst.imm.unwrap());
        },
        OP_C_ADDI16SP => {
            cpu.write_reg(Some(REG_SP), cpu.read_reg(Some(REG_SP)) + inst.imm.unwrap());
        },
        OP_C_ANDI => {
            cpu.write_reg(inst.rd, cpu.read_reg(inst.rd) & inst.imm.unwrap());
        },
        OP_C_SUB => {
            cpu.write_reg(inst.rd,
                cpu.read_reg(inst.rs1) - cpu.read_reg(inst.rs2));
        },
        OP_C_XOR => {
            cpu.write_reg(inst.rd,
                cpu.read_reg(inst.rs1) ^ cpu.read_reg(inst.rs2));
        },
        OP_C_OR => {
            cpu.write_reg(inst.rd,
                cpu.read_reg(inst.rs1) | cpu.read_reg(inst.rs2));
        },
        OP_C_AND => {
            cpu.write_reg(inst.rd,
                cpu.read_reg(inst.rs1) & cpu.read_reg(inst.rs2));
        },
        OP_C_J => {
            cpu.pc += inst.imm.unwrap() as usize;
        },
        OP_C_JAL => {
            cpu.write_reg(Some(1), (cpu.pc + INST_SIZE) as i32); 
            cpu.pc += inst.imm.unwrap() as usize;
        },
        OP_C_JALR => {
            cpu.write_reg(Some(LINK_REG), (cpu.pc + INST_SIZE) as i32); 
            cpu.pc += (cpu.read_reg(inst.rs1)  + inst.imm.unwrap()) as usize;
        },
        OP_C_BEQZ => {
            if cpu.read_reg(inst.rs1) == 0 {
                cpu.pc += inst.imm.unwrap() as usize;
            } 
        },
        OP_C_BNEZ => {
            if cpu.read_reg(inst.rs1) != 0 {
                cpu.pc += inst.imm.unwrap() as usize;
            } 
        },
        OP_C_JR => {
            cpu.pc += cpu.read_reg(inst.rs1) as usize;
        },
        OP_C_MV => {
            cpu.write_reg(inst.rd, cpu.read_reg(inst.rs2));
        },
        OP_C_EBREAK => {
            panic!("not yet implemented: OP_C_EBREAK");
        },
        OP_C_NOP => {/* NOP */},
        _ => panic!("not a compressed Instruction"),
    }

    // add the program counter when it isn't a branch instruction
    if cpu.pc == prev_pc {
        cpu.pc += INST_SIZE;
    }
}
