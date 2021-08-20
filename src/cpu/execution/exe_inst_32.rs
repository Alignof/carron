use crate::cpu::CPU;
use crate::cpu::instruction::{Instruction, OpecodeKind};
use crate::bus::dram::Dram;

pub fn exe_inst(inst: &Instruction, cpu: &mut CPU, dram: &mut Dram) {
    use OpecodeKind::*;
    const INST_SIZE: usize = 4;

    // add program counter
    cpu.pc += INST_SIZE;

    match inst.opc {
        OP_LUI => {
            cpu.write_reg(inst.rd, inst.imm.unwrap() << 12);
        },
        OP_AUIPC => {
            cpu.pc += (inst.imm.unwrap() << 12) as usize;
        },
        OP_JAL => {
            cpu.write_reg(inst.rd, (cpu.pc + INST_SIZE) as i32); 
            cpu.pc += inst.imm.unwrap() as usize;
        },
        OP_JALR => {
            cpu.write_reg(inst.rd, (cpu.pc + INST_SIZE) as i32); 
            cpu.pc += (cpu.reg[inst.rs1.unwrap()]  + inst.imm.unwrap()) as usize;
        },
        OP_BEQ => {
            if cpu.reg[inst.rs1.unwrap()] == cpu.reg[inst.rs2.unwrap()] {
                cpu.pc += inst.imm.unwrap() as usize;
            } 
        },
        OP_BNE => {
            if cpu.reg[inst.rs1.unwrap()] != cpu.reg[inst.rs2.unwrap()] {
                cpu.pc += inst.imm.unwrap() as usize;
            } 
        },
        OP_BLT => {
            if cpu.reg[inst.rs1.unwrap()] < cpu.reg[inst.rs2.unwrap()] {
                cpu.pc += inst.imm.unwrap() as usize;
            } 
        },
        OP_BGE => {
            if cpu.reg[inst.rs1.unwrap()] >= cpu.reg[inst.rs2.unwrap()] {
                cpu.pc += inst.imm.unwrap() as usize;
            } 
        },
        OP_BLTU => {
            if (cpu.reg[inst.rs1.unwrap()] as u32) < (cpu.reg[inst.rs2.unwrap()] as u32) {
                cpu.pc += inst.imm.unwrap() as usize;
            } 
        },
        OP_BGEU => {
            if (cpu.reg[inst.rs1.unwrap()] as u32) >= (cpu.reg[inst.rs2.unwrap()] as u32) {
                cpu.pc += inst.imm.unwrap() as usize;
            } 
        },
        OP_LB => {
            cpu.write_reg(inst.rd,  
                Dram::load8(dram, (cpu.reg[inst.rs1.unwrap()] + inst.imm.unwrap()) as usize));
        },
        OP_LH => {
            cpu.write_reg(inst.rd,  
                Dram::load16(dram, (cpu.reg[inst.rs1.unwrap()] + inst.imm.unwrap()) as usize));
        },
        OP_LW => {
            cpu.write_reg(inst.rd,  
                Dram::load32(dram, (cpu.reg[inst.rs1.unwrap()] + inst.imm.unwrap()) as usize));
        },
        OP_LBU => {
            cpu.write_reg(inst.rd,  
                Dram::load_u8(dram, (cpu.reg[inst.rs1.unwrap()] + inst.imm.unwrap()) as usize));
        },
        OP_LHU => {
            cpu.write_reg(inst.rd,  
                Dram::load_u16(dram, (cpu.reg[inst.rs1.unwrap()] + inst.imm.unwrap()) as usize));
        },
        OP_SB => {
            Dram::store8(dram, (cpu.reg[inst.rs1.unwrap()] + inst.imm.unwrap()) as usize,
                         cpu.reg[inst.rs2.unwrap()]);
        },
        OP_SH => {
            Dram::store16(dram, (cpu.reg[inst.rs1.unwrap()] + inst.imm.unwrap()) as usize,
                         cpu.reg[inst.rs2.unwrap()]);
        },
        OP_SW => {
            Dram::store32(dram, (cpu.reg[inst.rs1.unwrap()] + inst.imm.unwrap()) as usize,
                         cpu.reg[inst.rs2.unwrap()]);
        },
        OP_ADDI => {
            cpu.write_reg(inst.rd,
                cpu.reg[inst.rd.unwrap()] + cpu.reg[inst.rs1.unwrap()] + inst.imm.unwrap());
        },
        OP_SLTI => {
            cpu.write_reg(inst.rd,  
                (cpu.reg[inst.rs1.unwrap()] < inst.imm.unwrap()) as i32);
        },
        OP_SLTIU => {
            cpu.write_reg(inst.rd,  
                ((cpu.reg[inst.rs1.unwrap()] as u32) < inst.imm.unwrap() as u32) as i32);
        },
        OP_XORI => {
            cpu.write_reg(inst.rd, cpu.reg[inst.rs1.unwrap()] ^ inst.imm.unwrap());
        },
        OP_ORI => {
            cpu.write_reg(inst.rd, cpu.reg[inst.rs1.unwrap()] | inst.imm.unwrap());
        },
        OP_ANDI => {
            cpu.write_reg(inst.rd, cpu.reg[inst.rs1.unwrap()] & inst.imm.unwrap());
        },
        OP_SLLI => {
            cpu.write_reg(inst.rd,
                ((cpu.reg[inst.rs1.unwrap()] as u32) << inst.imm.unwrap()) as i32);
        },                                                
        OP_SRLI => {                                    
            cpu.write_reg(inst.rd,          
                ((cpu.reg[inst.rs1.unwrap()] as u32) >> inst.imm.unwrap()) as i32);
        },
        OP_ADD => {
            cpu.write_reg(inst.rd,
                cpu.reg[inst.rs1.unwrap()] + cpu.reg[inst.rs2.unwrap()]);
        },
        OP_SUB => {
            cpu.write_reg(inst.rd,
                cpu.reg[inst.rs1.unwrap()] - cpu.reg[inst.rs2.unwrap()]);
        },
        OP_SLL => {
            cpu.write_reg(inst.rd,
                ((cpu.reg[inst.rs1.unwrap()] as u32) << cpu.reg[inst.rs2.unwrap()]) as i32);
        },
        OP_SLT => {
            cpu.write_reg(inst.rd,
                (cpu.reg[inst.rs1.unwrap()] < cpu.reg[inst.rs2.unwrap()]) as i32);
        },
        OP_SLTU => {
            cpu.write_reg(inst.rd,
                ((cpu.reg[inst.rs1.unwrap()] as u32) < (cpu.reg[inst.rs2.unwrap()] as u32)) as i32);
        },
        OP_XOR => {
            cpu.write_reg(inst.rd,
                cpu.reg[inst.rs1.unwrap()] ^ cpu.reg[inst.rs2.unwrap()]);
        },
        OP_SRL => {
            cpu.write_reg(inst.rd,
                ((cpu.reg[inst.rs1.unwrap()] as u32)  >> cpu.reg[inst.rs2.unwrap()]) as i32);
        },
        OP_SRA => {
            cpu.write_reg(inst.rd,
                (cpu.reg[inst.rs1.unwrap()] as i32)  >> cpu.reg[inst.rs2.unwrap()]);
        },
        OP_OR => {
            cpu.write_reg(inst.rd,
                cpu.reg[inst.rs1.unwrap()] | cpu.reg[inst.rs2.unwrap()]);
        },
        OP_AND => {
            cpu.write_reg(inst.rd,
                cpu.reg[inst.rs1.unwrap()] & cpu.reg[inst.rs2.unwrap()]);
        },
        OP_FENCE => {
            panic!("not yet implemented: OP_FENCE");
        },
        OP_ECALL => {
            panic!("not yet implemented: OP_ECALL");
        },
        OP_EBREAK => {
            panic!("not yet implemented: OP_EBREAK");
        },
        _ => panic!("not a full instruction"),
    }
}
