use crate::cpu::{CPU, PrivilegedLevel, TrapCause};
use crate::cpu::instruction::{Instruction, OpecodeKind};
use crate::cpu::csr::{CSRname, Xstatus};

pub fn exec(inst: &Instruction, cpu: &mut CPU) -> Result<(), (Option<i32>, TrapCause, String)> {
    match inst.opc {
        OpecodeKind::OP_SRET => {
            cpu.priv_lv = match cpu.csrs.read_xstatus(&PrivilegedLevel::Supervisor, Xstatus::SPP) {
                0b00 => PrivilegedLevel::User,
                0b01 => PrivilegedLevel::Supervisor,
                0b10 => panic!("PrivilegedLevel 0x3 is Reserved."),
                0b11 => panic!("invalid transition. (S-mode -> M-mode)"),
                _ => panic!("invalid PrivilegedLevel"),
            };
            dbg!(&cpu.priv_lv);
            dbg_hex::dbg_hex!(cpu.csrs.read(CSRname::sepc.wrap())?);

            cpu.csrs.bitset(CSRname::sstatus.wrap(), (cpu.csrs.read_xstatus(&PrivilegedLevel::Supervisor, Xstatus::SPIE) << 1) as i32); // sstatus.SIE = sstatus.SPIE
            cpu.csrs.bitset(CSRname::sstatus.wrap(), 1 << 5); // sstatus.SPIE = 1
            cpu.csrs.bitclr(CSRname::sstatus.wrap(), 1 << 8); // sstatus.SPP = 0

            if cpu.csrs.read(CSRname::mstatus.wrap())? >> 22 & 1 == 1 { // mstatus.TSR == 1
                let except_pc = cpu.pc as i32;
                cpu.exception(except_pc, TrapCause::IllegalInst);
            } else {
                let new_pc = cpu.csrs.read(CSRname::sepc.wrap())?;
                cpu.update_pc(new_pc as i32);
            }
        },
        OpecodeKind::OP_MRET => {
            cpu.priv_lv = match cpu.csrs.read_xstatus(&PrivilegedLevel::Machine, Xstatus::MPP) {
                0b00 => PrivilegedLevel::User,
                0b01 => PrivilegedLevel::Supervisor,
                0b10 => panic!("PrivilegedLevel 0x3 is Reserved."),
                0b11 => PrivilegedLevel::Machine,
                _ => panic!("invalid PrivilegedLevel"),
            };

            cpu.csrs.bitset(CSRname::mstatus.wrap(), (cpu.csrs.read_xstatus(&PrivilegedLevel::Machine, Xstatus::MPIE) << 3) as i32); // mstatus.MIE = mstatus.MPIE
            cpu.csrs.bitset(CSRname::mstatus.wrap(), 1 << 7); // mstatus.MPIE = 1
            cpu.csrs.bitclr(CSRname::mstatus.wrap(), 0b11 << 11); // mstatus.MPP = 0

            let new_pc = cpu.csrs.read(CSRname::mepc.wrap())? as i32;
            cpu.update_pc(new_pc);
        },
        OpecodeKind::OP_SFENCE_VMA => {
            // nop (pipeline are not yet implemented)
            if cpu.csrs.read_xstatus(&cpu.priv_lv, Xstatus::TVM) != 0 {
                let except_pc = cpu.pc as i32;
                cpu.exception(except_pc, TrapCause::IllegalInst);
            }
        },
        _ => panic!("not an privileged extension"),
    }

    Ok(())
}

