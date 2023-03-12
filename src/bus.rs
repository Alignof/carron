mod clint;
mod device_tree;
pub mod dram;
mod mrom;
mod plic;
mod uart;

use crate::{elfload, Arguments, Isa, TrapCause};
use clint::Clint;
use dram::Dram;
use mrom::Mrom;
use plic::Plic;
use uart::Uart;

pub struct Bus {
    pub mrom: mrom::Mrom,
    pub clint: clint::Clint,
    pub dram: dram::Dram,
    pub uart: uart::Uart,
    plic: plic::Plic,
}

impl Bus {
    pub fn new(loader: elfload::ElfLoader, args: &Arguments, isa: Isa) -> Self {
        // load proxy kernel before user program when it's given
        let dram = Dram::new(loader, args, isa);
        let mut mrom = Mrom::new(dram.base_addr, isa);

        // create and load DTB
        mrom.load_dtb(dram.base_addr, dram.initrd_start, dram.initrd_end, isa);

        Bus {
            mrom,
            clint: Clint::new(),
            dram,
            uart: Uart::new(),
            plic: Plic::new(),
        }
    }

    // store
    pub fn store8(&mut self, addr: u64, data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        if self.mrom.in_range(addr) {
            self.mrom.store8(addr, data)
        } else if self.clint.in_range(addr) {
            self.clint.store8(addr, data)
        } else if self.dram.in_range(addr) {
            self.dram.store8(addr, data)
        } else if self.uart.in_range(addr) {
            self.uart.store8(addr, data)
        } else if self.plic.in_range(addr) {
            self.plic.store8(addr, data)
        } else {
            Err((
                Some(addr),
                TrapCause::StoreAMOAccessFault,
                format!("addr out of range at store8: {addr:#x}"),
            ))
        }
    }

    pub fn store16(
        &mut self,
        addr: u64,
        data: u64,
    ) -> Result<(), (Option<u64>, TrapCause, String)> {
        if self.mrom.in_range(addr) {
            self.mrom.store16(addr, data)
        } else if self.clint.in_range(addr) {
            self.clint.store16(addr, data)
        } else if self.dram.in_range(addr) {
            self.dram.store16(addr, data)
        } else if self.uart.in_range(addr) {
            self.uart.store16(addr, data)
        } else if self.plic.in_range(addr) {
            self.plic.store16(addr, data)
        } else {
            Err((
                Some(addr),
                TrapCause::StoreAMOAccessFault,
                format!("addr out of range at store16: {addr:#x}"),
            ))
        }
    }

    pub fn store32(
        &mut self,
        addr: u64,
        data: u64,
    ) -> Result<(), (Option<u64>, TrapCause, String)> {
        if self.mrom.in_range(addr) {
            self.mrom.store32(addr, data)
        } else if self.clint.in_range(addr) {
            self.clint.store32(addr, data)
        } else if self.dram.in_range(addr) {
            self.dram.store32(addr, data)
        } else if self.uart.in_range(addr) {
            self.uart.store32(addr, data)
        } else if self.plic.in_range(addr) {
            self.plic.store32(addr, data)
        } else {
            Err((
                Some(addr),
                TrapCause::StoreAMOAccessFault,
                format!("addr out of range at store32: {addr:#x}"),
            ))
        }
    }

    pub fn store64(
        &mut self,
        addr: u64,
        data: u64,
    ) -> Result<(), (Option<u64>, TrapCause, String)> {
        if self.mrom.in_range(addr) {
            self.mrom.store64(addr, data)
        } else if self.clint.in_range(addr) {
            self.clint.store64(addr, data)
        } else if self.dram.in_range(addr) {
            self.dram.store64(addr, data)
        } else if self.uart.in_range(addr) {
            self.uart.store64(addr, data)
        } else if self.plic.in_range(addr) {
            self.plic.store64(addr, data)
        } else {
            Err((
                Some(addr),
                TrapCause::StoreAMOAccessFault,
                format!("addr out of range at store64: {addr:#x}"),
            ))
        }
    }

    // load
    pub fn load8(&mut self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        if self.mrom.in_range(addr) {
            self.mrom.load8(addr)
        } else if self.clint.in_range(addr) {
            self.clint.load8(addr)
        } else if self.dram.in_range(addr) {
            self.dram.load8(addr)
        } else if self.uart.in_range(addr) {
            self.uart.load8(addr)
        } else if self.plic.in_range(addr) {
            self.plic.load8(addr)
        } else {
            Err((
                Some(addr),
                TrapCause::LoadAccessFault,
                format!("addr out of range at load8: {addr:#x}"),
            ))
        }
    }

