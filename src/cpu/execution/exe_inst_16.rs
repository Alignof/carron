use crate::cpu::CPU;
use crate::cpu::instruction::{Instruction, OpecodeKind};
use crate::bus::Device;

pub fn exe_cinst(inst: &Instruction, cpu: &mut CPU) {
    use OpecodeKind::*;
    const INST_SIZE: usize = 2;
    const REG_SP: usize = 2;
    const LINK_REG: usize = 1;

    // store previous program counter for excluding branch case
    let prev_pc = cpu.pc;

    match inst.opc {
        OP_C_LI => {
            cpu.regs.write(inst.rd, inst.imm.unwrap());
        },
        OP_C_LW => {
            cpu.regs.write(inst.rd,
                cpu.bus.load32((cpu.regs.read(inst.rs1) + inst.imm.unwrap()) as usize));
        },
        OP_C_LWSP => {
            cpu.regs.write(inst.rd,
                cpu.bus.load32((cpu.regs.read(Some(REG_SP)) + inst.imm.unwrap()) as usize));
        },
        OP_C_LUI => {
            cpu.regs.write(inst.rd, inst.imm.unwrap() << 12);
        },
        OP_C_SW => {
            cpu.bus.dram.store32((cpu.regs.read(inst.rs1) + inst.imm.unwrap()) as usize,
                         cpu.regs.read(inst.rs2));
        },
        OP_C_SLLI => {
            cpu.regs.write(inst.rd,
                ((cpu.regs.read(inst.rs1) as u32) << inst.imm.unwrap()) as i32);
        },
        OP_C_SWSP => {
            cpu.bus.dram.store32((cpu.regs.read(Some(REG_SP)) + inst.imm.unwrap()) as usize,
                         cpu.regs.read(inst.rs2));
        },
        OP_C_SRLI => {
            cpu.regs.write(inst.rd,
                cpu.regs.read(inst.rs1) >> inst.imm.unwrap());
        },
        OP_C_SRAI => {
            cpu.regs.write(inst.rd,
                cpu.regs.read(inst.rs1) >> inst.imm.unwrap());
        },
        OP_C_ADD => {
            cpu.regs.write(inst.rd,
                cpu.regs.read(inst.rs1) + cpu.regs.read(inst.rs2));
        },
        OP_C_ADDI4SPN => {
            cpu.regs.write(inst.rd, cpu.regs.read(inst.rd) + inst.imm.unwrap());
        },
        OP_C_ADDI => {
            cpu.regs.write(inst.rd, cpu.regs.read(inst.rd) + inst.imm.unwrap());
        },
        OP_C_ADDI16SP => {
            cpu.regs.write(Some(REG_SP), cpu.regs.read(Some(REG_SP)) + inst.imm.unwrap());
        },
        OP_C_ANDI => {
            cpu.regs.write(inst.rd, cpu.regs.read(inst.rd) & inst.imm.unwrap());
        },
        OP_C_SUB => {
            cpu.regs.write(inst.rd,
                cpu.regs.read(inst.rs1) - cpu.regs.read(inst.rs2));
        },
        OP_C_XOR => {
            cpu.regs.write(inst.rd,
                cpu.regs.read(inst.rs1) ^ cpu.regs.read(inst.rs2));
        },
        OP_C_OR => {
            cpu.regs.write(inst.rd,
                cpu.regs.read(inst.rs1) | cpu.regs.read(inst.rs2));
        },
        OP_C_AND => {
            cpu.regs.write(inst.rd,
                cpu.regs.read(inst.rs1) & cpu.regs.read(inst.rs2));
        },
        OP_C_J => {
            cpu.pc += inst.imm.unwrap() as usize;
        },
        OP_C_JAL => {
            cpu.regs.write(Some(1), (cpu.pc + INST_SIZE) as i32); 
            cpu.pc += inst.imm.unwrap() as usize;
        },
        OP_C_JALR => {
            cpu.regs.write(Some(LINK_REG), (cpu.pc + INST_SIZE) as i32); 
            cpu.pc += (cpu.regs.read(inst.rs1)  + inst.imm.unwrap()) as usize;
        },
        OP_C_BEQZ => {
            if cpu.regs.read(inst.rs1) == 0 {
                cpu.pc += inst.imm.unwrap() as usize;
            } 
        },
        OP_C_BNEZ => {
            if cpu.regs.read(inst.rs1) != 0 {
                cpu.pc += inst.imm.unwrap() as usize;
            } 
        },
        OP_C_JR => {
            cpu.pc += cpu.regs.read(inst.rs1) as usize;
        },
        OP_C_MV => {
            cpu.regs.write(inst.rd, cpu.regs.read(inst.rs2));
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
