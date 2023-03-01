use super::csr::Xstatus;
use super::{CSRname, Cpu, PrivilegedLevel};
use crate::{log, TrapCause};

impl Cpu {
    pub fn check_interrupt(&mut self) -> Result<(), (Option<u64>, TrapCause, String)> {
        const MSIP: u64 = 3;
        const SSIP: u64 = 1;
        const MTIP: u64 = 7;
        const MTIME: u64 = 0x0200_BFF8;
        const MTIMECMP: u64 = 0x0200_4000;
        let mie = self.csrs.read(CSRname::mie.wrap()).unwrap();
        let mtime: u64 = self.bus.load64(MTIME).unwrap();
        let mtimecmp: u64 = self.bus.load64(MTIMECMP).unwrap();

        if (mie >> MTIP) & 0b1 == 1 && mtime >= mtimecmp {
            self.csrs.write(CSRname::mip.wrap(), 1 << MTIP)
        }

        let mip = self.csrs.read(CSRname::mip.wrap()).unwrap();
        let mideleg = self.csrs.read(CSRname::mideleg.wrap()).unwrap();
        let is_interrupt_enabled = |bit: u64| {
            (mie >> bit) & 0b1 == 1 && (mip >> bit) & 0b1 == 1 && (mideleg >> bit) & 0b1 == 0
        };

        // mtime += 1
        self.bus.store64(MTIME, mtime + 1).unwrap();

        match self.priv_lv {
            PrivilegedLevel::Machine => {
                if self
                    .csrs
                    .read_xstatus(PrivilegedLevel::Machine, Xstatus::MIE)
                    == 1
                {
                    if is_interrupt_enabled(MTIP) {
                        // TODO: bit clear when mtimecmp written
                        self.csrs.bitclr(CSRname::mip.wrap(), 1 << MTIP);
                        return Err((
                            None,
                            TrapCause::MachineTimerInterrupt,
                            "machine timer interrupt".to_string(),
                        ));
                    }
                    if is_interrupt_enabled(MSIP) {
                        return Err((
                            None,
                            TrapCause::MachineSoftwareInterrupt,
                            "machine software interrupt".to_string(),
                        ));
                    }
                    if is_interrupt_enabled(SSIP) {
                        return Err((
                            None,
                            TrapCause::SupervisorSoftwareInterrupt,
                            "supervisor software interrupt".to_string(),
                        ));
                    }
                }
            }
            PrivilegedLevel::Supervisor => {
                if is_interrupt_enabled(MTIP) {
                    // TODO: bit clear when mtimecmp written
                    self.csrs.bitclr(CSRname::mip.wrap(), 1 << MTIP);
                    return Err((
                        None,
                        TrapCause::MachineTimerInterrupt,
                        "machine timer interrupt".to_string(),
                    ));
                }
                if is_interrupt_enabled(MSIP) {
                    return Err((
                        None,
                        TrapCause::MachineSoftwareInterrupt,
                        "machine software interrupt".to_string(),
                    ));
                }
                if self
                    .csrs
                    .read_xstatus(PrivilegedLevel::Supervisor, Xstatus::MIE)
                    == 1
                    && is_interrupt_enabled(SSIP)
                {
                    return Err((
                        None,
                        TrapCause::SupervisorSoftwareInterrupt,
                        "supervisor software interrupt".to_string(),
                    ));
                }
            }
            PrivilegedLevel::User => {
                if is_interrupt_enabled(MTIP) {
                    // TODO: bit clear when mtimecmp written
                    self.csrs.bitclr(CSRname::mip.wrap(), 1 << MTIP);
                    return Err((
                        None,
                        TrapCause::MachineTimerInterrupt,
                        "machine timer interrupt".to_string(),
                    ));
                }
                if is_interrupt_enabled(MSIP) {
                    return Err((
                        None,
                        TrapCause::MachineSoftwareInterrupt,
                        "machine software interrupt".to_string(),
                    ));
                }
                if is_interrupt_enabled(SSIP) {
                    return Err((
                        None,
                        TrapCause::SupervisorSoftwareInterrupt,
                        "supervisor software interrupt".to_string(),
                    ));
                }
            }
            _ => (),
        }

        Ok(())
    }

