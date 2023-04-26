use crate::cpu::csr::{CSRname, Xstatus};
use crate::cpu::instruction::{Instruction, OpecodeKind};
use crate::cpu::{Cpu, PrivilegedLevel, TrapCause};
use crate::log;

pub fn exec(inst: &Instruction, cpu: &mut Cpu) -> Result<(), (Option<u64>, TrapCause, String)> {
    match inst.opc {
        OpecodeKind::OP_SRET => {
            if cpu.csrs.read_xstatus(Xstatus::TSR) == 1 {
                return Err((
                    Some(cpu.bus.load32(cpu.pc())?),
                    TrapCause::IllegalInst,
                    "exec sret but mstatus.TSR == 1".to_string(),
                ));
            }

            let new_priv = cpu.csrs.read_xstatus(Xstatus::SPP);
            if cpu.csrs.read(CSRname::mstatus.wrap())? >> 22 & 1 == 1 {
                // mstatus.TSR == 1
                let except_pc = cpu.pc();
                cpu.trap(except_pc, TrapCause::IllegalInst);
            } else {
                let new_pc = cpu.csrs.read(CSRname::sepc.wrap())?;
                cpu.update_pc(new_pc);
            }

            cpu.csrs.write_xstatus(
                // sstatus.SIE = sstatus.SPIE
                Xstatus::SIE,
                cpu.csrs.read_xstatus(Xstatus::SPIE),
            );
            cpu.csrs.write_xstatus(Xstatus::SPIE, 0b1); // ssatus.SPIE = 1
            cpu.csrs.write_xstatus(Xstatus::SPP, 0b00); // ssatus.SPP = 0

            cpu.set_priv_lv(match new_priv {
                0b00 => PrivilegedLevel::User,
                0b01 => PrivilegedLevel::Supervisor,
                0b10 => panic!("PrivilegedLevel 0x3 is Reserved."),
                0b11 => panic!("invalid transition. (S-mode -> M-mode)"),
                _ => panic!("invalid PrivilegedLevel"),
            });
            log::debugln!("priv: {:?}", cpu.priv_lv());
            log::debugln!("csrs.sepc: {:#x}", cpu.csrs.read(CSRname::sepc.wrap())?);
        }
        OpecodeKind::OP_MRET => {
            let new_priv = cpu.csrs.read_xstatus(Xstatus::MPP);
            let new_pc = cpu.csrs.read(CSRname::mepc.wrap())?;
            cpu.update_pc(new_pc);

            cpu.csrs.write_xstatus(
                // sstatus.MIE = sstatus.MPIE
                Xstatus::MIE,
                cpu.csrs.read_xstatus(Xstatus::MPIE),
            );
            cpu.csrs.write_xstatus(Xstatus::MPIE, 0b1); // msatus.MPIE = 1
            cpu.csrs.write_xstatus(Xstatus::MPP, 0b00); // msatus.MPP = 0

            cpu.set_priv_lv(match new_priv {
                0b00 => PrivilegedLevel::User,
                0b01 => PrivilegedLevel::Supervisor,
                0b10 => panic!("PrivilegedLevel 0x3 is Reserved."),
                0b11 => PrivilegedLevel::Machine,
                _ => panic!("invalid PrivilegedLevel"),
            });
            log::debugln!("priv: {:?}", cpu.priv_lv());
        }
        OpecodeKind::OP_WFI => { /* nop */ }
        OpecodeKind::OP_SFENCE_VMA => {
            if cpu.priv_lv() == PrivilegedLevel::Supervisor
                && cpu.csrs.read_xstatus(Xstatus::TVM) == 1
            {
                let illegal_inst = cpu.bus.load32(cpu.pc())?;
                cpu.trap(illegal_inst, TrapCause::IllegalInst);
            }
        }
        _ => panic!("not an privileged extension"),
    }

    Ok(())
}
