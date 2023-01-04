pub mod csr_name;
pub mod instruction;
pub mod rv32;
pub mod rv64;

use crate::bus;

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

pub trait CPU {
    fn pc(&self) -> u32;
    fn add2pc(&mut self, addval: u32);
    fn update_pc(&mut self, newpc: u32);
    fn bus(&mut self) -> &mut bus::Bus;
    fn exec_one_cycle(&mut self) -> Result<(), (Option<u32>, TrapCause, String)>;
    fn check_interrupt(&mut self) -> Result<(), (Option<u32>, TrapCause, String)>;
    fn interrupt(&mut self, tval_addr: u32, cause_of_trap: TrapCause);
    fn exception(&mut self, tval_addr: u32, cause_of_trap: TrapCause);
    fn trap(&mut self, tval_addr: u32, cause_of_trap: TrapCause);
    fn trans_addr(
        &mut self,
        purpose: TransFor,
        align: TransAlign,
        addr: u32,
    ) -> Result<u32, (Option<u32>, TrapCause, String)>;
}
