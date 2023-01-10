pub mod csr;
pub mod decode;
pub mod execution;
pub mod fetch;
mod instruction;
mod mmu;
mod reg;

use crate::{bus, elfload, log, Isa};
use csr::{CSRname, Xstatus};
use std::collections::HashSet;

#[derive(Copy, Clone, Debug)]
#[allow(clippy::enum_clike_unportable_variant)]
pub enum TrapCause {
    InstAddrMisaligned = 0,
    IllegalInst = 2,
    Breakpoint = 3,
    LoadAddrMisaligned = 4,
    StoreAMOAddrMisaligned = 6,
    UmodeEcall = 8,
    SmodeEcall = 9,
    MmodeEcall = 11,
    InstPageFault = 12,
    LoadPageFault = 13,
    StoreAMOPageFault = 15,
    MachineSoftwareInterrupt = (1 << 31) + 3,
    MachineTimerInterrupt = (1 << 31) + 7,
    SupervisorSoftwareInterrupt = (1 << 31) + 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PrivilegedLevel {
    User = 0b00,
    Supervisor = 0b01,
    Reserved = 0b10,
    Machine = 0b11,
}

pub enum TransAlign {
    Size8 = 1,
    Size16 = 2,
    Size32 = 4,
    Size64 = 8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TransFor {
    Fetch,
    Load,
    StoreAMO,
    Deleg,
}

pub struct Cpu {
    pub pc: u64,
    pub bus: bus::Bus,
    pub regs: reg::Register,
    csrs: csr::CSRs,
    mmu: mmu::Mmu,
    pub reservation_set: HashSet<usize>,
    isa: Isa,
    pub priv_lv: PrivilegedLevel,
}

impl Cpu {
    pub fn new(loader: elfload::ElfLoader, pc_from_cl: Option<u64>, isa: Isa) -> Self {
        // initialize bus and get the entry point
        let bus = bus::Bus::new(loader, isa);

        Cpu {
            pc: pc_from_cl.unwrap_or(bus.mrom.base_addr),
            bus,
            regs: reg::Register::new(isa),
            csrs: csr::CSRs::new().init(),
            mmu: mmu::Mmu::new(),
            reservation_set: HashSet::new(),
            isa,
            priv_lv: PrivilegedLevel::Machine,
        }
    }

    fn add2pc(&mut self, addval: u64) {
        self.pc += addval;
    }

    fn update_pc(&mut self, newpc: u64) {
        self.pc = newpc;
    }

    pub fn exec_one_cycle(&mut self) -> Result<(), (Option<u64>, TrapCause, String)> {
        use execution::Execution;
        use fetch::fetch;

        self.check_interrupt()?;

        fetch(self)?.decode()?.execution(self)
    }

