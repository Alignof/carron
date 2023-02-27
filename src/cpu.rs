pub mod csr;
pub mod decode;
pub mod execution;
pub mod fetch;
mod instruction;
mod mmu;
mod reg;
mod trap;

use crate::{bus, elfload, log, Isa};
use csr::{CSRname, Xstatus};
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Copy, Clone, Debug)]
#[allow(clippy::enum_clike_unportable_variant)]
pub enum TrapCause {
    InstAddrMisaligned = 0,
    InstAccessFault = 1,
    IllegalInst = 2,
    Breakpoint = 3,
    LoadAddrMisaligned = 4,
    LoadAccessFault = 5,
    StoreAMOAddrMisaligned = 6,
    StoreAMOAccessFault = 7,
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
    isa: Rc<Isa>,
    pub priv_lv: PrivilegedLevel,
}

impl Cpu {
    pub fn new(
        loader: elfload::ElfLoader,
        pc_from_cl: Option<u64>,
        kernel_path: Option<String>,
        isa: Isa,
    ) -> Self {
        // initialize bus and get the entry point
        let bus = bus::Bus::new(loader, kernel_path, isa);
        let isa = Rc::new(isa);

        Cpu {
            pc: pc_from_cl.unwrap_or(bus.mrom.base_addr),
            bus,
            regs: reg::Register::new(isa.clone()),
            csrs: csr::CSRs::new(isa.clone()).init(),
            mmu: mmu::Mmu::new(isa.clone()),
            reservation_set: HashSet::new(),
            isa,
            priv_lv: PrivilegedLevel::Machine,
        }
    }

    fn add2pc(&mut self, addval: i32) {
        self.pc = (self.pc as i64 + addval as i64) as u64;
    }

    fn update_pc(&mut self, newpc: u64) {
        self.pc = newpc;
    }

    pub fn exec_one_cycle(&mut self) -> Result<(), (Option<u64>, TrapCause, String)> {
        use execution::Execution;
        use fetch::fetch;

        self.check_interrupt()?;

        fetch(self)?.decode(*self.isa)?.execution(self)
    }

    fn trans_addr(
        &mut self,
        purpose: TransFor,
        align: TransAlign,
        addr: u64,
    ) -> Result<u64, (Option<u64>, TrapCause, String)> {
        let addr = addr.fix2regsz(&self.isa);
        self.check_breakpoint(purpose, addr)?;

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
                    Ok(vaddr.fix2regsz(&self.isa))
                } else {
                    let cause = match purpose {
                        TransFor::Fetch | TransFor::Deleg => TrapCause::InstAddrMisaligned,
                        TransFor::Load => TrapCause::LoadAddrMisaligned,
                        TransFor::StoreAMO => TrapCause::StoreAMOAddrMisaligned,
                    };
                    Err((
                        Some(addr),
                        cause,
                        format!("address transration failed: {addr:#x}"),
                    ))
                }
            }
            Err(cause) => {
                log::debugln!("{:?}", cause);
                Err((
                    Some(addr),
                    cause,
                    format!("address transration failed: {addr:#x}"),
                ))
            }
        }
    }
}

trait CrossIsaUtil {
    fn fix2regsz(self, isa: &Rc<Isa>) -> Self;
}

impl CrossIsaUtil for u64 {
    fn fix2regsz(self, isa: &Rc<Isa>) -> Self {
        match **isa {
            Isa::Rv32 => self & 0xffffffff,
            Isa::Rv64 => self,
        }
    }
}
