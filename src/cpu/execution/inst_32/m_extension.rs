use crate::cpu::{CPU, TrapCause};
use crate::cpu::instruction::{Instruction, OpecodeKind};

pub fn exec(inst: &Instruction, cpu: &mut CPU) -> Result<(), (Option<i32>, TrapCause, String)> {
    let rs1 = cpu.regs.read(inst.rs1);
    let rs2 = cpu.regs.read(inst.rs2);

    match inst.opc {
        OpecodeKind::OP_MUL => {
            cpu.regs.write(inst.rd, rs1 * rs2);
        },
        OpecodeKind::OP_MULH => {
            cpu.regs.write(inst.rd, ((rs1 as i64 * rs2 as i64) >> 32) as i32);
        },
        OpecodeKind::OP_MULHSU => {
            cpu.regs.write(inst.rd, ((rs1 as i32 as i64 * rs2 as u32 as u64 as i64) >> 32) as u32 as i32);
        },
        OpecodeKind::OP_MULHU => {
            cpu.regs.write(inst.rd, ((rs1 as u32 as u64 * rs2 as u32 as u64) >> 32) as u32 as i32);
        },
        OpecodeKind::OP_DIV => {
            if rs2 == 0 {
                cpu.regs.write(inst.rd, -1);
            } else {
                cpu.regs.write(inst.rd, (rs1 as i64 / rs2 as i64) as i32);
            }
        },
        OpecodeKind::OP_DIVU => {
            if rs2 == 0 {
                cpu.regs.write(inst.rd, 2i32.pow(32) - 1);
            } else {
                cpu.regs.write(inst.rd, (rs1 as u32 as u64 / rs2 as u32 as u64) as i32);
            }
        },
        OpecodeKind::OP_REM => {
            if rs2 == 0 {
                cpu.regs.write(inst.rd, rs1);
            } else {
                cpu.regs.write(inst.rd, (rs1 as i64 % rs2 as i64) as i32);
            }
        },
        OpecodeKind::OP_REMU => {
            if rs2 == 0 {
                cpu.regs.write(inst.rd, rs1);
            } else {
                cpu.regs.write(inst.rd, (rs1 as u64 % rs2 as u64) as i32);
            }
        },
        _ => panic!("not an M extension"),
    }

    Ok(())
}

