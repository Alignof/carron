use crate::cpu::instruction::{Instruction, OpecodeKind};
use crate::cpu::{Cpu, TransAlign, TransFor, TrapCause};

pub fn exec(inst: &Instruction, cpu: &mut Cpu) -> Result<(), (Option<u64>, TrapCause, String)> {
    match inst.opc {
        OpecodeKind::OP_LR_W => {
            let load_addr =
                cpu.trans_addr(TransFor::Load, TransAlign::Size32, cpu.regs.read(inst.rs1))?;
            let _rl = inst.imm.unwrap() & 0x1;
            let _aq = inst.imm.unwrap() >> 1 & 0x1;
            cpu.regs.write(inst.rd, cpu.bus.load32(load_addr)?);
            cpu.reservation_set.insert(load_addr as usize);
        }
        OpecodeKind::OP_SC_W => {
            let store_addr = cpu.trans_addr(
                TransFor::StoreAMO,
                TransAlign::Size32,
                cpu.regs.read(inst.rs1),
            )?;
            // cache value == rs1 --> store rs2 to rs1 and assign zero to rd
            // cache value != rs1 --> ignore and assign non-zero to rd
            if cpu.reservation_set.contains(&(store_addr as usize)) {
                let _rl = inst.imm.unwrap() & 0x1;
                let _aq = inst.imm.unwrap() >> 1 & 0x1;
                cpu.bus.store32(store_addr, cpu.regs.read(inst.rs2))?;
                cpu.reservation_set.remove(&(store_addr as usize));
                cpu.regs.write(inst.rd, 0);
            } else {
                cpu.regs.write(inst.rd, 1);
            }

            cpu.reservation_set.clear();
        }
        OpecodeKind::OP_AMOSWAP_W => {
            let load_addr =
                cpu.trans_addr(TransFor::Load, TransAlign::Size32, cpu.regs.read(inst.rs1))?;
            cpu.regs.write(inst.rd, cpu.bus.load32(load_addr)?);
            let store_addr = cpu
                .trans_addr(
                    TransFor::StoreAMO,
                    TransAlign::Size32,
                    cpu.regs.read(inst.rs1),
                )
                .expect("transition address failed in AMO");
            cpu.bus.store32(store_addr, cpu.regs.read(inst.rs2))?;
        }
        OpecodeKind::OP_AMOADD_W => {
            let load_addr =
                cpu.trans_addr(TransFor::Load, TransAlign::Size32, cpu.regs.read(inst.rs1))?;
            cpu.regs.write(inst.rd, cpu.bus.load32(load_addr)?);
            let store_addr = cpu
                .trans_addr(
                    TransFor::StoreAMO,
                    TransAlign::Size32,
                    cpu.regs.read(inst.rs1),
                )
                .expect("transition address failed in AMO");
            cpu.bus.store32(
                store_addr,
                (cpu.regs.read(inst.rd) as i32 + cpu.regs.read(inst.rs2) as i32) as u64,
            )?;
        }
        OpecodeKind::OP_AMOXOR_W => {
            let load_addr =
                cpu.trans_addr(TransFor::Load, TransAlign::Size32, cpu.regs.read(inst.rs1))?;
            cpu.regs.write(inst.rd, cpu.bus.load32(load_addr)?);
            let store_addr = cpu
                .trans_addr(
                    TransFor::StoreAMO,
                    TransAlign::Size32,
                    cpu.regs.read(inst.rs1),
                )
                .expect("transition address failed in AMO");
            cpu.bus
                .store32(store_addr, cpu.regs.read(inst.rd) ^ cpu.regs.read(inst.rs2))?;
        }
        OpecodeKind::OP_AMOAND_W => {
            let load_addr =
                cpu.trans_addr(TransFor::Load, TransAlign::Size32, cpu.regs.read(inst.rs1))?;
            cpu.regs.write(inst.rd, cpu.bus.load32(load_addr)?);
            let store_addr = cpu
                .trans_addr(
                    TransFor::StoreAMO,
                    TransAlign::Size32,
                    cpu.regs.read(inst.rs1),
                )
                .expect("transition address failed in AMO");
            cpu.bus
                .store32(store_addr, cpu.regs.read(inst.rd) & cpu.regs.read(inst.rs2))?;
        }
        OpecodeKind::OP_AMOOR_W => {
            let load_addr =
                cpu.trans_addr(TransFor::Load, TransAlign::Size32, cpu.regs.read(inst.rs1))?;
            cpu.regs.write(inst.rd, cpu.bus.load32(load_addr)?);
            let store_addr = cpu
                .trans_addr(
                    TransFor::StoreAMO,
                    TransAlign::Size32,
                    cpu.regs.read(inst.rs1),
                )
                .expect("transition address failed in AMO");
            cpu.bus
                .store32(store_addr, cpu.regs.read(inst.rd) | cpu.regs.read(inst.rs2))?;
        }
        OpecodeKind::OP_AMOMIN_W => {
            let load_addr =
                cpu.trans_addr(TransFor::Load, TransAlign::Size32, cpu.regs.read(inst.rs1))?;
            cpu.regs.write(inst.rd, cpu.bus.load32(load_addr)?);
            let store_addr = cpu
                .trans_addr(
                    TransFor::StoreAMO,
                    TransAlign::Size32,
                    cpu.regs.read(inst.rs1),
                )
                .expect("transition address failed in AMO");
            cpu.bus.store32(
                store_addr,
                std::cmp::min(
                    cpu.regs.read(inst.rd) as i32,
                    cpu.regs.read(inst.rs2) as i32,
                ) as u64,
            )?;
        }
        OpecodeKind::OP_AMOMAX_W => {
            let load_addr =
                cpu.trans_addr(TransFor::Load, TransAlign::Size32, cpu.regs.read(inst.rs1))?;
            cpu.regs.write(inst.rd, cpu.bus.load32(load_addr)?);
            let store_addr = cpu
                .trans_addr(
                    TransFor::StoreAMO,
                    TransAlign::Size32,
                    cpu.regs.read(inst.rs1),
                )
                .expect("transition address failed in AMO");
            cpu.bus.store32(
                store_addr,
                std::cmp::max(
                    cpu.regs.read(inst.rd) as i32,
                    cpu.regs.read(inst.rs2) as i32,
                ) as u64,
            )?;
        }
        OpecodeKind::OP_AMOMINU_W => {
            let load_addr =
                cpu.trans_addr(TransFor::Load, TransAlign::Size32, cpu.regs.read(inst.rs1))?;
            cpu.regs.write(inst.rd, cpu.bus.load32(load_addr)?);
            let store_addr = cpu
                .trans_addr(
                    TransFor::StoreAMO,
                    TransAlign::Size32,
                    cpu.regs.read(inst.rs1),
                )
                .expect("transition address failed in AMO");
            cpu.bus.store32(
                store_addr,
                std::cmp::min(cpu.regs.read(inst.rd), cpu.regs.read(inst.rs2)),
            )?;
        }
        OpecodeKind::OP_AMOMAXU_W => {
            let load_addr =
                cpu.trans_addr(TransFor::Load, TransAlign::Size32, cpu.regs.read(inst.rs1))?;
            cpu.regs.write(inst.rd, cpu.bus.load32(load_addr)?);
            let store_addr = cpu
                .trans_addr(
                    TransFor::StoreAMO,
                    TransAlign::Size32,
                    cpu.regs.read(inst.rs1),
                )
                .expect("transition address failed in AMO");
            cpu.bus.store32(
                store_addr,
                std::cmp::max(cpu.regs.read(inst.rd), cpu.regs.read(inst.rs2)),
            )?;
        }
        OpecodeKind::OP_LR_D => {
            let load_addr =
                cpu.trans_addr(TransFor::Load, TransAlign::Size64, cpu.regs.read(inst.rs1))?;
            let _rl = inst.imm.unwrap() & 0x1;
            let _aq = inst.imm.unwrap() >> 1 & 0x1;
            cpu.regs.write(inst.rd, cpu.bus.load64(load_addr)?);
            cpu.reservation_set.insert(load_addr as usize);
        }
        OpecodeKind::OP_SC_D => {
            let store_addr = cpu.trans_addr(
                TransFor::StoreAMO,
                TransAlign::Size64,
                cpu.regs.read(inst.rs1),
            )?;
            // cache value == rs1 --> store rs2 to rs1 and assign zero to rd
            // cache value != rs1 --> ignore and assign non-zero to rd
            if cpu.reservation_set.contains(&(store_addr as usize)) {
                let _rl = inst.imm.unwrap() & 0x1;
                let _aq = inst.imm.unwrap() >> 1 & 0x1;
                cpu.bus.store64(store_addr, cpu.regs.read(inst.rs2))?;
                cpu.reservation_set.remove(&(store_addr as usize));
                cpu.regs.write(inst.rd, 0);
            } else {
                cpu.regs.write(inst.rd, 1);
            }

            cpu.reservation_set.clear();
        }
        OpecodeKind::OP_AMOSWAP_D => {
            let load_addr =
                cpu.trans_addr(TransFor::Load, TransAlign::Size64, cpu.regs.read(inst.rs1))?;
            cpu.regs.write(inst.rd, cpu.bus.load64(load_addr)?);
            let store_addr = cpu
                .trans_addr(
                    TransFor::StoreAMO,
                    TransAlign::Size64,
                    cpu.regs.read(inst.rs1),
                )
                .expect("transition address failed in AMO");
            cpu.bus.store64(store_addr, cpu.regs.read(inst.rs2))?;
        }
        OpecodeKind::OP_AMOADD_D => {
            let load_addr =
                cpu.trans_addr(TransFor::Load, TransAlign::Size64, cpu.regs.read(inst.rs1))?;
            cpu.regs.write(inst.rd, cpu.bus.load64(load_addr)?);
            let store_addr = cpu
                .trans_addr(
                    TransFor::StoreAMO,
                    TransAlign::Size64,
                    cpu.regs.read(inst.rs1),
                )
                .expect("transition address failed in AMO");
            cpu.bus.store64(
                store_addr,
                (cpu.regs.read(inst.rd) as i64 + cpu.regs.read(inst.rs2) as i64) as u64,
            )?;
        }
        OpecodeKind::OP_AMOXOR_D => {
            let load_addr =
                cpu.trans_addr(TransFor::Load, TransAlign::Size64, cpu.regs.read(inst.rs1))?;
            cpu.regs.write(inst.rd, cpu.bus.load64(load_addr)?);
            let store_addr = cpu
                .trans_addr(
                    TransFor::StoreAMO,
                    TransAlign::Size64,
                    cpu.regs.read(inst.rs1),
                )
                .expect("transition address failed in AMO");
            cpu.bus
                .store64(store_addr, cpu.regs.read(inst.rd) ^ cpu.regs.read(inst.rs2))?;
        }
        OpecodeKind::OP_AMOAND_D => {
            let load_addr =
                cpu.trans_addr(TransFor::Load, TransAlign::Size64, cpu.regs.read(inst.rs1))?;
            cpu.regs.write(inst.rd, cpu.bus.load64(load_addr)?);
            let store_addr = cpu
                .trans_addr(
                    TransFor::StoreAMO,
                    TransAlign::Size64,
                    cpu.regs.read(inst.rs1),
                )
                .expect("transition address failed in AMO");
            cpu.bus
                .store64(store_addr, cpu.regs.read(inst.rd) & cpu.regs.read(inst.rs2))?;
        }
        OpecodeKind::OP_AMOOR_D => {
            let load_addr =
                cpu.trans_addr(TransFor::Load, TransAlign::Size64, cpu.regs.read(inst.rs1))?;
            cpu.regs.write(inst.rd, cpu.bus.load64(load_addr)?);
            let store_addr = cpu
                .trans_addr(
                    TransFor::StoreAMO,
                    TransAlign::Size64,
                    cpu.regs.read(inst.rs1),
                )
                .expect("transition address failed in AMO");
            cpu.bus
                .store64(store_addr, cpu.regs.read(inst.rd) | cpu.regs.read(inst.rs2))?;
        }
        OpecodeKind::OP_AMOMIN_D => {
            let load_addr =
                cpu.trans_addr(TransFor::Load, TransAlign::Size64, cpu.regs.read(inst.rs1))?;
            cpu.regs.write(inst.rd, cpu.bus.load64(load_addr)?);
            let store_addr = cpu
                .trans_addr(
                    TransFor::StoreAMO,
                    TransAlign::Size64,
                    cpu.regs.read(inst.rs1),
                )
                .expect("transition address failed in AMO");
            cpu.bus.store64(
                store_addr,
                std::cmp::min(
                    cpu.regs.read(inst.rd) as i64,
                    cpu.regs.read(inst.rs2) as i64,
                ) as u64,
            )?;
        }
        OpecodeKind::OP_AMOMAX_D => {
            let load_addr =
                cpu.trans_addr(TransFor::Load, TransAlign::Size64, cpu.regs.read(inst.rs1))?;
            cpu.regs.write(inst.rd, cpu.bus.load64(load_addr)?);
            let store_addr = cpu
                .trans_addr(
                    TransFor::StoreAMO,
                    TransAlign::Size64,
                    cpu.regs.read(inst.rs1),
                )
                .expect("transition address failed in AMO");
            cpu.bus.store64(
                store_addr,
                std::cmp::max(
                    cpu.regs.read(inst.rd) as i64,
                    cpu.regs.read(inst.rs2) as i64,
                ) as u64,
            )?;
        }
        OpecodeKind::OP_AMOMINU_D => {
            let load_addr =
                cpu.trans_addr(TransFor::Load, TransAlign::Size64, cpu.regs.read(inst.rs1))?;
            cpu.regs.write(inst.rd, cpu.bus.load64(load_addr)?);
            let store_addr = cpu
                .trans_addr(
                    TransFor::StoreAMO,
                    TransAlign::Size64,
                    cpu.regs.read(inst.rs1),
                )
                .expect("transition address failed in AMO");
            cpu.bus.store64(
                store_addr,
                std::cmp::min(cpu.regs.read(inst.rd), cpu.regs.read(inst.rs2)),
            )?;
        }
        OpecodeKind::OP_AMOMAXU_D => {
            let load_addr =
                cpu.trans_addr(TransFor::Load, TransAlign::Size64, cpu.regs.read(inst.rs1))?;
            cpu.regs.write(inst.rd, cpu.bus.load64(load_addr)?);
            let store_addr = cpu
                .trans_addr(
                    TransFor::StoreAMO,
                    TransAlign::Size64,
                    cpu.regs.read(inst.rs1),
                )
                .expect("transition address failed in AMO");
            cpu.bus.store64(
                store_addr,
                std::cmp::max(cpu.regs.read(inst.rd), cpu.regs.read(inst.rs2)),
            )?;
        }
        _ => panic!("not an A extension"),
    }

    Ok(())
}
