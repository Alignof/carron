use crate::cpu::CPU;
use crate::cpu::instruction::{Instruction, OpecodeKind};

pub fn exec(inst: &Instruction, cpu: &mut CPU) {
    match inst.opc {
        OpecodeKind::OP_MUL    => {
            cpu.regs.write(inst.rd, cpu.regs.read(inst.rs1) * cpu.regs.read(inst.rs2));
        },
        OpecodeKind::OP_MULH   => {
            cpu.regs.write(inst.rd, ((cpu.regs.read(inst.rs1) as i64 * cpu.regs.read(inst.rs2) as i64) >> 32) as i32);
        },
        OpecodeKind::OP_MULHSU => {
            cpu.regs.write(inst.rd, ((cpu.regs.read(inst.rs1) as i64 * cpu.regs.read(inst.rs2) as u64 as i64) >> 32) as u32 as i32);
        },
        OpecodeKind::OP_MULHU  => {
            cpu.regs.write(inst.rd, ((cpu.regs.read(inst.rs1) as u64 * cpu.regs.read(inst.rs2) as u64) >> 32) as u32 as i32);
        },
        OpecodeKind::OP_DIV    => {
            cpu.regs.write(inst.rd, cpu.regs.read(inst.rs1) / cpu.regs.read(inst.rs2));
        },
        OpecodeKind::OP_DIVU   => {
            cpu.regs.write(inst.rd, (cpu.regs.read(inst.rs1) as u32 / cpu.regs.read(inst.rs2) as u32) as i32);
        },
        OpecodeKind::OP_REM    => {
            cpu.regs.write(inst.rd, cpu.regs.read(inst.rs1) % cpu.regs.read(inst.rs2));
        },
        OpecodeKind::OP_REMU   => {
            cpu.regs.write(inst.rd, (cpu.regs.read(inst.rs1) as u32 % cpu.regs.read(inst.rs2) as u32) as i32);
        },
        _ => panic!("not an M extension"),
    }
}

