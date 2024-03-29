use crate::cpu::instruction::{Instruction, OpecodeKind};
use crate::cpu::{Cpu, TrapCause};
use crate::Isa;

pub fn exec(inst: &Instruction, cpu: &mut Cpu) -> Result<(), (Option<u64>, TrapCause, String)> {
    let rs1 = cpu.regs.read(inst.rs1);
    let rs2 = cpu.regs.read(inst.rs2);
    let mulhu_64 = |x: u64, y: u64| -> u64 {
        let xl: u64 = x & 0xFFFF_FFFF;
        let xh: u64 = x >> 32;
        let yl: u64 = y & 0xFFFF_FFFF;
        let yh: u64 = y >> 32;

        let t: u64 = xh * yl + ((xl * yl) >> 32);
        let tl: u64 = t & 0xFFFF_FFFF;
        let th: u64 = t >> 32;

        let u: u64 = xl * yh + tl;
        let v: u64 = xh * yh + th + (u >> 32);

        (v >> 32) << 32 | v
    };

    match inst.opc {
        OpecodeKind::OP_MUL => {
            cpu.regs.write(inst.rd, rs1 * rs2);
        }
        OpecodeKind::OP_MULH => {
            cpu.regs.write(
                inst.rd,
                match *cpu.isa {
                    Isa::Rv32 => ((rs1 as i32 as i64 * rs2 as i32 as i64) >> 32) as u64,
                    Isa::Rv64 => {
                        let rs1 = rs1 as i64;
                        let rs2 = rs2 as i64;
                        if (rs1 < 0) == (rs2 < 0) {
                            mulhu_64(rs1.unsigned_abs(), rs2.unsigned_abs())
                        } else {
                            !mulhu_64(rs1.unsigned_abs(), rs2.unsigned_abs())
                                + (rs1 * rs2 == 0) as u64
                        }
                    }
                },
            );
        }
        OpecodeKind::OP_MULHSU => {
            cpu.regs.write(
                inst.rd,
                match *cpu.isa {
                    Isa::Rv32 => ((rs1 as i32 as i64 * rs2 as i64) >> 32) as u64,
                    Isa::Rv64 => {
                        let rs1 = rs1 as i64;
                        if rs1 < 0 {
                            !mulhu_64(rs1.unsigned_abs(), rs2) + (rs1 as u64 * rs2 == 0) as u64
                        } else {
                            mulhu_64(rs1.unsigned_abs(), rs2)
                        }
                    }
                },
            );
        }
        OpecodeKind::OP_MULHU => {
            cpu.regs.write(
                inst.rd,
                match *cpu.isa {
                    Isa::Rv32 => (rs1 * rs2) >> 32,
                    Isa::Rv64 => mulhu_64(rs1, rs2),
                },
            );
        }
        OpecodeKind::OP_DIV => {
            if rs2 == 0 {
                cpu.regs.write(inst.rd, u64::MAX); // -1
            } else if rs1 as i64 == i64::MIN && rs2 as i64 == -1 {
                cpu.regs.write(inst.rd, rs1);
            } else {
                cpu.regs.write(
                    inst.rd,
                    match *cpu.isa {
                        Isa::Rv32 => (rs1 as i32 as i64 / rs2 as i32 as i64) as u64,
                        Isa::Rv64 => (rs1 as i64 / rs2 as i64) as u64,
                    },
                )
            }
        }
        OpecodeKind::OP_DIVU => {
            if rs2 == 0 {
                cpu.regs.write(inst.rd, (2i32.pow(32) - 1) as u64);
            } else {
                cpu.regs.write(inst.rd, rs1 / rs2);
            }
        }
        OpecodeKind::OP_REM => {
            if rs2 == 0 {
                cpu.regs.write(inst.rd, rs1);
            } else if rs1 as i64 == i64::MIN && rs2 as i64 == -1 {
                cpu.regs.write(inst.rd, 0);
            } else {
                cpu.regs
                    .write(inst.rd, (rs1 as i32 as i64 % rs2 as i32 as i64) as u64);
            }
        }
        OpecodeKind::OP_REMU => {
            if rs2 == 0 {
                cpu.regs.write(inst.rd, rs1);
            } else {
                cpu.regs.write(inst.rd, rs1 % rs2);
            }
        }
        OpecodeKind::OP_MULW => {
            cpu.regs
                .write(inst.rd, (rs1 as i32 * rs2 as i32) as i64 as u64);
        }
        OpecodeKind::OP_DIVW => {
            let rs1 = rs1 as i32;
            let rs2 = rs2 as i32;
            if rs2 == 0 {
                cpu.regs.write(inst.rd, u64::MAX); // -1
            } else if rs1 == i32::MIN && rs2 == -1 {
                cpu.regs.write(inst.rd, rs1 as u64);
            } else {
                cpu.regs.write(inst.rd, (rs1 / rs2) as u64)
            }
        }
        OpecodeKind::OP_REMW => {
            let rs1 = rs1 as i32;
            let rs2 = rs2 as i32;
            if rs2 == 0 {
                cpu.regs.write(inst.rd, rs1 as u64);
            } else if rs1 == i32::MIN && rs2 == -1 {
                cpu.regs.write(inst.rd, 0);
            } else {
                cpu.regs.write(inst.rd, (rs1 % rs2) as u64);
            }
        }
        OpecodeKind::OP_DIVUW => {
            if rs2 == 0 {
                cpu.regs.write(inst.rd, (2i32.pow(32) - 1) as u64);
            } else {
                cpu.regs
                    .write(inst.rd, (rs1 as u32 / rs2 as u32) as i32 as u64);
            }
        }
        OpecodeKind::OP_REMUW => {
            if rs2 == 0 {
                cpu.regs.write(inst.rd, rs1 as i32 as u64);
            } else {
                cpu.regs
                    .write(inst.rd, (rs1 as u32 % rs2 as u32) as i32 as u64);
            }
        }
        _ => panic!("not an M extension"),
    }

    Ok(())
}