    fn check_interrupt(&mut self) -> Result<(), (Option<u64>, TrapCause, String)> {
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
        self.bus.store64(MTIME, mtime as i64 + 1).unwrap();

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

    fn interrupt(&mut self, tval_addr: u64, cause_of_trap: TrapCause) {
        self.csrs
            .write(CSRname::mcause.wrap(), cause_of_trap as u64);
        self.csrs.write(CSRname::mepc.wrap(), self.pc);

        // check Machine Trap Delegation Registers
        let mcause = self.csrs.read(CSRname::mcause.wrap()).unwrap();
        let mideleg = self.csrs.read(CSRname::mideleg.wrap()).unwrap();
        if self.priv_lv != PrivilegedLevel::Machine && (mideleg & 1 << mcause) != 0 {
            log::infoln!("delegated");
            self.csrs
                .write(CSRname::scause.wrap(), cause_of_trap as u64);
            self.csrs.write(CSRname::sepc.wrap(), self.pc);
            self.csrs.write(CSRname::stval.wrap(), tval_addr);
            self.priv_lv = PrivilegedLevel::Supervisor;

            let new_pc = self.csrs.read(CSRname::stvec.wrap()).unwrap();
            self.update_pc(new_pc);
        } else {
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
            let new_pc = if mtvec & 0b1 == 1 {
                (mtvec - 1) + 4 * cause_of_trap as u64
            } else {
                mtvec
            };
            self.update_pc(new_pc);
        }
    }

    fn exception(&mut self, tval_addr: u64, cause_of_trap: TrapCause) {
        self.csrs
            .write(CSRname::mcause.wrap(), cause_of_trap as u64);
        self.csrs.write(CSRname::mepc.wrap(), self.pc);

        // check Machine Trap Delegation Registers
        let mcause = self.csrs.read(CSRname::mcause.wrap()).unwrap();
        let medeleg = self.csrs.read(CSRname::medeleg.wrap()).unwrap();
        if self.priv_lv != PrivilegedLevel::Machine && (medeleg & 1 << mcause) != 0 {
            // https://msyksphinz.hatenablog.com/entry/2018/04/03/040000
            log::infoln!("delegated");
            self.csrs
                .write(CSRname::scause.wrap(), cause_of_trap as u64);
            self.csrs.write(CSRname::sepc.wrap(), self.pc);
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

            let new_pc = self.csrs.read(CSRname::stvec.wrap()).unwrap();
            self.update_pc(new_pc);
        } else {
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

            let new_pc = self.csrs.read(CSRname::mtvec.wrap()).unwrap();
            self.update_pc(new_pc);
        }
    }

    pub fn trap(&mut self, tval_addr: u64, cause_of_trap: TrapCause) {
        match cause_of_trap {
            TrapCause::InstAddrMisaligned
            | TrapCause::IllegalInst
            | TrapCause::Breakpoint
            | TrapCause::UmodeEcall
            | TrapCause::SmodeEcall
            | TrapCause::MmodeEcall
            | TrapCause::LoadAddrMisaligned
            | TrapCause::StoreAMOAddrMisaligned
            | TrapCause::InstPageFault
            | TrapCause::LoadPageFault
            | TrapCause::StoreAMOPageFault => {
                self.exception(tval_addr, cause_of_trap);
            }
            TrapCause::MachineTimerInterrupt
            | TrapCause::MachineSoftwareInterrupt
            | TrapCause::SupervisorSoftwareInterrupt => {
                self.interrupt(tval_addr, cause_of_trap);
            }
        }

        log::infoln!("new pc: 0x{:x}", self.pc);
    }

    fn trans_addr(
        &mut self,
        purpose: TransFor,
        align: TransAlign,
        addr: u64,
    ) -> Result<u64, (Option<u64>, TrapCause, String)> {
        let addr = self.check_breakpoint(purpose, addr)?;
        let mut trans_priv = self.priv_lv;

        if (purpose == TransFor::Load || purpose == TransFor::StoreAMO)
            && self
                .csrs
                .read_xstatus(PrivilegedLevel::Machine, Xstatus::MPRV)
                == 1
        {
            trans_priv = match self
                .csrs
                .read_xstatus(PrivilegedLevel::Machine, Xstatus::MPP)
            {
                0b00 => PrivilegedLevel::User,
                0b01 => PrivilegedLevel::Supervisor,
                0b10 => panic!("PrivilegedLevel 0x3 is Reserved."),
                0b11 => PrivilegedLevel::Machine,
                _ => panic!("invalid PrivilegedLevel"),
            }
        }

        match self
            .mmu
            .trans_addr(purpose, addr, &self.csrs, &self.bus.dram, trans_priv)
        {
            Ok(vaddr) => {
                if addr % align as u64 == 0 {
                    match self.isa {
                        Isa::Rv32 => Ok(vaddr & 0xffffffff),
                        Isa::Rv64 => Ok(vaddr),
                    }
                } else {
                    let cause = match purpose {
                        TransFor::Fetch | TransFor::Deleg => TrapCause::InstAddrMisaligned,
                        TransFor::Load => TrapCause::LoadAddrMisaligned,
                        TransFor::StoreAMO => TrapCause::StoreAMOAddrMisaligned,
                    };
                    Err((
                        Some(addr),
                        cause,
                        format!("address transration failed: {:?}", cause),
                    ))
                }
            }
            Err(cause) => {
                log::debugln!("{:?}", cause);
                Err((
                    Some(addr),
                    cause,
                    format!("address transration failed: {:?}", cause),
                ))
            }
        }
    }
}