    fn get_deleg(&self, cause_of_trap: TrapCause) -> u64 {
        match cause_of_trap {
            TrapCause::InstAddrMisaligned
            | TrapCause::InstAccessFault
            | TrapCause::IllegalInst
            | TrapCause::Breakpoint
            | TrapCause::UmodeEcall
            | TrapCause::SmodeEcall
            | TrapCause::MmodeEcall
            | TrapCause::LoadAddrMisaligned
            | TrapCause::LoadAccessFault
            | TrapCause::StoreAMOAddrMisaligned
            | TrapCause::StoreAMOAccessFault
            | TrapCause::InstPageFault
            | TrapCause::LoadPageFault
            | TrapCause::StoreAMOPageFault => self.csrs.read(CSRname::medeleg.wrap()).unwrap(),
            TrapCause::MachineTimerInterrupt
            | TrapCause::MachineSoftwareInterrupt
            | TrapCause::SupervisorSoftwareInterrupt => {
                self.csrs.read(CSRname::mideleg.wrap()).unwrap()
            }
        }
    }

    pub fn trap(&mut self, tval_addr: u64, cause_of_trap: TrapCause) {
        // check Machine Trap Delegation Registers
        let deleg = self.get_deleg(cause_of_trap);
        let new_pc = if self.priv_lv != PrivilegedLevel::Machine
            && (deleg & 1 << cause_of_trap as u32) != 0
        {
            let scause = self.csrs.read(CSRname::scause.wrap()).unwrap();
            log::infoln!("delegated");
            self.csrs
                .write(CSRname::scause.wrap(), cause_of_trap as u64);
            self.csrs.write(CSRname::sepc.wrap(), self.pc());
            self.csrs.write(CSRname::stval.wrap(), tval_addr);
            self.csrs.write_xstatus(
                // sstatus.SPIE = sstatus.SIE
                PrivilegedLevel::Supervisor,
                Xstatus::SPIE,
                self.csrs
                    .read_xstatus(PrivilegedLevel::Supervisor, Xstatus::SIE),
            );
            self.csrs
                .write_xstatus(PrivilegedLevel::Supervisor, Xstatus::SIE, 0b0); // Ssatus.SIE = 0
            self.csrs.write_xstatus(
                PrivilegedLevel::Supervisor,
                Xstatus::SPP,
                self.priv_lv as u64,
            ); // set prev_priv to SPP
            self.priv_lv = PrivilegedLevel::Supervisor;

            let stvec = self.csrs.read(CSRname::stvec.wrap()).unwrap();
            if stvec & 0b1 == 1 {
                (stvec - 1) + 4 * scause.trailing_zeros() as u64
            } else {
                stvec
            }
        } else {
            let mcause = self.csrs.read(CSRname::mcause.wrap()).unwrap();
            self.csrs
                .write(CSRname::mcause.wrap(), cause_of_trap as u64);
            self.csrs.write(CSRname::mepc.wrap(), self.pc());
            self.csrs.write(CSRname::mtval.wrap(), tval_addr);
            self.csrs.write_xstatus(
                // sstatus.MPIE = sstatus.MIE
                PrivilegedLevel::Machine,
                Xstatus::MPIE,
                self.csrs
                    .read_xstatus(PrivilegedLevel::Machine, Xstatus::MIE),
            );
            self.csrs
                .write_xstatus(PrivilegedLevel::Machine, Xstatus::MIE, 0b0); // msatus.MIE = 0
            self.csrs
                .write_xstatus(PrivilegedLevel::Machine, Xstatus::MPP, self.priv_lv as u64); // set prev_priv to MPP
            self.priv_lv = PrivilegedLevel::Machine;

            let mtvec = self.csrs.read(CSRname::mtvec.wrap()).unwrap();
            if mtvec & 0b1 == 1 {
                (mtvec - 1) + 4 * mcause.trailing_zeros() as u64
            } else {
                mtvec
            }
        };

        self.update_pc(new_pc);
        log::infoln!("new pc: 0x{:x}", self.pc());
    }
}
