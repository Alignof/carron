use crate::cpu::CPU;
use crate::cpu::instruction::{Instruction, OpecodeKind};

pub fn exec(inst: &Instruction, cpu: &mut CPU) {
    match inst.opc {
        OpecodeKind::OP_CSRRW => {
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2) as i32);
            cpu.csrs.write(inst.rs2, cpu.regs.read(inst.rs1));
        },
        OpecodeKind::OP_CSRRS => {
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2) as i32);
            cpu.csrs.bitset(inst.rs2, cpu.regs.read(inst.rs1));
        },
        OpecodeKind::OP_CSRRC => {
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2) as i32);
            cpu.csrs.bitclr(inst.rs2, cpu.regs.read(inst.rs1));
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
}

