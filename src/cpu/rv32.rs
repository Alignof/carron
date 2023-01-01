pub mod csr;
pub mod decode;
pub mod execution;
pub mod fetch;
mod mmu;
mod reg;

use super::{PrivilegedLevel, TransAlign, TransFor, TrapCause, CPU};
use crate::bus;
use crate::elfload;
use crate::log;
use csr::{CSRname, Xstatus};
use std::collections::HashSet;

pub struct Cpu32 {
    pub pc: u32,
    pub bus: bus::Bus,
    pub regs: reg::Register,
    csrs: csr::CSRs,
    mmu: mmu::MMU,
    pub reservation_set: HashSet<usize>,
    pub priv_lv: PrivilegedLevel,
}

impl Cpu32 {
    pub fn new(loader: elfload::ElfLoader, pc_from_cl: Option<u32>) -> Box<dyn CPU> {
        // initialize bus and get the entry point
        let bus = bus::Bus::new(loader);

        Box::new(Cpu32 {
            pc: pc_from_cl.unwrap_or(bus.mrom.base_addr),
            bus,
            regs: reg::Register::new(),
            csrs: csr::CSRs::new().init(),
            mmu: mmu::MMU::new(),
            reservation_set: HashSet::new(),
            priv_lv: PrivilegedLevel::Machine,
        })
    }
}

impl CPU for Cpu32 {
    fn pc(&mut self) -> u32 {
        self.pc
    }

    fn bus(&mut self) -> &mut bus::Bus {
        &mut self.bus
    }

    fn add2pc(&mut self, addval: u32) {
        self.pc += addval;
    }

    fn update_pc(&mut self, newpc: u32) {
        self.pc = newpc;
    }

    fn exec_one_cycle(&mut self) -> Result<(), (Option<u32>, TrapCause, String)> {
        use crate::cpu::rv32::execution::Execution;
        use crate::cpu::rv32::fetch::fetch;

        self.check_interrupt()?;

        fetch(self)?.decode()?.execution(self)
    }

    fn check_interrupt(&mut self) -> Result<(), (Option<u32>, TrapCause, String)> {
        const MSIP: u32 = 3;
        const SSIP: u32 = 1;
        const MTIP: u32 = 7;
        const MTIME: u32 = 0x0200_BFF8;
        const MTIMECMP: u32 = 0x0200_4000;
        let mie = self.csrs.read(CSRname::mie.wrap()).unwrap();
        let mtime: u64 = (self.bus.load32(MTIME + 4).unwrap() as u64) << 32
            | self.bus.load32(MTIME).unwrap() as u64;
        let mtimecmp: u64 = (self.bus.load32(MTIMECMP + 4).unwrap() as u64) << 32
            | self.bus.load32(MTIMECMP).unwrap() as u64;

        if (mie >> MTIP) & 0b1 == 1 && mtime >= mtimecmp {
            self.csrs.write(CSRname::mip.wrap(), 1 << MTIP)
        }

        let mip = self.csrs.read(CSRname::mip.wrap()).unwrap();
        let mideleg = self.csrs.read(CSRname::mideleg.wrap()).unwrap();
        let is_interrupt_enabled = |bit: u32| {
            (mie >> bit) & 0b1 == 1 && (mip >> bit) & 0b1 == 1 && (mideleg >> bit) & 0b1 == 0
        };

        // mtime += 1
        self.bus
            .store32(MTIME, ((mtime + 1) & 0xFFFF_FFFF) as u32)
            .unwrap();
        self.bus
            .store32(MTIME + 4, ((mtime + 1) >> 32 & 0xFFFF_FFFF) as u32)
            .unwrap();

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

    fn interrupt(&mut self, tval_addr: u32, cause_of_trap: TrapCause) {
        self.csrs
            .write(CSRname::mcause.wrap(), cause_of_trap as u32);
        self.csrs.write(CSRname::mepc.wrap(), self.pc);

        // check Machine Trap Delegation Registers
        let mcause = self.csrs.read(CSRname::mcause.wrap()).unwrap();
        let mideleg = self.csrs.read(CSRname::mideleg.wrap()).unwrap();
        if self.priv_lv != PrivilegedLevel::Machine && (mideleg & 1 << mcause) != 0 {
            log::infoln!("delegated");
            self.csrs
                .write(CSRname::scause.wrap(), cause_of_trap as u32);
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
                .write_xstatus(PrivilegedLevel::Machine, Xstatus::MPP, self.priv_lv as u32); // set prev_priv to MPP
            self.priv_lv = PrivilegedLevel::Machine;

            let mtvec = self.csrs.read(CSRname::mtvec.wrap()).unwrap();
            let new_pc = if mtvec & 0b1 == 1 {
                (mtvec - 1) + 4 * cause_of_trap as u32
            } else {
                mtvec
            };
            self.update_pc(new_pc);
        }
    }

    fn exception(&mut self, tval_addr: u32, cause_of_trap: TrapCause) {
        self.csrs
            .write(CSRname::mcause.wrap(), cause_of_trap as u32);
        self.csrs.write(CSRname::mepc.wrap(), self.pc);

        // check Machine Trap Delegation Registers
        let mcause = self.csrs.read(CSRname::mcause.wrap()).unwrap();
        let medeleg = self.csrs.read(CSRname::medeleg.wrap()).unwrap();
        if self.priv_lv != PrivilegedLevel::Machine && (medeleg & 1 << mcause) != 0 {
            // https://msyksphinz.hatenablog.com/entry/2018/04/03/040000
            log::infoln!("delegated");
            self.csrs
                .write(CSRname::scause.wrap(), cause_of_trap as u32);
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
                self.priv_lv as u32,
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
                .write_xstatus(PrivilegedLevel::Machine, Xstatus::MPP, self.priv_lv as u32); // set prev_priv to MPP
            self.priv_lv = PrivilegedLevel::Machine;

            let new_pc = self.csrs.read(CSRname::mtvec.wrap()).unwrap();
            self.update_pc(new_pc);
        }
    }

    fn trap(&mut self, tval_addr: u32, cause_of_trap: TrapCause) {
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
        addr: u32,
    ) -> Result<u32, (Option<u32>, TrapCause, String)> {
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
                if addr % align as u32 == 0 {
                    Ok(vaddr)
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
