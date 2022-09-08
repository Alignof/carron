use crate::cpu::{CPU, PrivilegedLevel, TrapCause};
use crate::cpu::instruction::{Instruction, OpecodeKind};
use crate::cpu::csr::{CSRname, Xstatus};

pub fn exec(inst: &Instruction, cpu: &mut CPU) -> Result<(), (Option<u32>, TrapCause, String)> {
    match inst.opc {
        OpecodeKind::OP_SRET => {
            if cpu.csrs.read_xstatus(PrivilegedLevel::Machine, Xstatus::TSR) == 1 {
                return Err((
                    Some(cpu.bus.load32(cpu.pc)? as u32),
                    TrapCause::IllegalInst,
                    "exec sret but mstatus.TSR == 1".to_string()
                ));
            }

            cpu.priv_lv = match cpu.csrs.read_xstatus(PrivilegedLevel::Supervisor, Xstatus::SPP) {
                0b00 => PrivilegedLevel::User,
                0b01 => PrivilegedLevel::Supervisor,
                0b10 => panic!("PrivilegedLevel 0x3 is Reserved."),
                0b11 => panic!("invalid transition. (S-mode -> M-mode)"),
                _ => panic!("invalid PrivilegedLevel"),
            };
            dbg!(cpu.priv_lv);
            dbg_hex::dbg_hex!(cpu.csrs.read(CSRname::sepc.wrap())?);

            cpu.csrs.write_xstatus( // sstatus.SIE = sstatus.SPIE
                PrivilegedLevel::Supervisor,
                Xstatus::SIE,
                cpu.csrs.read_xstatus(PrivilegedLevel::Supervisor, Xstatus::SPIE)
            );
            cpu.csrs.write_xstatus(PrivilegedLevel::Supervisor, Xstatus::SPIE, 0b1); // ssatus.SPIE = 1
            cpu.csrs.write_xstatus(PrivilegedLevel::Supervisor, Xstatus::SPP, 0b00); // ssatus.SPP = 0

            if cpu.csrs.read(CSRname::mstatus.wrap())? >> 22 & 1 == 1 { // mstatus.TSR == 1
                let except_pc = cpu.pc;
                cpu.exception(except_pc, TrapCause::IllegalInst);
            } else {
                let new_pc = cpu.csrs.read(CSRname::sepc.wrap())?;
                cpu.update_pc(new_pc as i32);
            }
        },
        OpecodeKind::OP_MRET => {
            cpu.priv_lv = match cpu.csrs.read_xstatus(PrivilegedLevel::Machine, Xstatus::MPP) {
                0b00 => PrivilegedLevel::User,
                0b01 => PrivilegedLevel::Supervisor,
                0b10 => panic!("PrivilegedLevel 0x3 is Reserved."),
                0b11 => PrivilegedLevel::Machine,
                _ => panic!("invalid PrivilegedLevel"),
            };
            dbg!(cpu.priv_lv);

            cpu.csrs.write_xstatus( // sstatus.MIE = sstatus.MPIE
                PrivilegedLevel::Machine,
                Xstatus::MIE,
                cpu.csrs.read_xstatus(PrivilegedLevel::Machine, Xstatus::MPIE)
            );
            cpu.csrs.write_xstatus(PrivilegedLevel::Machine, Xstatus::MPIE, 0b1); // msatus.MPIE = 1
            cpu.csrs.write_xstatus(PrivilegedLevel::Machine, Xstatus::MPP, 0b00); // msatus.MPP = 0

            let new_pc = cpu.csrs.read(CSRname::mepc.wrap())? as i32;
            cpu.update_pc(new_pc);
        },
        OpecodeKind::OP_WFI => {
            /* nop */
        },
        OpecodeKind::OP_SFENCE_VMA => {
            if cpu.priv_lv == PrivilegedLevel::Supervisor && cpu.csrs.read_xstatus(PrivilegedLevel::Machine, Xstatus::TVM) == 1 {
                cpu.exception(cpu.bus.load32(cpu.pc)? as u32, TrapCause::IllegalInst);
            }
        },
        _ => panic!("not an privileged extension"),
    }

    Ok(())
}

