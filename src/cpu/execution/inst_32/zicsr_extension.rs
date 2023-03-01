use crate::cpu::csr::{CSRname, Xstatus};
use crate::cpu::instruction::{Instruction, OpecodeKind};
use crate::cpu::{Cpu, PrivilegedLevel, TransAlign, TransFor, TrapCause};

fn check_accessible(cpu: &mut Cpu, dist: usize) -> Result<(), (Option<u64>, TrapCause, String)> {
    let inst_addr = cpu.trans_addr(TransFor::Fetch, TransAlign::Size8, cpu.pc())?;
    let invalid_instruction = Some(cpu.bus.load_u32(inst_addr).expect("get instruction failed"));

    if dist >= 4096 {
        return Err((
            invalid_instruction,
            TrapCause::IllegalInst,
            format!("csr size is 4096, but you accessed {dist}"),
        ));
    }

    match cpu.priv_lv {
        PrivilegedLevel::User => {
            if (0x100..=0x180).contains(&dist) || (0x300..=0x344).contains(&dist) {
                return Err((
                    invalid_instruction,
                    TrapCause::IllegalInst,
                    format!("You are in User mode but accessed {dist}"),
                ));
            }
        }
        PrivilegedLevel::Supervisor => {
            if (0x300..=0x344).contains(&dist) {
                return Err((
                    invalid_instruction,
                    TrapCause::IllegalInst,
                    format!("You are in Supervisor mode but accessed {dist}"),
                ));
            }

            if dist == CSRname::satp as usize
                && cpu
                    .csrs
                    .read_xstatus(PrivilegedLevel::Machine, Xstatus::TVM)
                    == 1
            {
                return Err((
                    invalid_instruction,
                    TrapCause::IllegalInst,
                    "mstatus.TVM == 1 but accessed satp".to_string(),
                ));
            }
        }
        _ => (),
    }

    if (0xc00..=0xc1f).contains(&dist) {
        let ctren = cpu.csrs.read(CSRname::mcounteren.wrap())?;
        if ctren >> (dist - 0xc00) & 0x1 == 1 {
            return Err((
                invalid_instruction,
                TrapCause::IllegalInst,
                "mcounteren bit is clear, but attempt reading".to_string(),
            ));
        }
    }

    match dist {
        0x000..=0x005 | 0x040..=0x044 => Ok(()),
        0x100 | 0x102..=0x106 => Ok(()),
        0x140..=0x144 => Ok(()),
        0x180 => Ok(()),
        0x300..=0x306 | 0x320..=0x33f | 0x340..=0x344 => Ok(()),
        0x3a0..=0x3a3 | 0x3b0..=0x3bf => Ok(()),
        0x7a0..=0x7a3 | 0x7b0..=0x7b3 => Ok(()),
        0xb00..=0xb1f | 0xb80..=0xb9f => Ok(()),
        0xc00..=0xc1f | 0xc80..=0xc9f => Ok(()),
        0xf11..=0xf14 => Ok(()),
        _ => Err((
            Some(dist as u64),
            TrapCause::IllegalInst,
            format!("unknown CSR number: {dist}"),
        )),
    }
}

pub fn exec(inst: &Instruction, cpu: &mut Cpu) -> Result<(), (Option<u64>, TrapCause, String)> {
    check_accessible(cpu, inst.rs2.unwrap())?;

    match inst.opc {
        OpecodeKind::OP_CSRRW => {
            let rs1 = cpu.regs.read(inst.rs1);
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2)?);
            cpu.csrs.write(inst.rs2, rs1);
        }
        OpecodeKind::OP_CSRRS => {
            let rs1 = cpu.regs.read(inst.rs1);
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2)?);
            cpu.csrs.bitset(inst.rs2, rs1);
        }
        OpecodeKind::OP_CSRRC => {
            let rs1 = cpu.regs.read(inst.rs1);
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2)?);
            cpu.csrs.bitclr(inst.rs2, rs1);
        }
        OpecodeKind::OP_CSRRWI => {
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2)?);
            cpu.csrs.write(inst.rs2, inst.rs1.unwrap() as u64);
        }
        OpecodeKind::OP_CSRRSI => {
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2)?);
            cpu.csrs.bitset(inst.rs2, inst.rs1.unwrap() as u64);
        }
        OpecodeKind::OP_CSRRCI => {
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2)?);
            cpu.csrs.bitclr(inst.rs2, inst.rs1.unwrap() as u64);
        }
        _ => panic!("not an Zicsr extension"),
    }

    Ok(())
}
