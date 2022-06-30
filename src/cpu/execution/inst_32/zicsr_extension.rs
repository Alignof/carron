use crate::cpu::{CPU, TrapCause};
use crate::cpu::csr::{CSRname};
use crate::cpu::instruction::{Instruction, OpecodeKind};

pub fn exec(inst: &Instruction, cpu: &mut CPU) -> Result<(), String> {
    if Some(0xc00) <= inst.rs2 && inst.rs2 <= Some(0xc1f) {
        let ctren = cpu.csrs.read(CSRname::mcounteren.wrap());
        if ctren >> (inst.rs2.unwrap() - 0xc00) & 0x1 == 1 {
            cpu.exception(cpu.pc as i32, TrapCause::IllegalInst);
            return Err("CSR error".to_string());
        }
    }

    match inst.opc {
        OpecodeKind::OP_CSRRW => {
            let rs1 = cpu.regs.read(inst.rs1) as i32;
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2) as i32);
            cpu.csrs.write(inst.rs2, rs1);
        },
        OpecodeKind::OP_CSRRS => {
            let rs1 = cpu.regs.read(inst.rs1) as i32;
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2) as i32);
            cpu.csrs.bitset(inst.rs2, rs1);
        },
        OpecodeKind::OP_CSRRC => {
            let rs1 = cpu.regs.read(inst.rs1) as i32;
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2) as i32);
            cpu.csrs.bitclr(inst.rs2, rs1);
        },
        OpecodeKind::OP_CSRRWI => {
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2) as i32);
            cpu.csrs.write(inst.rs2, inst.rs1.unwrap() as i32);
        },
        OpecodeKind::OP_CSRRSI => {
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2) as i32);
            cpu.csrs.bitset(inst.rs2, inst.rs1.unwrap() as i32);
        },
        OpecodeKind::OP_CSRRCI => {
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2) as i32);
            cpu.csrs.bitclr(inst.rs2, inst.rs1.unwrap() as i32);
        },
        _ => panic!("not an Zicsr extension"),
    }

    Ok(())
}

