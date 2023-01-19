pub mod clint;
pub mod device_tree;
pub mod dram;
pub mod mrom;

use crate::{elfload, Isa, TrapCause};
use clint::Clint;
use dram::Dram;
use mrom::Mrom;

pub struct Bus {
    pub mrom: mrom::Mrom,
    pub clint: clint::Clint,
    pub dram: dram::Dram,
}

impl Bus {
    pub fn new(loader: elfload::ElfLoader, isa: Isa) -> Self {
        // load proxy kernel before user program when it's given
        let dram = Dram::new(loader);
        let mut mrom = Mrom::new(dram.base_addr, isa);

        // create and load DTB
        mrom.load_dtb(dram.base_addr);

        Bus {
            mrom,
            clint: Clint::new(),
            dram,
        }
    }

    // get 1 byte
    pub fn raw_byte(&self, addr: u64) -> u8 {
        if self.mrom.in_range(addr) {
            self.mrom.raw_byte(addr)
        } else if self.clint.in_range(addr) {
            self.clint.raw_byte(addr)
        } else if self.dram.in_range(addr) {
            self.dram.raw_byte(addr)
        } else {
            panic!("bus.raw_byte() failed: {}", addr)
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
        } else {
            Err((
                Some(addr),
                TrapCause::StoreAMOPageFault,
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
        } else {
            Err((
                Some(addr),
                TrapCause::StoreAMOPageFault,
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
        } else {
            Err((
                Some(addr),
                TrapCause::StoreAMOPageFault,
                format!("addr out of range at store32: {addr:#x}"),
            ))
        }
    }

    pub fn store64(
        &mut self,
        addr: u64,
        data: i64,
    ) -> Result<(), (Option<u64>, TrapCause, String)> {
        if self.mrom.in_range(addr) {
            self.mrom.store64(addr, data as u64)
        } else if self.clint.in_range(addr) {
            self.clint.store64(addr, data as u64)
        } else if self.dram.in_range(addr) {
            self.dram.store64(addr, data as u64)
        } else {
            Err((
                Some(addr),
                TrapCause::StoreAMOPageFault,
                format!("addr out of range at store64: {addr:#x}"),
            ))
        }
    }

    // load
    pub fn load8(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        if self.mrom.in_range(addr) {
            self.mrom.load8(addr)
        } else if self.clint.in_range(addr) {
            self.clint.load8(addr)
        } else if self.dram.in_range(addr) {
            self.dram.load8(addr)
        } else {
            Err((
                Some(addr),
                TrapCause::LoadPageFault,
                format!("addr out of range at loat8: {addr:#x}"),
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
        } else {
            Err((
                Some(addr),
                TrapCause::LoadPageFault,
                format!("addr out of range at loat16: {addr:#x}"),
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
        } else {
            Err((
                Some(addr),
                TrapCause::LoadPageFault,
                format!("addr out of range at loat32: {addr:#x}"),
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
        } else {
            Err((
                Some(addr),
                TrapCause::LoadPageFault,
                format!("addr out of range at loat64: {addr:#x}"),
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
        } else {
            Err((
                Some(addr),
                TrapCause::LoadPageFault,
                format!("addr out of range at loat_u8: {addr:#x}"),
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
        } else {
            Err((
                Some(addr),
                TrapCause::LoadPageFault,
                format!("addr out of range at loat_u16: {addr:#x}"),
            ))
        }
    }
}

pub trait Device {
    fn in_range(&self, addr: u64) -> bool;
    fn addr2index(&self, addr: u64) -> usize;
    fn raw_byte(&self, addr: u64) -> u8;
    fn store8(&mut self, addr: u64, data: u64) -> Result<(), (Option<u64>, TrapCause, String)>;
    fn store16(&mut self, addr: u64, data: u64) -> Result<(), (Option<u64>, TrapCause, String)>;
    fn store32(&mut self, addr: u64, data: u64) -> Result<(), (Option<u64>, TrapCause, String)>;
    fn store64(&mut self, addr: u64, data: u64) -> Result<(), (Option<u64>, TrapCause, String)>;
    fn load8(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)>;
    fn load16(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)>;
    fn load32(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)>;
    fn load64(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)>;
    fn load_u8(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)>;
    fn load_u16(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)>;
}
