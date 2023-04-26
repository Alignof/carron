use super::csr::Xstatus;
use super::{CSRname, Cpu, PrivilegedLevel};
use crate::{log, Isa, TrapCause};

impl Cpu {
    pub fn check_interrupt(&mut self) -> Result<(), (Option<u64>, TrapCause, String)> {
        const SSIP: u64 = 1;
        const MSIP: u64 = 3;
        const STIP: u64 = 5;
        const MTIP: u64 = 7;
        const SEIP: u64 = 9;
        const MEIP: u64 = 11;

        const MTIME: u64 = 0x0200_BFF8;
        const MTIMECMP: u64 = 0x0200_4000;
        let mtime: u64 = self.bus.load64(MTIME).unwrap();
        let mtimecmp: u64 = self.bus.load64(MTIMECMP).unwrap();
        if mtime >= mtimecmp {
            self.csrs
                .bitset(CSRname::mip.wrap(), 1 << MTIP | 1 << STIP)
                .unwrap() // ignore result
        } else {
            self.csrs
                .bitclr(CSRname::mip.wrap(), 1 << MTIP | 1 << STIP)
                .unwrap()
        };

        self.csrs
            .write(
                CSRname::mip.wrap(),
                self.csrs.read(CSRname::mip.wrap()).unwrap() & !(self.bus.plic.mip_mask)
                    | self.bus.plic.mip_value,
            )
            .ok();

        let mip = self.csrs.read(CSRname::mip.wrap()).unwrap();
        let mie = self.csrs.read(CSRname::mie.wrap()).unwrap();

        let pending_interrupts = mip & mie;
        let mideleg = self.csrs.read(CSRname::mideleg.wrap()).unwrap();
        let mstatus_mie = self.csrs.read_xstatus(PrivilegedLevel::Machine, Xstatus::MIE);
        let m_enabled = match self.priv_lv() {
            PrivilegedLevel::Machine => (mstatus_mie != 0) as i64,
            _ => 1,
        };
        let enabled_interrupt_mask = pending_interrupts & !mideleg & (-m_enabled as u64);
        let enabled_interrupt_mask = if enabled_interrupt_mask == 0 {
            let mstatus_sie = self.csrs.read_xstatus(PrivilegedLevel::Machine, Xstatus::SIE);
            let s_enabled = match self.priv_lv() {
                PrivilegedLevel::Machine => 0,
                PrivilegedLevel::Supervisor => (mstatus_sie != 0) as i64,
                _ => 1,
            };

            pending_interrupts & mideleg & (-s_enabled as u64)
        } else {
            enabled_interrupt_mask
        };
        let is_interrupt_enabled = |bit: u64| (enabled_interrupt_mask & (1 << bit)) != 0;

        if is_interrupt_enabled(MEIP) {
            self.csrs.bitclr(CSRname::mip.wrap(), 1 << MEIP).unwrap();
            self.bus.plic.mip_value = 0;
            return Err((
                Some(0),
                TrapCause::MachineExternalInterrupt,
                "machine external interrupt".to_string(),
            ));
        }
        if is_interrupt_enabled(MSIP) {
            self.csrs.bitclr(CSRname::mip.wrap(), 1 << MSIP).unwrap();
            return Err((
                Some(0),
                TrapCause::MachineSoftwareInterrupt,
                "machine software interrupt".to_string(),
            ));
        }
        if is_interrupt_enabled(MTIP) {
            // TODO: bit clear when mtimecmp written
            self.csrs.bitclr(CSRname::mip.wrap(), 1 << MTIP).unwrap();
            return Err((
                Some(0),
                TrapCause::MachineTimerInterrupt,
                "machine timer interrupt".to_string(),
            ));
        }
        if is_interrupt_enabled(SEIP) {
            self.csrs.bitclr(CSRname::mip.wrap(), 1 << SEIP).unwrap();
            self.bus.plic.mip_value = 0;
            return Err((
                Some(0),
                TrapCause::SupervisorExternalInterrupt,
                "supervisor external interrupt".to_string(),
            ));
        }
        if is_interrupt_enabled(SSIP) {
            self.csrs.bitclr(CSRname::mip.wrap(), 1 << SSIP).unwrap();
            return Err((
                Some(0),
                TrapCause::SupervisorSoftwareInterrupt,
                "supervisor software interrupt".to_string(),
            ));
        }
        if is_interrupt_enabled(STIP) {
            // TODO: bit clear when mtimecmp written
            self.csrs.bitclr(CSRname::mip.wrap(), 1 << STIP).unwrap();
            return Err((
                Some(0),
                TrapCause::SupervisorTimerInterrupt,
                "supervisor timer interrupt".to_string(),
            ));
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
            TrapCause::SupervisorSoftwareInterrupt
            | TrapCause::MachineSoftwareInterrupt
            | TrapCause::SupervisorTimerInterrupt
            | TrapCause::MachineTimerInterrupt
            | TrapCause::SupervisorExternalInterrupt
            | TrapCause::MachineExternalInterrupt => {
                self.csrs.read(CSRname::mideleg.wrap()).unwrap()
            }
        }
    }

