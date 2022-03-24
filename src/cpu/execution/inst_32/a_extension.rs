use crate::cpu::{CPU, TransFor};
use crate::cpu::instruction::{Instruction, OpecodeKind};

pub fn exec(inst: &Instruction, cpu: &mut CPU) {
    match inst.opc {
        OpecodeKind::OP_LR_W => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1)) {
                let _rl = inst.imm.unwrap() & 0x1;
                let _aq = inst.imm.unwrap() >> 1 & 0x1;
                cpu.regs.write(inst.rd, cpu.bus.load32(load_addr));
                // --TODO--
                // and store rs1 address to cache
            }
		},
        OpecodeKind::OP_SC_W => {
            if let Some(store_addr) = cpu.trans_addr(TransFor::Store, cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                // --TODO--
                // cache value == rs1 --> store rs2 to rs1 and assign zero to rd
                // cache value != rs1 --> ignore and assign non-zero to rd
                let _rl = inst.imm.unwrap() & 0x1;
                let _aq = inst.imm.unwrap() >> 1 & 0x1;
                cpu.bus.store32(store_addr, cpu.regs.read(inst.rs2));
            }
		},
        OpecodeKind::OP_AMOSWAP_W => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1)) {
                cpu.regs.write(inst.rd, cpu.bus.load32(load_addr));
                let store_addr = cpu.trans_addr(TransFor::Store, cpu.regs.read(inst.rs1))
                    .expect("transition address failed in AMO");
                cpu.bus.store32(store_addr, cpu.regs.read(inst.rs2));
            }
		},
        OpecodeKind::OP_AMOADD_W => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1)) {
                cpu.regs.write(inst.rd, cpu.bus.load32(load_addr));
                let store_addr = cpu.trans_addr(TransFor::Store, cpu.regs.read(inst.rs1))
                    .expect("transition address failed in AMO");
                cpu.bus.store32(store_addr, cpu.regs.read(inst.rd) + cpu.regs.read(inst.rs2));
            }
		},
        OpecodeKind::OP_AMOXOR_W => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1)) {
                cpu.regs.write(inst.rd, cpu.bus.load32(load_addr));
                let store_addr = cpu.trans_addr(TransFor::Store, cpu.regs.read(inst.rs1))
                    .expect("transition address failed in AMO");
                cpu.bus.store32(store_addr, cpu.regs.read(inst.rd) ^ cpu.regs.read(inst.rs2));
            }
		},
        OpecodeKind::OP_AMOAND_W => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1)) {
                cpu.regs.write(inst.rd, cpu.bus.load32(load_addr));
                let store_addr = cpu.trans_addr(TransFor::Store, cpu.regs.read(inst.rs1))
                    .expect("transition address failed in AMO");
                cpu.bus.store32(store_addr, cpu.regs.read(inst.rd) & cpu.regs.read(inst.rs2));
            }
		},
        OpecodeKind::OP_AMOOR_W => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1)) {
                cpu.regs.write(inst.rd, cpu.bus.load32(load_addr));
                let store_addr = cpu.trans_addr(TransFor::Store, cpu.regs.read(inst.rs1))
                    .expect("transition address failed in AMO");
                cpu.bus.store32(store_addr, cpu.regs.read(inst.rd) | cpu.regs.read(inst.rs2));
            }
		},
        OpecodeKind::OP_AMOMIN_W => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1)) {
                cpu.regs.write(inst.rd, cpu.bus.load32(load_addr));
                let store_addr = cpu.trans_addr(TransFor::Store, cpu.regs.read(inst.rs1))
                    .expect("transition address failed in AMO");
                cpu.bus.store32(store_addr, std::cmp::min(cpu.regs.read(inst.rd), cpu.regs.read(inst.rs2)));
            }
		},
        OpecodeKind::OP_AMOMAX_W => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1)) {
                cpu.regs.write(inst.rd, cpu.bus.load32(load_addr));
                let store_addr = cpu.trans_addr(TransFor::Store, cpu.regs.read(inst.rs1))
                    .expect("transition address failed in AMO");
                cpu.bus.store32(store_addr, std::cmp::max(cpu.regs.read(inst.rd), cpu.regs.read(inst.rs2)));
            }
		},
        OpecodeKind::OP_AMOMINU_W => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1)) {
                cpu.regs.write(inst.rd, cpu.bus.load32(load_addr));
                let store_addr = cpu.trans_addr(TransFor::Store, cpu.regs.read(inst.rs1))
                    .expect("transition address failed in AMO");
                cpu.bus.store32(store_addr, std::cmp::min(cpu.regs.read(inst.rd) as u32, cpu.regs.read(inst.rs2) as u32) as i32);
            }
		},
        OpecodeKind::OP_AMOMAXU_W => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1)) {
                cpu.regs.write(inst.rd, cpu.bus.load32(load_addr));
                let store_addr = cpu.trans_addr(TransFor::Store, cpu.regs.read(inst.rs1))
                    .expect("transition address failed in AMO");
                cpu.bus.store32(store_addr, std::cmp::max(cpu.regs.read(inst.rd) as u32, cpu.regs.read(inst.rs2) as u32) as i32);
            }
		},
        _ => panic!("not an A extension"),
    }
}

