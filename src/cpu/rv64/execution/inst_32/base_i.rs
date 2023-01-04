use super::Cpu64;
use crate::cpu::instruction::{Instruction, OpecodeKind};
use crate::cpu::CPU;
use crate::cpu::{PrivilegedLevel, TransAlign, TransFor, TrapCause};

pub fn exec(inst: &Instruction, cpu: &mut Cpu64) -> Result<(), (Option<u64>, TrapCause, String)> {
    const INST_SIZE: u64 = 4;

    match inst.opc {
        OpecodeKind::OP_LUI => {
            cpu.regs.write(inst.rd, inst.imm.unwrap() as u64);
        }
        OpecodeKind::OP_AUIPC => {
            cpu.regs.write(inst.rd, cpu.pc + inst.imm.unwrap() as u64);
        }
        OpecodeKind::OP_JAL => {
            cpu.regs.write(inst.rd, cpu.pc + INST_SIZE);
            cpu.add2pc(inst.imm.unwrap() as u64);
        }
        OpecodeKind::OP_JALR => {
            // calc next_pc before updated
            let next_pc = cpu.pc + INST_SIZE;
            // setting the least-significant bit of the result to zero-->vvvvvv
            cpu.update_pc((cpu.regs.read(inst.rs1) + inst.imm.unwrap() as u64) & !0x1);
            cpu.regs.write(inst.rd, next_pc);
        }
        OpecodeKind::OP_BEQ => {
            if cpu.regs.read(inst.rs1) == cpu.regs.read(inst.rs2) {
                cpu.add2pc(inst.imm.unwrap() as u64);
            }
        }
        OpecodeKind::OP_BNE => {
            if cpu.regs.read(inst.rs1) != cpu.regs.read(inst.rs2) {
                cpu.add2pc(inst.imm.unwrap() as u64);
            }
        }
        OpecodeKind::OP_BLT => {
            if (cpu.regs.read(inst.rs1) as i32) < (cpu.regs.read(inst.rs2) as i32) {
                cpu.add2pc(inst.imm.unwrap() as u64);
            }
        }
        OpecodeKind::OP_BGE => {
            if (cpu.regs.read(inst.rs1) as i32) >= (cpu.regs.read(inst.rs2) as i32) {
                cpu.add2pc(inst.imm.unwrap() as u64);
            }
        }
        OpecodeKind::OP_BLTU => {
            if cpu.regs.read(inst.rs1) < cpu.regs.read(inst.rs2) {
                cpu.add2pc(inst.imm.unwrap() as u64);
            }
        }
        OpecodeKind::OP_BGEU => {
            if cpu.regs.read(inst.rs1) >= cpu.regs.read(inst.rs2) {
                cpu.add2pc(inst.imm.unwrap() as u64);
            }
        }
        OpecodeKind::OP_LB => {
            let load_addr = cpu.trans_addr(
                TransFor::Load,
                TransAlign::Size8,
                (cpu.regs.read(inst.rs1) as i32 + inst.imm.unwrap()) as u64,
            )?;
            cpu.regs.write(inst.rd, cpu.bus.load8(load_addr)?);
        }
        OpecodeKind::OP_LH => {
            let load_addr = cpu.trans_addr(
                TransFor::Load,
                TransAlign::Size16,
                (cpu.regs.read(inst.rs1) as i32 + inst.imm.unwrap()) as u64,
            )?;
            cpu.regs.write(inst.rd, cpu.bus.load16(load_addr)?);
        }
        OpecodeKind::OP_LW => {
            let load_addr = cpu.trans_addr(
                TransFor::Load,
                TransAlign::Size32,
                (cpu.regs.read(inst.rs1) as i32 + inst.imm.unwrap()) as u64,
            )?;
            cpu.regs.write(inst.rd, cpu.bus.load32(load_addr)?);
        }
        OpecodeKind::OP_LBU => {
            let load_addr = cpu.trans_addr(
                TransFor::Load,
                TransAlign::Size8,
                cpu.regs.read(inst.rs1) + inst.imm.unwrap() as u64,
            )?;
            cpu.regs.write(inst.rd, cpu.bus.load_u8(load_addr)?);
        }
        OpecodeKind::OP_LHU => {
            let load_addr = cpu.trans_addr(
                TransFor::Load,
                TransAlign::Size16,
                cpu.regs.read(inst.rs1) + inst.imm.unwrap() as u64,
            )?;
            cpu.regs.write(inst.rd, cpu.bus.load_u16(load_addr)?);
        }
        OpecodeKind::OP_SB => {
            let store_addr = cpu.trans_addr(
                TransFor::StoreAMO,
                TransAlign::Size8,
                (cpu.regs.read(inst.rs1) as i32 + inst.imm.unwrap()) as u64,
            )?;
            cpu.bus.store8(store_addr, cpu.regs.read(inst.rs2))?;
        }
        OpecodeKind::OP_SH => {
            let store_addr = cpu.trans_addr(
                TransFor::StoreAMO,
                TransAlign::Size16,
                (cpu.regs.read(inst.rs1) as i32 + inst.imm.unwrap()) as u64,
            )?;
            cpu.bus.store16(store_addr, cpu.regs.read(inst.rs2))?;
        }
        OpecodeKind::OP_SW => {
            let store_addr = cpu.trans_addr(
                TransFor::StoreAMO,
                TransAlign::Size32,
                (cpu.regs.read(inst.rs1) as i32 + inst.imm.unwrap()) as u64,
            )?;
            cpu.bus.store32(store_addr, cpu.regs.read(inst.rs2))?;
        }
        OpecodeKind::OP_ADDI => {
            cpu.regs.write(
                inst.rd,
                (cpu.regs.read(inst.rs1) as i32 + inst.imm.unwrap()) as u64,
            );
        }
        OpecodeKind::OP_SLTI => {
            cpu.regs.write(
                inst.rd,
                ((cpu.regs.read(inst.rs1) as i32) < inst.imm.unwrap()) as u64,
            );
        }
        OpecodeKind::OP_SLTIU => {
            cpu.regs.write(
                inst.rd,
                (cpu.regs.read(inst.rs1) < (inst.imm.unwrap() as u64)) as u64,
            );
        }
        OpecodeKind::OP_XORI => {
            cpu.regs
                .write(inst.rd, cpu.regs.read(inst.rs1) ^ inst.imm.unwrap() as u64);
        }
        OpecodeKind::OP_ORI => {
            cpu.regs
                .write(inst.rd, cpu.regs.read(inst.rs1) | inst.imm.unwrap() as u64);
        }
        OpecodeKind::OP_ANDI => {
            cpu.regs
                .write(inst.rd, cpu.regs.read(inst.rs1) & inst.imm.unwrap() as u64);
        }
        OpecodeKind::OP_SLLI => {
            cpu.regs
                .write(inst.rd, cpu.regs.read(inst.rs1) << inst.imm.unwrap() as u64);
        }
        OpecodeKind::OP_SRLI => {
            cpu.regs
                .write(inst.rd, cpu.regs.read(inst.rs1) >> inst.imm.unwrap() as u64);
        }
        OpecodeKind::OP_SRAI => {
            cpu.regs.write(
                inst.rd,
                ((cpu.regs.read(inst.rs1) as i32) >> inst.imm.unwrap()) as u64,
            );
        }
        OpecodeKind::OP_ADD => {
            cpu.regs
                .write(inst.rd, cpu.regs.read(inst.rs1) + cpu.regs.read(inst.rs2));
        }
        OpecodeKind::OP_SUB => {
            cpu.regs
                .write(inst.rd, cpu.regs.read(inst.rs1) - cpu.regs.read(inst.rs2));
        }
        OpecodeKind::OP_SLL => {
            cpu.regs
                .write(inst.rd, cpu.regs.read(inst.rs1) << cpu.regs.read(inst.rs2));
        }
        OpecodeKind::OP_SLT => {
            cpu.regs.write(
                inst.rd,
                ((cpu.regs.read(inst.rs1) as i32) < (cpu.regs.read(inst.rs2) as i32)) as u64,
            );
        }
        OpecodeKind::OP_SLTU => {
            cpu.regs.write(
                inst.rd,
                (cpu.regs.read(inst.rs1) < cpu.regs.read(inst.rs2)) as u64,
            );
        }
        OpecodeKind::OP_XOR => {
            cpu.regs
                .write(inst.rd, cpu.regs.read(inst.rs1) ^ cpu.regs.read(inst.rs2));
        }
        OpecodeKind::OP_SRL => {
            cpu.regs
                .write(inst.rd, cpu.regs.read(inst.rs1) >> cpu.regs.read(inst.rs2));
        }
        OpecodeKind::OP_SRA => {
            cpu.regs.write(
                inst.rd,
                ((cpu.regs.read(inst.rs1) as i32) >> cpu.regs.read(inst.rs2)) as u64,
            );
        }
        OpecodeKind::OP_OR => {
            cpu.regs
                .write(inst.rd, cpu.regs.read(inst.rs1) | cpu.regs.read(inst.rs2));
        }
        OpecodeKind::OP_AND => {
            cpu.regs
                .write(inst.rd, cpu.regs.read(inst.rs1) & cpu.regs.read(inst.rs2));
        }
        OpecodeKind::OP_FENCE => {
            // nop (pipeline are not yet implemented)
        }
        OpecodeKind::OP_ECALL => {
            cpu.exception(
                cpu.pc,
                match cpu.priv_lv {
                    PrivilegedLevel::User => TrapCause::UmodeEcall,
                    PrivilegedLevel::Supervisor => TrapCause::SmodeEcall,
                    PrivilegedLevel::Machine => TrapCause::MmodeEcall,
                    _ => panic!("cannot enviroment call in current privileged mode."),
                },
            );
        }
        OpecodeKind::OP_EBREAK => {
            cpu.exception(cpu.pc, TrapCause::Breakpoint);
        }
        _ => panic!("not an Base extension"),
    }

    Ok(())
}