    pub fn trap(&mut self, tval_addr: u64, cause_of_trap: TrapCause) {
        let prev_priv = self.priv_lv();

        // check Machine Trap Delegation Registers
        let deleg = self.get_deleg(cause_of_trap);
        let new_pc = if self.priv_lv() != PrivilegedLevel::Machine
            && (deleg & 1 << cause_of_trap as u32) != 0
        {
            self.set_priv_lv(PrivilegedLevel::Supervisor);
            let scause = self.csrs.read(CSRname::scause.wrap()).unwrap();

            log::infoln!("delegated");
            self.csrs
                .write(
                    CSRname::scause.wrap(),
                    match *self.isa {
                        Isa::Rv32 => cause_of_trap as u64,
                        Isa::Rv64 => match cause_of_trap {
                            TrapCause::MachineSoftwareInterrupt
                            | TrapCause::MachineTimerInterrupt
                            | TrapCause::MachineExternalInterrupt
                            | TrapCause::SupervisorSoftwareInterrupt
                            | TrapCause::SupervisorTimerInterrupt
                            | TrapCause::SupervisorExternalInterrupt => {
                                (1 << 63) | (cause_of_trap as u64 & 0x7fff_ffff)
                            }
                            _ => cause_of_trap as u64,
                        },
                    },
                )
                .unwrap();
            self.csrs.write(CSRname::sepc.wrap(), self.pc()).unwrap();
            self.csrs.write(CSRname::stval.wrap(), tval_addr).unwrap();
            self.csrs.write_xstatus(
                PrivilegedLevel::Supervisor,
                // sstatus.SPIE = sstatus.SIE
                Xstatus::SPIE,
                self.csrs.read_xstatus(PrivilegedLevel::Supervisor, Xstatus::SIE),
            );
            self.csrs.write_xstatus(PrivilegedLevel::Supervisor, Xstatus::SIE, 0b0); // Ssatus.SIE = 0
            self.csrs.write_xstatus(PrivilegedLevel::Supervisor, Xstatus::SPP, prev_priv as u64); // set prev_priv to SPP

            let stvec = self.csrs.read(CSRname::stvec.wrap()).unwrap();
            if stvec & 0b1 == 1 {
                (stvec - 1) + 4 * scause.trailing_zeros() as u64
            } else {
                stvec
            }
        } else {
            self.set_priv_lv(PrivilegedLevel::Machine);
            let mcause = self.csrs.read(CSRname::mcause.wrap()).unwrap();

            self.csrs
                .write(
                    CSRname::mcause.wrap(),
                    match *self.isa {
                        Isa::Rv32 => cause_of_trap as u64,
                        Isa::Rv64 => match cause_of_trap {
                            TrapCause::MachineSoftwareInterrupt
                            | TrapCause::MachineTimerInterrupt
                            | TrapCause::MachineExternalInterrupt
                            | TrapCause::SupervisorSoftwareInterrupt
                            | TrapCause::SupervisorTimerInterrupt
                            | TrapCause::SupervisorExternalInterrupt => {
                                (1 << 63) | (cause_of_trap as u64 & 0x7fff_ffff)
                            }
                            _ => cause_of_trap as u64,
                        },
                    },
                )
                .unwrap();
            self.csrs.write(CSRname::mepc.wrap(), self.pc()).unwrap();
            self.csrs.write(CSRname::mtval.wrap(), tval_addr).unwrap();
            self.csrs.write_xstatus(
                PrivilegedLevel::Machine,
                // sstatus.MPIE = sstatus.MIE
                Xstatus::MPIE,
                self.csrs.read_xstatus(PrivilegedLevel::Machine, Xstatus::MIE),
            );
            self.csrs.write_xstatus(PrivilegedLevel::Machine, Xstatus::MIE, 0b0); // msatus.MIE = 0
            self.csrs.write_xstatus(PrivilegedLevel::Machine, Xstatus::MPP, prev_priv as u64); // set prev_priv to MPP

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
