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

    // riscv-privileged-20190608.pdf p7
    let csrs_ranges = vec![
        0x000..=0x0ff,
        0x400..=0x4ff,
        0x800..=0x8ff,
        0xc00..=0xc7f,
        0xc80..=0xcbf,
        0xcc0..=0xcff,
        0x100..=0x1ff,
        0x500..=0x57f,
        0x580..=0x5bf,
        0x5c0..=0x5ff,
        0x900..=0x97f,
        0x980..=0x9bf,
        0x9c0..=0x9ff,
        0xd00..=0xd7f,
        //0xd80..=0xdbf,
        0xdc0..=0xdff,
        0x200..=0x2ff,
        0x600..=0x67f,
        0x680..=0x6bf,
        0x6c0..=0x6ff,
        0xa00..=0xa7f,
        0xa80..=0xabf,
        0xac0..=0xaff,
        0xe00..=0xe7f,
        0xe80..=0xebf,
        0xec0..=0xeff,
        //0x300..=0x3ff,
        0x300..=0x30b, // disable mstateen0(0x30c)
        0x30d..=0x3bf, // disable pmpaddr16~
        0x700..=0x77f,
        0x780..=0x79f,
        0x7a0..=0x7af,
        0x7b0..=0x7bf,
        0x7c0..=0x7ff,
        0xb00..=0xb7f,
        0xb80..=0xbbf,
        0xbc0..=0xbff,
        0xf00..=0xf7f,
        //0xf80..=0xfbf,
        0xfc0..=0xfff,
    ];

    if csrs_ranges.iter().any(|x| x.contains(&dist)) {
        match dist {
            // == depends on privilege ==
            // scounteren(0x106) only allow higher
            0x106 => match cpu.priv_lv {
                PrivilegedLevel::User => Err((
                    Some(dist as u64),
                    TrapCause::IllegalInst,
                    format!("unknown CSR number: {dist}"),
                )),
                _ => Ok(()),
            },
            // stimecmp(0x14d) only supervisor
            0x14d => match cpu.priv_lv {
                PrivilegedLevel::Supervisor => Ok(()),
                _ => Err((
                    Some(dist as u64),
                    TrapCause::IllegalInst,
                    format!("unknown CSR number: {dist}"),
                )),
            },
            _ => Ok(()),
        }
    } else {
        // out of range
        Err((
            Some(dist as u64),
            TrapCause::IllegalInst,
            format!("unknown CSR number: {dist}"),
        ))
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
