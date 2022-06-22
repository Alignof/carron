use crate::cpu::{CPU, PrivilegedLevel, TransFor, TrapCause};
use crate::cpu::instruction::{Instruction, OpecodeKind};

pub fn exec(inst: &Instruction, cpu: &mut CPU) {
    const INST_SIZE: u32 = 4;

    match inst.opc {
        OpecodeKind::OP_LUI => {
            cpu.regs.write(inst.rd, inst.imm.unwrap());
        },
        OpecodeKind::OP_AUIPC => {
            cpu.regs.write(inst.rd, cpu.pc as i32 + inst.imm.unwrap());
        },
        OpecodeKind::OP_JAL => {
            cpu.regs.write(inst.rd, (cpu.pc + INST_SIZE) as i32); 
            cpu.add2pc(inst.imm.unwrap());
        },
        OpecodeKind::OP_JALR => {
            // calc next_pc before updated
            let next_pc = cpu.pc + INST_SIZE;
            // setting the least-significant bit of the result to zero->vvvvvv
            cpu.update_pc((cpu.regs.read(inst.rs1) + inst.imm.unwrap()) & !0x1);
            cpu.regs.write(inst.rd, next_pc as i32); 
        },
        OpecodeKind::OP_BEQ => {
            if cpu.regs.read(inst.rs1) == cpu.regs.read(inst.rs2) {
                cpu.add2pc(inst.imm.unwrap());
            } 
        },
        OpecodeKind::OP_BNE => {
            if cpu.regs.read(inst.rs1) != cpu.regs.read(inst.rs2) {
                cpu.add2pc(inst.imm.unwrap());
            } 
        },
        OpecodeKind::OP_BLT => {
            if cpu.regs.read(inst.rs1) < cpu.regs.read(inst.rs2) {
                cpu.add2pc(inst.imm.unwrap());
            } 
        },
        OpecodeKind::OP_BGE => {
            if cpu.regs.read(inst.rs1) >= cpu.regs.read(inst.rs2) {
                cpu.add2pc(inst.imm.unwrap());
            } 
        },
        OpecodeKind::OP_BLTU => {
            if (cpu.regs.read(inst.rs1) as u32) < (cpu.regs.read(inst.rs2) as u32) {
                cpu.add2pc(inst.imm.unwrap());
            } 
        },
        OpecodeKind::OP_BGEU => {
            if (cpu.regs.read(inst.rs1) as u32) >= (cpu.regs.read(inst.rs2) as u32) {
                cpu.add2pc(inst.imm.unwrap());
            } 
        },
        OpecodeKind::OP_LB => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                cpu.regs.write(inst.rd, cpu.bus.load8(load_addr));
            }
        },
        OpecodeKind::OP_LH => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                cpu.regs.write(inst.rd, cpu.bus.load16(load_addr));
            }
        },
        OpecodeKind::OP_LW => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                cpu.regs.write(inst.rd, cpu.bus.load32(load_addr));
            }
        },
        OpecodeKind::OP_LBU => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                cpu.regs.write(inst.rd, cpu.bus.load_u8(load_addr));
            }
        },
        OpecodeKind::OP_LHU => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                cpu.regs.write(inst.rd, cpu.bus.load_u16(load_addr));
            }
        },
        OpecodeKind::OP_SB => {
            if let Some(store_addr) = cpu.trans_addr(TransFor::StoreAMO, cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                cpu.bus.store8(store_addr, cpu.regs.read(inst.rs2));
            }
        },
        OpecodeKind::OP_SH => {
            if let Some(store_addr) = cpu.trans_addr(TransFor::StoreAMO, cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                cpu.bus.store16(store_addr, cpu.regs.read(inst.rs2));
            }
        },
        OpecodeKind::OP_SW => {
            if let Some(store_addr) = cpu.trans_addr(TransFor::StoreAMO, cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                cpu.bus.store32(store_addr, cpu.regs.read(inst.rs2));
            }
        },
        OpecodeKind::OP_ADDI => {
            cpu.regs.write(inst.rd, cpu.regs.read(inst.rs1) + inst.imm.unwrap());
        },
        OpecodeKind::OP_SLTI => {
            cpu.regs.write(inst.rd,  
                (cpu.regs.read(inst.rs1) < inst.imm.unwrap()) as i32);
        },
        OpecodeKind::OP_SLTIU => {
            cpu.regs.write(inst.rd,  
                ((cpu.regs.read(inst.rs1) as u32) < inst.imm.unwrap() as u32) as i32);
        },
        OpecodeKind::OP_XORI => {
            cpu.regs.write(inst.rd, cpu.regs.read(inst.rs1) ^ inst.imm.unwrap());
        },
        OpecodeKind::OP_ORI => {
            cpu.regs.write(inst.rd, cpu.regs.read(inst.rs1) | inst.imm.unwrap());
        },
        OpecodeKind::OP_ANDI => {
            cpu.regs.write(inst.rd, cpu.regs.read(inst.rs1) & inst.imm.unwrap());
        },
        OpecodeKind::OP_SLLI => {
            cpu.regs.write(inst.rd,
                ((cpu.regs.read(inst.rs1) as u32) << inst.imm.unwrap()) as i32);
        },                                                
        OpecodeKind::OP_SRLI => {
            cpu.regs.write(inst.rd,
                ((cpu.regs.read(inst.rs1) as u32) >> inst.imm.unwrap()) as i32);
        },
        OpecodeKind::OP_SRAI => {
            cpu.regs.write(inst.rd,
                ((cpu.regs.read(inst.rs1) as i32) >> inst.imm.unwrap()) as i32);
        },
        OpecodeKind::OP_ADD => {
            cpu.regs.write(inst.rd,
                cpu.regs.read(inst.rs1) + cpu.regs.read(inst.rs2));
        },
        OpecodeKind::OP_SUB => {
            cpu.regs.write(inst.rd,
                cpu.regs.read(inst.rs1) - cpu.regs.read(inst.rs2));
        },
        OpecodeKind::OP_SLL => {
            cpu.regs.write(inst.rd,
                ((cpu.regs.read(inst.rs1) as u32) << cpu.regs.read(inst.rs2)) as i32);
        },
        OpecodeKind::OP_SLT => {
            cpu.regs.write(inst.rd,
                (cpu.regs.read(inst.rs1) < cpu.regs.read(inst.rs2)) as i32);
        },
        OpecodeKind::OP_SLTU => {
            cpu.regs.write(inst.rd,
                ((cpu.regs.read(inst.rs1) as u32) < (cpu.regs.read(inst.rs2) as u32)) as i32);
        },
        OpecodeKind::OP_XOR => {
            cpu.regs.write(inst.rd,
                cpu.regs.read(inst.rs1) ^ cpu.regs.read(inst.rs2));
        },
        OpecodeKind::OP_SRL => {
            cpu.regs.write(inst.rd,
                ((cpu.regs.read(inst.rs1) as u32)  >> cpu.regs.read(inst.rs2)) as i32);
        },
        OpecodeKind::OP_SRA => {
            cpu.regs.write(inst.rd,
                (cpu.regs.read(inst.rs1) as i32)  >> cpu.regs.read(inst.rs2));
        },
        OpecodeKind::OP_OR => {
            cpu.regs.write(inst.rd,
                cpu.regs.read(inst.rs1) | cpu.regs.read(inst.rs2));
        },
        OpecodeKind::OP_AND => {
            cpu.regs.write(inst.rd,
                cpu.regs.read(inst.rs1) & cpu.regs.read(inst.rs2));
        },
        OpecodeKind::OP_FENCE => {
            // nop (pipeline are not yet implemented)
        },
        OpecodeKind::OP_ECALL => {
            cpu.exception(cpu.pc as i32, 
                match cpu.priv_lv {
                    PrivilegedLevel::User => TrapCause::UmodeEcall,
                    PrivilegedLevel::Supervisor => TrapCause::SmodeEcall,
                    PrivilegedLevel::Machine => TrapCause::MmodeEcall,
                    _ => panic!("cannot enviroment call in current privileged mode."),
                }
            );
        },
        OpecodeKind::OP_EBREAK => {
            panic!("not yet implemented: OP_EBREAK");
        },
        _ => panic!("not an Base extension"),
    }
}

