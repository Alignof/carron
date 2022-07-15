use crate::cpu::{CPU, TrapCause};
use crate::cpu::csr::CSRname;
use crate::cpu::instruction::{Instruction, OpecodeKind};

pub fn exec(inst: &Instruction, cpu: &mut CPU) -> Result<(), (Option<i32>, TrapCause, String)> {
    cpu.csrs.check_accessible(cpu.priv_lv, inst.rs2.unwrap())?;
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

