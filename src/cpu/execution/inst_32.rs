use crate::cpu::{CPU, PrivilegedLevel, TransFor, TrapCause};
use crate::cpu::csr::{CSRname, Xstatus};
use crate::cpu::instruction::{Instruction, OpecodeKind};

pub fn exe_inst(inst: &Instruction, cpu: &mut CPU) {
    use OpecodeKind::*;
    const INST_SIZE: u32 = 4;

    // store previous program counter for excluding branch case
    let prev_pc = cpu.pc;

    match inst.opc {
        OP_LUI => {
            cpu.regs.write(inst.rd, inst.imm.unwrap());
        },
        OP_AUIPC => {
            cpu.regs.write(inst.rd, cpu.pc as i32 + inst.imm.unwrap());
        },
        OP_JAL => {
            cpu.regs.write(inst.rd, (cpu.pc + INST_SIZE) as i32); 
            cpu.add2pc(inst.imm.unwrap());
        },
        OP_JALR => {
            // calc next_pc before updated
            let next_pc = cpu.pc + INST_SIZE;
            // setting the least-significant bit of the result to zero->vvvvvv
            cpu.update_pc((cpu.regs.read(inst.rs1) + inst.imm.unwrap()) & !0x1);
            cpu.regs.write(inst.rd, next_pc as i32); 
        },
        OP_BEQ => {
            if cpu.regs.read(inst.rs1) == cpu.regs.read(inst.rs2) {
                cpu.add2pc(inst.imm.unwrap());
            } 
        },
        OP_BNE => {
            if cpu.regs.read(inst.rs1) != cpu.regs.read(inst.rs2) {
                cpu.add2pc(inst.imm.unwrap());
            } 
        },
        OP_BLT => {
            if cpu.regs.read(inst.rs1) < cpu.regs.read(inst.rs2) {
                cpu.add2pc(inst.imm.unwrap());
            } 
        },
        OP_BGE => {
            if cpu.regs.read(inst.rs1) >= cpu.regs.read(inst.rs2) {
                cpu.add2pc(inst.imm.unwrap());
            } 
        },
        OP_BLTU => {
            if (cpu.regs.read(inst.rs1) as u32) < (cpu.regs.read(inst.rs2) as u32) {
                cpu.add2pc(inst.imm.unwrap());
            } 
        },
        OP_BGEU => {
            if (cpu.regs.read(inst.rs1) as u32) >= (cpu.regs.read(inst.rs2) as u32) {
                cpu.add2pc(inst.imm.unwrap());
            } 
        },
        OP_LB => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                cpu.regs.write(inst.rd, cpu.bus.load8(load_addr));
            }
        },
        OP_LH => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                cpu.regs.write(inst.rd, cpu.bus.load16(load_addr));
            }
        },
        OP_LW => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                cpu.regs.write(inst.rd, cpu.bus.load32(load_addr));
            }
        },
        OP_LBU => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                cpu.regs.write(inst.rd, cpu.bus.load_u8(load_addr));
            }
        },
        OP_LHU => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                cpu.regs.write(inst.rd, cpu.bus.load_u16(load_addr));
            }
        },
        OP_SB => {
            if let Some(store_addr) = cpu.trans_addr(TransFor::Store, cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                cpu.bus.store8(store_addr, cpu.regs.read(inst.rs2));
            }
        },
        OP_SH => {
            if let Some(store_addr) = cpu.trans_addr(TransFor::Store, cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                cpu.bus.store16(store_addr, cpu.regs.read(inst.rs2));
            }
        },
        OP_SW => {
            if let Some(store_addr) = cpu.trans_addr(TransFor::Store, cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                cpu.bus.store32(store_addr, cpu.regs.read(inst.rs2));
            }
        },
        OP_ADDI => {
            cpu.regs.write(inst.rd, cpu.regs.read(inst.rs1) + inst.imm.unwrap());
        },
        OP_SLTI => {
            cpu.regs.write(inst.rd,  
                (cpu.regs.read(inst.rs1) < inst.imm.unwrap()) as i32);
        },
        OP_SLTIU => {
            cpu.regs.write(inst.rd,  
                ((cpu.regs.read(inst.rs1) as u32) < inst.imm.unwrap() as u32) as i32);
        },
        OP_XORI => {
            cpu.regs.write(inst.rd, cpu.regs.read(inst.rs1) ^ inst.imm.unwrap());
        },
        OP_ORI => {
            cpu.regs.write(inst.rd, cpu.regs.read(inst.rs1) | inst.imm.unwrap());
        },
        OP_ANDI => {
            cpu.regs.write(inst.rd, cpu.regs.read(inst.rs1) & inst.imm.unwrap());
        },
        OP_SLLI => {
            cpu.regs.write(inst.rd,
                ((cpu.regs.read(inst.rs1) as u32) << inst.imm.unwrap()) as i32);
        },                                                
        OP_SRLI => {
            cpu.regs.write(inst.rd,
                ((cpu.regs.read(inst.rs1) as u32) >> inst.imm.unwrap()) as i32);
        },
        OP_SRAI => {
            cpu.regs.write(inst.rd,
                ((cpu.regs.read(inst.rs1) as i32) >> inst.imm.unwrap()) as i32);
        },
        OP_ADD => {
            cpu.regs.write(inst.rd,
                cpu.regs.read(inst.rs1) + cpu.regs.read(inst.rs2));
        },
        OP_SUB => {
            cpu.regs.write(inst.rd,
                cpu.regs.read(inst.rs1) - cpu.regs.read(inst.rs2));
        },
        OP_SLL => {
            cpu.regs.write(inst.rd,
                ((cpu.regs.read(inst.rs1) as u32) << cpu.regs.read(inst.rs2)) as i32);
        },
        OP_SLT => {
            cpu.regs.write(inst.rd,
                (cpu.regs.read(inst.rs1) < cpu.regs.read(inst.rs2)) as i32);
        },
        OP_SLTU => {
            cpu.regs.write(inst.rd,
                ((cpu.regs.read(inst.rs1) as u32) < (cpu.regs.read(inst.rs2) as u32)) as i32);
        },
        OP_XOR => {
            cpu.regs.write(inst.rd,
                cpu.regs.read(inst.rs1) ^ cpu.regs.read(inst.rs2));
        },
        OP_SRL => {
            cpu.regs.write(inst.rd,
                ((cpu.regs.read(inst.rs1) as u32)  >> cpu.regs.read(inst.rs2)) as i32);
        },
        OP_SRA => {
            cpu.regs.write(inst.rd,
                (cpu.regs.read(inst.rs1) as i32)  >> cpu.regs.read(inst.rs2));
        },
        OP_OR => {
            cpu.regs.write(inst.rd,
                cpu.regs.read(inst.rs1) | cpu.regs.read(inst.rs2));
        },
        OP_AND => {
            cpu.regs.write(inst.rd,
                cpu.regs.read(inst.rs1) & cpu.regs.read(inst.rs2));
        },
        OP_FENCE => {
            // nop (pipeline are not yet implemented)
        },
        OP_ECALL => {
            cpu.exception(cpu.pc as i32, 
                match cpu.priv_lv {
                    PrivilegedLevel::User => TrapCause::UmodeEcall,
                    PrivilegedLevel::Supervisor => TrapCause::SmodeEcall,
                    _ => panic!("cannot enviroment call in current privileged mode."),
                }
            );
        },
        OP_EBREAK => {
            panic!("not yet implemented: OP_EBREAK");
        },
        OP_CSRRW => {
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2) as i32);
            cpu.csrs.write(inst.rs2, cpu.regs.read(inst.rs1));
        },
        OP_CSRRS => {
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2) as i32);
            cpu.csrs.bitset(inst.rs2, cpu.regs.read(inst.rs1));
        },
        OP_CSRRC => {
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2) as i32);
            cpu.csrs.bitclr(inst.rs2, cpu.regs.read(inst.rs1));
        },
        OP_CSRRWI => {
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2) as i32);
            cpu.csrs.write(inst.rs2, inst.rs1.unwrap() as i32);
        },
        OP_CSRRSI => {
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2) as i32);
            cpu.csrs.bitset(inst.rs2, inst.rs1.unwrap() as i32);
        },
        OP_CSRRCI => {
            cpu.regs.write(inst.rd, cpu.csrs.read(inst.rs2) as i32);
            cpu.csrs.bitclr(inst.rs2, inst.rs1.unwrap() as i32);
        },
        OP_SRET => {
            cpu.priv_lv = match cpu.csrs.read_xstatus(&cpu.priv_lv, Xstatus::SPP) {
                0b00 => PrivilegedLevel::User,
                0b01 => PrivilegedLevel::Supervisor,
                0b10 => panic!("PrivilegedLevel 0x3 is Reserved."),
                0b11 => panic!("invalid transition. (S-mode -> M-mode)"),
                _ => panic!("invalid PrivilegedLevel"),
            };
            dbg!(&cpu.priv_lv);
            dbg_hex::dbg_hex!(cpu.csrs.read(CSRname::sepc.wrap()));

            if cpu.csrs.read_xstatus(&cpu.priv_lv, Xstatus::TVM) == 0 {
                let new_pc = cpu.csrs.read(CSRname::sepc.wrap());
                cpu.update_pc(new_pc as i32);
            } else {
                let except_pc = cpu.pc as i32;
                cpu.exception(except_pc, TrapCause::IllegalInst);
            }
        },
        OP_MRET => {
            cpu.priv_lv = match cpu.csrs.read_xstatus(&cpu.priv_lv, Xstatus::MPP) {
                0b00 => PrivilegedLevel::User,
                0b01 => PrivilegedLevel::Supervisor,
                0b10 => panic!("PrivilegedLevel 0x3 is Reserved."),
                0b11 => PrivilegedLevel::Machine,
                _ => panic!("invalid PrivilegedLevel"),
            };
            let new_pc = cpu.csrs.read(CSRname::mepc.wrap()) as i32;
            cpu.update_pc(new_pc);
        },
        OP_SFENCE_VMA => {
            // nop (pipeline are not yet implemented)
            if cpu.csrs.read_xstatus(&cpu.priv_lv, Xstatus::TVM) != 0 {
                let except_pc = cpu.pc as i32;
                cpu.exception(except_pc, TrapCause::IllegalInst);
            }
        },
        OP_LR_W => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1)) {
                let _rl = inst.imm.unwrap() & 0x1;
                let _aq = inst.imm.unwrap() >> 1 & 0x1;
                cpu.regs.write(inst.rd, cpu.bus.load32(load_addr));
                // --TODO--
                // and store rs1 address to cache
            }
		},
        OP_SC_W => {
            if let Some(store_addr) = cpu.trans_addr(TransFor::Store, cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                // --TODO--
                // cache value == rs1 --> store rs2 to rs1 and assign zero to rd
                // cache value != rs1 --> ignore and assign non-zero to rd
                let _rl = inst.imm.unwrap() & 0x1;
                let _aq = inst.imm.unwrap() >> 1 & 0x1;
                cpu.bus.store32(store_addr, cpu.regs.read(inst.rs2));
            }
		},
        OP_AMOSWAP_W => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1)) {
                cpu.regs.write(inst.rd, cpu.bus.load32(load_addr));
                let store_addr = cpu.trans_addr(TransFor::Store, cpu.regs.read(inst.rs1))
                    .expect("transition address failed in AMO");
                cpu.bus.store32(store_addr, cpu.regs.read(inst.rs2));
            }
		},
        OP_AMOADD_W => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1)) {
                cpu.regs.write(inst.rd, cpu.bus.load32(load_addr));
                let store_addr = cpu.trans_addr(TransFor::Store, cpu.regs.read(inst.rs1))
                    .expect("transition address failed in AMO");
                cpu.bus.store32(store_addr, cpu.regs.read(inst.rd) + cpu.regs.read(inst.rs2));
            }
		},
        OP_AMOXOR_W => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1)) {
                cpu.regs.write(inst.rd, cpu.bus.load32(load_addr));
                let store_addr = cpu.trans_addr(TransFor::Store, cpu.regs.read(inst.rs1))
                    .expect("transition address failed in AMO");
                cpu.bus.store32(store_addr, cpu.regs.read(inst.rd) ^ cpu.regs.read(inst.rs2));
            }
		},
        OP_AMOAND_W => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1)) {
                cpu.regs.write(inst.rd, cpu.bus.load32(load_addr));
                let store_addr = cpu.trans_addr(TransFor::Store, cpu.regs.read(inst.rs1))
                    .expect("transition address failed in AMO");
                cpu.bus.store32(store_addr, cpu.regs.read(inst.rd) & cpu.regs.read(inst.rs2));
            }
		},
        OP_AMOOR_W => {
            if let Some(load_addr) = cpu.trans_addr(TransFor::Load, cpu.regs.read(inst.rs1)) {
                cpu.regs.write(inst.rd, cpu.bus.load32(load_addr));
                let store_addr = cpu.trans_addr(TransFor::Store, cpu.regs.read(inst.rs1))
                    .expect("transition address failed in AMO");
                cpu.bus.store32(store_addr, cpu.regs.read(inst.rd) | cpu.regs.read(inst.rs2));
            }
		},
        OP_AMOMIN_W => {
            panic!("not yet implemented: AMOMIN_W");
		},
        OP_AMOMAX_W => {
            panic!("not yet implemented: AMOMAX_W");
		},
        OP_AMOMINU_W => {
            panic!("not yet implemented: AMOINU_W");
		},
        OP_AMOMAXU_W => {
            panic!("not yet implemented: AMOAXU_W");
		},
        _ => panic!("not a full instruction"),
    }

    // add the program counter when it isn't a branch instruction
    if cpu.pc == prev_pc {
        cpu.add2pc(INST_SIZE as i32);
    }
}
