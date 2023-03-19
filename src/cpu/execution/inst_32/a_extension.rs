use crate::cpu::instruction::{Instruction, OpecodeKind};
use crate::cpu::{Cpu, TransAlign, TransFor, TrapCause};

fn atomic_memory_operations_32<F: Fn(u64, u64) -> u64>(
    operation: F,
    inst: &Instruction,
    cpu: &mut Cpu,
) -> Result<(), (Option<u64>, TrapCause, String)> {
    let amo_addr = cpu.trans_addr(
        TransFor::StoreAMO,
        TransAlign::Size32,
        cpu.regs.read(inst.rs1),
    )?;
    let loaded_data = cpu.bus.load32(amo_addr)?;
    let rs2_data = cpu.regs.read(inst.rs2);
    cpu.regs.write(inst.rd, loaded_data);
    cpu.bus
        .store32(amo_addr, operation(loaded_data, rs2_data))?;

    Ok(())
}

fn atomic_memory_operations_64<F: Fn(u64, u64) -> u64>(
    operation: F,
    inst: &Instruction,
    cpu: &mut Cpu,
) -> Result<(), (Option<u64>, TrapCause, String)> {
    let amo_addr = cpu.trans_addr(
        TransFor::StoreAMO,
        TransAlign::Size64,
        cpu.regs.read(inst.rs1),
    )?;
    let loaded_data = cpu.bus.load64(amo_addr)?;
    let rs2_data = cpu.regs.read(inst.rs2);
    cpu.regs.write(inst.rd, loaded_data);
    cpu.bus
        .store64(amo_addr, operation(loaded_data, rs2_data))?;

    Ok(())
}

pub fn exec(inst: &Instruction, cpu: &mut Cpu) -> Result<(), (Option<u64>, TrapCause, String)> {
    match inst.opc {
        OpecodeKind::OP_LR_W => {
            let load_addr =
                cpu.trans_addr(TransFor::Load, TransAlign::Size32, cpu.regs.read(inst.rs1))?;
            let _rl = inst.imm.unwrap() & 0x1;
            let _aq = inst.imm.unwrap() >> 1 & 0x1;
            cpu.regs.write(inst.rd, cpu.bus.load32(load_addr)?);
            cpu.reservation_set = Some(load_addr as usize);
        }
        OpecodeKind::OP_SC_W => {
            let store_addr = cpu.trans_addr(
                TransFor::StoreAMO,
                TransAlign::Size32,
                cpu.regs.read(inst.rs1),
            )?;
            // cache value == rs1 --> store rs2 to rs1 and assign zero to rd
            // cache value != rs1 --> ignore and assign non-zero to rd
            if cpu.reservation_set == Some(store_addr as usize) {
                let _rl = inst.imm.unwrap() & 0x1;
                let _aq = inst.imm.unwrap() >> 1 & 0x1;
                cpu.bus.store32(store_addr, cpu.regs.read(inst.rs2))?;
                cpu.reservation_set = None;
                cpu.regs.write(inst.rd, 0);
            } else {
                cpu.regs.write(inst.rd, 1);
            }
        }
        OpecodeKind::OP_AMOSWAP_W => {
            atomic_memory_operations_32(|_, y| y, inst, cpu)?;
        }
        OpecodeKind::OP_AMOADD_W => {
            atomic_memory_operations_32(|x, y| (x as i32 + y as i32) as u64, inst, cpu)?;
        }
        OpecodeKind::OP_AMOXOR_W => {
            atomic_memory_operations_32(|x, y| (x as i32 ^ y as i32) as u64, inst, cpu)?;
        }
        OpecodeKind::OP_AMOAND_W => {
            atomic_memory_operations_32(|x, y| (x as i32 & y as i32) as u64, inst, cpu)?;
        }
        OpecodeKind::OP_AMOOR_W => {
            atomic_memory_operations_32(|x, y| (x as i32 | y as i32) as u64, inst, cpu)?;
        }
        OpecodeKind::OP_AMOMIN_W => {
            atomic_memory_operations_32(
                |x, y| std::cmp::min(x as i32, y as i32) as u64,
                inst,
                cpu,
            )?;
        }
        OpecodeKind::OP_AMOMAX_W => {
            atomic_memory_operations_32(
                |x, y| std::cmp::max(x as i32, y as i32) as u64,
                inst,
                cpu,
            )?;
        }
        OpecodeKind::OP_AMOMINU_W => {
            atomic_memory_operations_32(
                |x, y| std::cmp::min(x as u32, y as u32) as u64,
                inst,
                cpu,
            )?;
        }
        OpecodeKind::OP_AMOMAXU_W => {
            atomic_memory_operations_32(
                |x, y| std::cmp::max(x as u32, y as u32) as u64,
                inst,
                cpu,
            )?;
        }
        OpecodeKind::OP_LR_D => {
            let load_addr =
                cpu.trans_addr(TransFor::Load, TransAlign::Size64, cpu.regs.read(inst.rs1))?;
            let _rl = inst.imm.unwrap() & 0x1;
            let _aq = inst.imm.unwrap() >> 1 & 0x1;
            cpu.regs.write(inst.rd, cpu.bus.load64(load_addr)?);
            cpu.reservation_set = Some(load_addr as usize);
        }
        OpecodeKind::OP_SC_D => {
            let store_addr = cpu.trans_addr(
                TransFor::StoreAMO,
                TransAlign::Size64,
                cpu.regs.read(inst.rs1),
            )?;
            // cache value == rs1 --> store rs2 to rs1 and assign zero to rd
            // cache value != rs1 --> ignore and assign non-zero to rd
            if cpu.reservation_set == Some(store_addr as usize) {
                let _rl = inst.imm.unwrap() & 0x1;
                let _aq = inst.imm.unwrap() >> 1 & 0x1;
                cpu.bus.store64(store_addr, cpu.regs.read(inst.rs2))?;
                cpu.reservation_set = None;
                cpu.regs.write(inst.rd, 0);
            } else {
                cpu.regs.write(inst.rd, 1);
            }
        }
        OpecodeKind::OP_AMOSWAP_D => {
            atomic_memory_operations_64(|_, y| y, inst, cpu)?;
        }
        OpecodeKind::OP_AMOADD_D => {
            atomic_memory_operations_64(|x, y| (x as i64 + y as i64) as u64, inst, cpu)?;
        }
        OpecodeKind::OP_AMOXOR_D => {
            atomic_memory_operations_64(|x, y| (x as i64 ^ y as i64) as u64, inst, cpu)?;
        }
        OpecodeKind::OP_AMOAND_D => {
            atomic_memory_operations_64(|x, y| (x as i64 & y as i64) as u64, inst, cpu)?;
        }
        OpecodeKind::OP_AMOOR_D => {
            atomic_memory_operations_64(|x, y| (x as i64 | y as i64) as u64, inst, cpu)?;
        }
        OpecodeKind::OP_AMOMIN_D => {
            atomic_memory_operations_64(
                |x, y| std::cmp::min(x as i64, y as i64) as u64,
                inst,
                cpu,
            )?;
        }
        OpecodeKind::OP_AMOMAX_D => {
            atomic_memory_operations_64(
                |x, y| std::cmp::max(x as i64, y as i64) as u64,
                inst,
                cpu,
            )?;
        }
        OpecodeKind::OP_AMOMINU_D => {
            atomic_memory_operations_64(std::cmp::min, inst, cpu)?;
        }
        OpecodeKind::OP_AMOMAXU_D => {
            atomic_memory_operations_64(std::cmp::max, inst, cpu)?;
        }
        _ => panic!("not an A extension"),
    }

    Ok(())
}
