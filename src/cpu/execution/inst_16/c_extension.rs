use crate::cpu::instruction::{Instruction, OpecodeKind};
use crate::cpu::{Cpu, TransAlign, TransFor, TrapCause};

pub fn exec(inst: &Instruction, cpu: &mut Cpu) -> Result<(), (Option<u64>, TrapCause, String)> {
    const INST_SIZE: u64 = 2;
    const REG_SP: usize = 2;
    const REG_LINK: usize = 1;

    match inst.opc {
        OpecodeKind::OP_C_LI => {
            cpu.regs.write(inst.rd, inst.imm.unwrap() as u64);
        }
        OpecodeKind::OP_C_LW => {
            let load_addr = cpu.trans_addr(
                TransFor::Load,
                TransAlign::Size32,
                (cpu.regs.read(inst.rs1) as i32 + inst.imm.unwrap()) as u64,
            )?;
            cpu.regs.write(inst.rd, cpu.bus.load32(load_addr)?);
        }
        OpecodeKind::OP_C_LWSP => {
            let load_addr = cpu.trans_addr(
                TransFor::Load,
                TransAlign::Size32,
                (cpu.regs.read(Some(REG_SP)) as i32 + inst.imm.unwrap()) as u64,
            )?;
            cpu.regs.write(inst.rd, cpu.bus.load32(load_addr)?);
        }
        OpecodeKind::OP_C_LUI => {
            cpu.regs.write(inst.rd, inst.imm.unwrap() as u64);
        }
        OpecodeKind::OP_C_SW => {
            let store_addr = cpu.trans_addr(
                TransFor::StoreAMO,
                TransAlign::Size32,
                (cpu.regs.read(inst.rs1) as i32 + inst.imm.unwrap()) as u64,
            )?;
            cpu.bus.store32(store_addr, cpu.regs.read(inst.rs2))?;
        }
        OpecodeKind::OP_C_SWSP => {
            let store_addr = cpu.trans_addr(
                TransFor::StoreAMO,
                TransAlign::Size32,
                (cpu.regs.read(Some(REG_SP)) as i32 + inst.imm.unwrap()) as u64,
            )?;
            cpu.bus.store32(store_addr, cpu.regs.read(inst.rs2))?;
        }
        OpecodeKind::OP_C_SLLI => {
            cpu.regs
                .write(inst.rd, cpu.regs.read(inst.rs1) << inst.imm.unwrap() as u64);
        }
        OpecodeKind::OP_C_SRLI => {
            cpu.regs
                .write(inst.rd, cpu.regs.read(inst.rs1) >> inst.imm.unwrap() as u64);
        }
        OpecodeKind::OP_C_SRAI => {
            cpu.regs.write(
                inst.rd,
                ((cpu.regs.read(inst.rs1) as i32) >> inst.imm.unwrap()) as u64,
            );
        }
        OpecodeKind::OP_C_ADD => {
            cpu.regs
                .write(inst.rd, cpu.regs.read(inst.rs1) + cpu.regs.read(inst.rs2));
        }
        OpecodeKind::OP_C_ADDI4SPN => {
            cpu.regs.write(
                inst.rd,
                cpu.regs.read(Some(REG_SP)) + inst.imm.unwrap() as u64,
            );
        }
        OpecodeKind::OP_C_ADDI => {
            cpu.regs
                .write(inst.rd, cpu.regs.read(inst.rd) + inst.imm.unwrap() as u64);
        }
        OpecodeKind::OP_C_ADDI16SP => {
            cpu.regs.write(
                Some(REG_SP),
                cpu.regs.read(Some(REG_SP)) + inst.imm.unwrap() as u64,
            );
        }
        OpecodeKind::OP_C_ANDI => {
            cpu.regs
                .write(inst.rd, cpu.regs.read(inst.rd) & inst.imm.unwrap() as u64);
        }
        OpecodeKind::OP_C_SUB => {
            cpu.regs
                .write(inst.rd, cpu.regs.read(inst.rs1) - cpu.regs.read(inst.rs2));
        }
        OpecodeKind::OP_C_XOR => {
            cpu.regs
                .write(inst.rd, cpu.regs.read(inst.rs1) ^ cpu.regs.read(inst.rs2));
        }
        OpecodeKind::OP_C_OR => {
            cpu.regs
                .write(inst.rd, cpu.regs.read(inst.rs1) | cpu.regs.read(inst.rs2));
        }
        OpecodeKind::OP_C_AND => {
            cpu.regs
                .write(inst.rd, cpu.regs.read(inst.rs1) & cpu.regs.read(inst.rs2));
        }
        OpecodeKind::OP_C_J => {
            cpu.pc += inst.imm.unwrap() as u64;
        }
        OpecodeKind::OP_C_JAL => {
            cpu.regs.write(Some(1), cpu.pc + INST_SIZE);
            cpu.add2pc(inst.imm.unwrap() as u64);
        }
        OpecodeKind::OP_C_JALR => {
            // calc next_pc before updated
            let next_pc = cpu.pc + INST_SIZE;
            // setting the least-significant bit of
            // the result to zero                ->vvvvvv
            cpu.update_pc(cpu.regs.read(inst.rs1) & !0x1);
            cpu.regs.write(Some(REG_LINK), next_pc);
        }
        OpecodeKind::OP_C_BEQZ => {
            if cpu.regs.read(inst.rs1) == 0 {
                cpu.add2pc(inst.imm.unwrap() as u64);
            }
        }
        OpecodeKind::OP_C_BNEZ => {
            if cpu.regs.read(inst.rs1) != 0 {
                cpu.add2pc(inst.imm.unwrap() as u64);
            }
        }
        OpecodeKind::OP_C_JR => {
            cpu.update_pc(cpu.regs.read(inst.rs1));
        }
        OpecodeKind::OP_C_MV => {
            cpu.regs.write(inst.rd, cpu.regs.read(inst.rs2));
        }
        OpecodeKind::OP_C_EBREAK => {
            panic!("not yet implemented: OP_C_EBREAK");
        }
        OpecodeKind::OP_C_NOP => { /* NOP */ }
        _ => panic!("not a compressed Instruction"),
    }

    Ok(())
}

#[cfg(test)]
mod exe_16 {
    use crate::bus;
    use crate::cpu::execution::inst_16::c_extension::exec;
    use crate::cpu::instruction::{Instruction, OpecodeKind::*};
    use crate::cpu::{csr, mmu, reg, Cpu, PrivilegedLevel};
    use crate::{elfload, Isa};
    use std::collections::HashSet;

    #[test]
    fn c_extension_test() {
        let dummy_elf =
            elfload::ElfLoader::try_new("./HelloWorld").expect("creating dummy_elf failed");
        let bus = bus::Bus::new(dummy_elf, Isa::Rv32);
        let mut cpu: Cpu = Cpu {
            pc: bus.mrom.base_addr,
            bus,
            regs: reg::Register::new(),
            csrs: csr::CSRs::new().init(),
            mmu: mmu::Mmu::new(),
            reservation_set: HashSet::new(),
            priv_lv: PrivilegedLevel::Machine,
        };

        exec(
            &Instruction {
                opc: OP_C_LI,
                rd: Some(10),
                rs1: None,
                rs2: None,
                imm: Some(42),
            },
            &mut cpu,
        )
        .unwrap();
        assert_eq!(cpu.regs.read(Some(10)), 42);
    }
}
