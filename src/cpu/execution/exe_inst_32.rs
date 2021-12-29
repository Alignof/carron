use crate::cpu::{CPU, PrivilegedLevel, TrapCause};
use crate::cpu::csr::{CSRname, Xstatus};
use crate::cpu::instruction::{Instruction, OpecodeKind};

pub fn exe_inst(inst: &Instruction, cpu: &mut CPU) {
    use OpecodeKind::*;
    const INST_SIZE: u32 = 4;

    // store previous program counter for excluding branch case
    let prev_pc = cpu.pc;

    match inst.opc {
        OP_LUI => {
            cpu.regs.write(inst.rd, inst.imm.unwrap() << 12);
        },
        OP_AUIPC => {
            cpu.regs.write(inst.rd, cpu.pc as i32 + (inst.imm.unwrap() << 12));
        },
        OP_JAL => {
            cpu.regs.write(inst.rd, (cpu.pc + INST_SIZE) as i32); 
            cpu.add2pc(inst.imm.unwrap());
        },
        OP_JALR => {
            let next_pc = cpu.pc + INST_SIZE;
            // setting the least-significant bit of the result to zero->vvvvvv
            cpu.update_pc((cpu.regs.read(inst.rs1)  + inst.imm.unwrap()) & !0x1);
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
            if let Some(load_addr) = cpu.trans_addr(cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                cpu.regs.write(inst.rd, cpu.bus.load8(load_addr));
            }
        },
        OP_LH => {
            if let Some(load_addr) = cpu.trans_addr(cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                cpu.regs.write(inst.rd, cpu.bus.load16(load_addr));
            }
        },
        OP_LW => {
            if let Some(load_addr) = cpu.trans_addr(cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                cpu.regs.write(inst.rd, cpu.bus.load32(load_addr));
            }
        },
        OP_LBU => {
            if let Some(load_addr) = cpu.trans_addr(cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                cpu.regs.write(inst.rd, cpu.bus.load_u8(load_addr));
            }
        },
        OP_LHU => {
            if let Some(load_addr) = cpu.trans_addr(cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                cpu.regs.write(inst.rd, cpu.bus.load_u16(load_addr));
            }
        },
        OP_SB => {
            if let Some(store_addr) = cpu.trans_addr(cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                cpu.bus.store8(store_addr, cpu.regs.read(inst.rs2));
            }
        },
        OP_SH => {
            if let Some(store_addr) = cpu.trans_addr(cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
                cpu.bus.store16(store_addr, cpu.regs.read(inst.rs2));
            }
        },
        OP_SW => {
            if let Some(store_addr) = cpu.trans_addr(cpu.regs.read(inst.rs1) + inst.imm.unwrap()) {
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
            use CSRname::*;
            let (xcause, xepc, xstatus, xtvec) = match cpu.priv_lv {
                PrivilegedLevel::User => {
                    (ucause.wrap(), uepc.wrap(), ustatus.wrap(), utvec.wrap())
                },
                PrivilegedLevel::Supervisor => {
                    (scause.wrap(), sepc.wrap(), sstatus.wrap(), stvec.wrap())
                },
                PrivilegedLevel::Machine => {
                    (mcause.wrap(), mepc.wrap(), mstatus.wrap(), mtvec.wrap())
                },
                _ => panic!("cannot enviroment call in current privileged mode."),
            };

            // udpate CSRs
            cpu.csrs.write(xcause,
                match cpu.priv_lv {
                    PrivilegedLevel::User => TrapCause::UmodeEcall,
                    PrivilegedLevel::Supervisor => TrapCause::SmodeEcall,
                    _ => panic!("cannot enviroment call in current privileged mode."),
                } as i32
            );
            cpu.csrs.write(xepc, cpu.pc as i32);
            cpu.csrs.bitclr(xstatus, 0x3 << 11);

            // change privileged level to Machine
            cpu.priv_lv = PrivilegedLevel::Machine;

            // update pc from xtvec
            dbg!(&cpu.priv_lv);
            dbg_hex::dbg_hex!(cpu.csrs.read(xtvec) as i32);
            let new_pc = cpu.trans_addr(cpu.csrs.read(xtvec) as i32).unwrap();
            cpu.update_pc(new_pc as i32);
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
        _ => panic!("not a full instruction"),
    }

    // add the program counter when it isn't a branch instruction
    if cpu.pc == prev_pc {
        cpu.add2pc(INST_SIZE as i32);
    }
}