    pub fn load16(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        if self.mrom.in_range(addr) {
            self.mrom.load16(addr)
        } else if self.clint.in_range(addr) {
            self.clint.load16(addr)
        } else if self.dram.in_range(addr) {
            self.dram.load16(addr)
        } else if self.uart.in_range(addr) {
            self.uart.load16(addr)
        } else if self.plic.in_range(addr) {
            self.plic.load16(addr)
        } else {
            Err((
                Some(addr),
                TrapCause::LoadAccessFault,
                format!("addr out of range at load16: {addr:#x}"),
            ))
        }
    }

    pub fn load32(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        if self.mrom.in_range(addr) {
            self.mrom.load32(addr)
        } else if self.clint.in_range(addr) {
            self.clint.load32(addr)
        } else if self.dram.in_range(addr) {
            self.dram.load32(addr)
        } else if self.uart.in_range(addr) {
            self.uart.load32(addr)
        } else if self.plic.in_range(addr) {
            self.plic.load32(addr)
        } else {
            Err((
                Some(addr),
                TrapCause::LoadAccessFault,
                format!("addr out of range at load32: {addr:#x}"),
            ))
        }
    }

    pub fn load64(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        if self.mrom.in_range(addr) {
            self.mrom.load64(addr)
        } else if self.clint.in_range(addr) {
            self.clint.load64(addr)
        } else if self.dram.in_range(addr) {
            self.dram.load64(addr)
        } else if self.uart.in_range(addr) {
            self.uart.load64(addr)
        } else if self.plic.in_range(addr) {
            self.plic.load64(addr)
        } else {
            Err((
                Some(addr),
                TrapCause::LoadAccessFault,
                format!("addr out of range at load64: {addr:#x}"),
            ))
        }
    }

    pub fn load_u8(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        if self.mrom.in_range(addr) {
            self.mrom.load_u8(addr)
        } else if self.clint.in_range(addr) {
            self.clint.load_u8(addr)
        } else if self.dram.in_range(addr) {
            self.dram.load_u8(addr)
        } else if self.uart.in_range(addr) {
            self.uart.load_u8(addr)
        } else if self.plic.in_range(addr) {
            self.plic.load_u8(addr)
        } else {
            Err((
                Some(addr),
                TrapCause::LoadAccessFault,
                format!("addr out of range at load_u8: {addr:#x}"),
            ))
        }
    }

    pub fn load_u16(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        if self.mrom.in_range(addr) {
            self.mrom.load_u16(addr)
        } else if self.clint.in_range(addr) {
            self.clint.load_u16(addr)
        } else if self.dram.in_range(addr) {
            self.dram.load_u16(addr)
        } else if self.uart.in_range(addr) {
            self.uart.load_u16(addr)
        } else if self.plic.in_range(addr) {
            self.plic.load_u16(addr)
        } else {
            Err((
                Some(addr),
                TrapCause::LoadAccessFault,
                format!("addr out of range at load_u16: {addr:#x}"),
            ))
        }
    }

    pub fn load_u32(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        if self.mrom.in_range(addr) {
            self.mrom.load_u32(addr)
        } else if self.clint.in_range(addr) {
            self.clint.load_u32(addr)
        } else if self.dram.in_range(addr) {
            self.dram.load_u32(addr)
        } else if self.uart.in_range(addr) {
            self.uart.load_u32(addr)
        } else if self.plic.in_range(addr) {
            self.plic.load_u32(addr)
        } else {
            Err((
                Some(addr),
                TrapCause::LoadAccessFault,
                format!("addr out of range at load_u32: {addr:#x}"),
            ))
        }
    }
}

pub trait Device {
    fn in_range(&self, addr: u64) -> bool;
    fn addr2index(&self, addr: u64) -> usize;
    fn store8(&mut self, addr: u64, data: u64) -> Result<(), (Option<u64>, TrapCause, String)>;
    fn store16(&mut self, addr: u64, data: u64) -> Result<(), (Option<u64>, TrapCause, String)>;
    fn store32(&mut self, addr: u64, data: u64) -> Result<(), (Option<u64>, TrapCause, String)>;
    fn store64(&mut self, addr: u64, data: u64) -> Result<(), (Option<u64>, TrapCause, String)>;
    fn load8(&mut self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)>;
    fn load16(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)>;
    fn load32(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)>;
    fn load64(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)>;
    fn load_u8(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)>;
    fn load_u16(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)>;
    fn load_u32(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)>;
}
