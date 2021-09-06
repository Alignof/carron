use crate::cpu::{CPU, PrivilegedLevel};
use crate::cpu::csr::{CSRname, Mstatus};
use crate::cpu::instruction::{Instruction, OpecodeKind};

pub fn exe_inst(inst: &Instruction, cpu: &mut CPU) {
    use OpecodeKind::*;
    const INST_SIZE: usize = 4;

    // store previous program counter for excluding branch case
    let prev_pc = cpu.pc;

    match inst.opc {
        OP_LUI => {
            cpu.write_reg(inst.rd, inst.imm.unwrap() << 12);
        },
        OP_AUIPC => {
            cpu.write_reg(inst.rd, cpu.pc as i32 + (inst.imm.unwrap() << 12));
        },
        OP_JAL => {
            cpu.write_reg(inst.rd, (cpu.pc + INST_SIZE) as i32); 
            cpu.add2pc(inst.imm.unwrap());
        },
        OP_JALR => {
            let next_pc = cpu.pc + INST_SIZE;
            // setting the least-significant bit of the result to zero->vvvvvv
            cpu.update_pc((cpu.read_reg(inst.rs1)  + inst.imm.unwrap()) & !0x1);
            cpu.write_reg(inst.rd, next_pc as i32); 
        },
        OP_BEQ => {
            if cpu.read_reg(inst.rs1) == cpu.read_reg(inst.rs2) {
                cpu.add2pc(inst.imm.unwrap());
            } 
        },
        OP_BNE => {
            if cpu.read_reg(inst.rs1) != cpu.read_reg(inst.rs2) {
                cpu.add2pc(inst.imm.unwrap());
            } 
        },
        OP_BLT => {
            if cpu.read_reg(inst.rs1) < cpu.read_reg(inst.rs2) {
                cpu.add2pc(inst.imm.unwrap());
            } 
        },
        OP_BGE => {
            if cpu.read_reg(inst.rs1) >= cpu.read_reg(inst.rs2) {
                cpu.add2pc(inst.imm.unwrap());
            } 
        },
        OP_BLTU => {
            if (cpu.read_reg(inst.rs1) as u32) < (cpu.read_reg(inst.rs2) as u32) {
                cpu.add2pc(inst.imm.unwrap());
            } 
        },
        OP_BGEU => {
            if (cpu.read_reg(inst.rs1) as u32) >= (cpu.read_reg(inst.rs2) as u32) {
                cpu.add2pc(inst.imm.unwrap());
            } 
        },
        OP_LB => {
            cpu.write_reg(inst.rd,  
                cpu.bus.dram.load8((cpu.read_reg(inst.rs1) + inst.imm.unwrap()) as usize));
        },
        OP_LH => {
            cpu.write_reg(inst.rd,  
                cpu.bus.dram.load16((cpu.read_reg(inst.rs1) + inst.imm.unwrap()) as usize));
        },
        OP_LW => {
            cpu.write_reg(inst.rd,  
                cpu.bus.dram.load32((cpu.read_reg(inst.rs1) + inst.imm.unwrap()) as usize));
        },
        OP_LBU => {
            cpu.write_reg(inst.rd,  
                cpu.bus.dram.load_u8((cpu.read_reg(inst.rs1) + inst.imm.unwrap()) as usize));
        },
        OP_LHU => {
            cpu.write_reg(inst.rd,  
                cpu.bus.dram.load_u16((cpu.read_reg(inst.rs1) + inst.imm.unwrap()) as usize));
        },
        OP_SB => {
            cpu.bus.dram.store8((cpu.read_reg(inst.rs1) + inst.imm.unwrap()) as usize,
                         cpu.read_reg(inst.rs2));
        },
        OP_SH => {
            cpu.bus.dram.store16((cpu.read_reg(inst.rs1) + inst.imm.unwrap()) as usize,
                         cpu.read_reg(inst.rs2));
        },
        OP_SW => {
            cpu.bus.dram.store32((cpu.read_reg(inst.rs1) + inst.imm.unwrap()) as usize,
                         cpu.read_reg(inst.rs2));
        },
        OP_ADDI => {
            cpu.write_reg(inst.rd, cpu.read_reg(inst.rs1) + inst.imm.unwrap());
        },
        OP_SLTI => {
            cpu.write_reg(inst.rd,  
                (cpu.read_reg(inst.rs1) < inst.imm.unwrap()) as i32);
        },
        OP_SLTIU => {
            cpu.write_reg(inst.rd,  
                ((cpu.read_reg(inst.rs1) as u32) < inst.imm.unwrap() as u32) as i32);
        },
        OP_XORI => {
            cpu.write_reg(inst.rd, cpu.read_reg(inst.rs1) ^ inst.imm.unwrap());
        },
        OP_ORI => {
            cpu.write_reg(inst.rd, cpu.read_reg(inst.rs1) | inst.imm.unwrap());
        },
        OP_ANDI => {
            cpu.write_reg(inst.rd, cpu.read_reg(inst.rs1) & inst.imm.unwrap());
        },
        OP_SLLI => {
            cpu.write_reg(inst.rd,
                ((cpu.read_reg(inst.rs1) as u32) << inst.imm.unwrap()) as i32);
        },                                                
        OP_SRLI => {
            cpu.write_reg(inst.rd,
                ((cpu.read_reg(inst.rs1) as u32) >> inst.imm.unwrap()) as i32);
        },
        OP_SRAI => {
            cpu.write_reg(inst.rd,
                ((cpu.read_reg(inst.rs1) as i32) >> inst.imm.unwrap()) as i32);
        },
        OP_ADD => {
            cpu.write_reg(inst.rd,
                cpu.read_reg(inst.rs1) + cpu.read_reg(inst.rs2));
        },
        OP_SUB => {
            cpu.write_reg(inst.rd,
                cpu.read_reg(inst.rs1) - cpu.read_reg(inst.rs2));
        },
        OP_SLL => {
            cpu.write_reg(inst.rd,
                ((cpu.read_reg(inst.rs1) as u32) << cpu.read_reg(inst.rs2)) as i32);
        },
        OP_SLT => {
            cpu.write_reg(inst.rd,
                (cpu.read_reg(inst.rs1) < cpu.read_reg(inst.rs2)) as i32);
        },
        OP_SLTU => {
            cpu.write_reg(inst.rd,
                ((cpu.read_reg(inst.rs1) as u32) < (cpu.read_reg(inst.rs2) as u32)) as i32);
        },
        OP_XOR => {
            cpu.write_reg(inst.rd,
                cpu.read_reg(inst.rs1) ^ cpu.read_reg(inst.rs2));
        },
        OP_SRL => {
            cpu.write_reg(inst.rd,
                ((cpu.read_reg(inst.rs1) as u32)  >> cpu.read_reg(inst.rs2)) as i32);
        },
        OP_SRA => {
            cpu.write_reg(inst.rd,
                (cpu.read_reg(inst.rs1) as i32)  >> cpu.read_reg(inst.rs2));
        },
        OP_OR => {
            cpu.write_reg(inst.rd,
                cpu.read_reg(inst.rs1) | cpu.read_reg(inst.rs2));
        },
        OP_AND => {
            cpu.write_reg(inst.rd,
                cpu.read_reg(inst.rs1) & cpu.read_reg(inst.rs2));
        },
        OP_FENCE => {
            // nop (pipeline are not yet implemented)
        },
        OP_ECALL => {
            cpu.write_csr(CSRname::mcause.wrap(), match cpu.priv_lv {
                PrivilegedLevel::User => 8,
                PrivilegedLevel::Supervisor => 9,
                _ => panic!("cannot enviroment call in current privileged mode."),
            });
            cpu.write_csr(CSRname::mepc.wrap(), cpu.pc as i32);
            cpu.bitclr_csr(CSRname::mstatus.wrap(), 0x3 << 11);
            cpu.priv_lv = PrivilegedLevel::Machine;
            cpu.update_pc(cpu.read_csr(CSRname::mtvec.wrap()) as i32);
        },
        OP_EBREAK => {
            panic!("not yet implemented: OP_EBREAK");
        },
        OP_CSRRW => {
            cpu.write_reg(inst.rd, cpu.read_csr(inst.rs2) as i32);
            cpu.write_csr(inst.rs2, cpu.read_reg(inst.rs1));
        },
        OP_CSRRS => {
            cpu.write_reg(inst.rd, cpu.read_csr(inst.rs2) as i32);
            cpu.bitset_csr(inst.rs2, cpu.read_reg(inst.rs1));
        },
        OP_CSRRC => {
            cpu.write_reg(inst.rd, cpu.read_csr(inst.rs2) as i32);
            cpu.bitclr_csr(inst.rs2, cpu.read_reg(inst.rs1));
        },
        OP_CSRRWI => {
            cpu.write_reg(inst.rd, cpu.read_csr(inst.rs2) as i32);
            cpu.write_csr(inst.rs2, inst.rs1.unwrap() as i32);
        },
        OP_CSRRSI => {
            cpu.write_reg(inst.rd, cpu.read_csr(inst.rs2) as i32);
            cpu.bitset_csr(inst.rs2, inst.rs1.unwrap() as i32);
        },
        OP_CSRRCI => {
            cpu.write_reg(inst.rd, cpu.read_csr(inst.rs2) as i32);
            cpu.bitclr_csr(inst.rs2, inst.rs1.unwrap() as i32);
        },
        OP_MRET => {
            cpu.update_pc(cpu.read_csr(CSRname::mepc.wrap()) as i32);
            cpu.priv_lv = match cpu.read_csr_mstatus(Mstatus::MPP) {
                0b00 => PrivilegedLevel::User,
                0b01 => PrivilegedLevel::Supervisor,
                0b11 => PrivilegedLevel::Machine,
                _ => panic!("PrivilegedLevel 0x3 is Reserved."),
            }
        },
        _ => panic!("not a full instruction"),
    }

    // add the program counter when it isn't a branch instruction
    if cpu.pc == prev_pc {
        cpu.add2pc(INST_SIZE as i32);
    }
}
