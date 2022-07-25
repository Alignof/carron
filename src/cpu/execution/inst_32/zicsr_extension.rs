use crate::cpu::{CPU, PrivilegedLevel, TrapCause};
use crate::cpu::csr::{CSRname, Xstatus};
use crate::cpu::instruction::{Instruction, OpecodeKind};

fn check_accessible(cpu: &mut CPU, dist: usize) -> Result<(), (Option<i32>, TrapCause, String)> {
    let invalid_instruction = Some(cpu.bus.load32(cpu.pc)?);
    if dist >= 4096 {
        return Err((
                invalid_instruction,
                TrapCause::IllegalInst,
                format!("csr size is 4096, but you accessed {}", dist)
        ));
    }

    match cpu.priv_lv {
        PrivilegedLevel::User => {
            if (0x100 <= dist && dist <= 0x180) || (0x300 <= dist && dist <= 0x344) {
                    return Err((
                        invalid_instruction,
                        TrapCause::IllegalInst,
                        format!("You are in User mode but accessed {}", dist)
                    ));
                }
        },
        PrivilegedLevel::Supervisor => {
            if 0x300 <= dist && dist <= 0x344 {
                return Err((
                    invalid_instruction,
                    TrapCause::IllegalInst,
                    format!("You are in Supervisor mode but accessed {}", dist)
                ));
            }

            if dist == CSRname::satp as usize && cpu.csrs.read_xstatus(PrivilegedLevel::Machine, Xstatus::TVM) == 1 {
                return Err((
                    invalid_instruction,
                    TrapCause::IllegalInst,
                    format!("mstatsu.TVM == 1 but accessed satp")
                ));
            }
        },
        _ => (),
    }

    if 0xc00 <= dist && dist <= 0xc1f {
        let ctren = cpu.csrs.read(CSRname::mcounteren.wrap())?;
        if ctren >> (dist - 0xc00) & 0x1 == 1 {
            return Err((
                invalid_instruction,
                TrapCause::IllegalInst,
                "mcounteren bit is clear, but attempt reading".to_string()
            ));
        }
    }

    Ok(())
}

pub fn exec(inst: &Instruction, cpu: &mut CPU) -> Result<(), (Option<i32>, TrapCause, String)> {
    check_accessible(cpu, inst.rs2.unwrap())?;

    match inst.opc {
        OpecodeKind::OP_CSRRW => {
            let rs1 = cpu.regs.read(inst.rs1) as i32;
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2)? as i32);
            cpu.csrs.write(inst.rs2, rs1);
        },
        OpecodeKind::OP_CSRRS => {
            let rs1 = cpu.regs.read(inst.rs1) as i32;
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2)? as i32);
            cpu.csrs.bitset(inst.rs2, rs1);
        },
        OpecodeKind::OP_CSRRC => {
            let rs1 = cpu.regs.read(inst.rs1) as i32;
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2)? as i32);
            cpu.csrs.bitclr(inst.rs2, rs1);
        },
        OpecodeKind::OP_CSRRWI => {
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2)? as i32);
            cpu.csrs.write(inst.rs2, inst.rs1.unwrap() as i32);
        },
        OpecodeKind::OP_CSRRSI => {
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2)? as i32);
            cpu.csrs.bitset(inst.rs2, inst.rs1.unwrap() as i32);
        },
        OpecodeKind::OP_CSRRCI => {
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2)? as i32);
            cpu.csrs.bitclr(inst.rs2, inst.rs1.unwrap() as i32);
        },
        _ => panic!("not an Zicsr extension"),
    }

    if inst.rs2 == CSRname::misa.wrap() && cpu.csrs.read(CSRname::misa.wrap())? >> 2 & 0x1 == 0 {
        if cpu.pc % 4 != 0 {
            cpu.csrs.bitset(inst.rs2, 0b100);
        }
    }

    Ok(())
}

