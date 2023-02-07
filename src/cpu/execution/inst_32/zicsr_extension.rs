use crate::cpu::csr::{CSRname, Xstatus};
use crate::cpu::instruction::{Instruction, OpecodeKind};
use crate::cpu::{Cpu, PrivilegedLevel, TransAlign, TransFor, TrapCause};
use crate::Isa;

fn check_accessible(cpu: &mut Cpu, dist: usize) -> Result<(), (Option<u64>, TrapCause, String)> {
    let inst_addr = cpu.trans_addr(TransFor::Fetch, TransAlign::Size8, cpu.pc)?;
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

    Ok(())
}

fn check_warl(cpu: &mut Cpu, dst: usize, original: u64) {
    const MISA: usize = CSRname::misa as usize;
    const MSTATUS: usize = CSRname::mstatus as usize;

    match dst {
        MISA => {
            if cpu.csrs.read(CSRname::misa.wrap()).unwrap() >> 2 & 0x1 == 0 && cpu.pc % 4 != 0 {
                cpu.csrs.bitset(Some(dst), 0b100);
            }
        }
        MSTATUS => match *cpu.isa {
            Isa::Rv32 => (),
            Isa::Rv64 => {
                if cpu
                    .csrs
                    .read_xstatus(PrivilegedLevel::Machine, Xstatus::UXL)
                    == 0b00
                {
                    cpu.csrs.bitset(Some(dst), ((original >> 32) & 0b11) << 32);
                }
            }
        },
        _ => (),
    }
}

pub fn exec(inst: &Instruction, cpu: &mut Cpu) -> Result<(), (Option<u64>, TrapCause, String)> {
    check_accessible(cpu, inst.rs2.unwrap())?;
    let original = cpu.csrs.read(inst.rs2)?;

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

    check_warl(cpu, inst.rs2.unwrap(), original);
    Ok(())
}
