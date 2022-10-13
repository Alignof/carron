pub mod mrom;
pub mod clint;
pub mod dram;
pub mod device_tree;

use crate::{elfload, TrapCause};
use mrom::Mrom;
use clint::Clint;
use dram::Dram;

pub struct Bus {
    pub mrom: mrom::Mrom,
    pub clint: clint::Clint,
    pub dram: dram::Dram,
}

impl Bus {
    pub fn new(loader: elfload::ElfLoader) -> Bus {
        // load proxy kernel before user program when it's given
        let dram = Dram::new(loader);
        let mut mrom = Mrom::new(dram.base_addr);

        // create and load DTB
        mrom.load_dtb(dram.base_addr);

        Bus {
            mrom, 
            clint: Clint::new(),
            dram,
        }
    }

    // get 1 byte
    pub fn raw_byte(&self, addr: u32) -> u8 {
        if self.mrom.in_range(addr) {
            self.mrom.raw_byte(addr)
        } else if self.clint.in_range(addr){
            self.clint.raw_byte(addr)
        } else if self.dram.in_range(addr){
            self.dram.raw_byte(addr)
        } else {
            panic!("bus.raw_byte() failed: {}", addr)
        }
    }

    // store
    pub fn store8(&mut self, addr: u32, data: u32) -> Result<(), (Option<u32>, TrapCause, String)> {
        if self.mrom.in_range(addr) {
            self.mrom.store8(addr, data)
        } else if self.clint.in_range(addr){
            self.clint.store8(addr, data)
        } else if self.dram.in_range(addr){
            self.dram.store8(addr, data)
        } else {
            Err((
                Some(addr),
                TrapCause::StoreAMOPageFault,
                "addr out of range at store8".to_string()
            ))
        }
    }

    pub fn store16(&mut self, addr: u32, data: u32) -> Result<(), (Option<u32>, TrapCause, String)> {
        if self.mrom.in_range(addr) {
            self.mrom.store16(addr, data)
        } else if self.clint.in_range(addr){
            self.clint.store16(addr, data)
        } else if self.dram.in_range(addr){
            self.dram.store16(addr, data)
        } else {
            Err((
                Some(addr),
                TrapCause::StoreAMOPageFault,
                "addr out of range at store16".to_string()
            ))
        }
    }

    pub fn store32(&mut self, addr: u32, data: u32) -> Result<(), (Option<u32>, TrapCause, String)> {
        if self.mrom.in_range(addr) {
            self.mrom.store32(addr, data)
        } else if self.clint.in_range(addr){
            self.clint.store32(addr, data)
        } else if self.dram.in_range(addr){
            self.dram.store32(addr, data)
        } else {
            Err((
                Some(addr),
                TrapCause::StoreAMOPageFault,
                "addr out of range at store32".to_string()
            ))
        }
    }

    pub fn store64(&mut self, addr: u32, data: i64) -> Result<(), (Option<u32>, TrapCause, String)> {
        if self.mrom.in_range(addr) {
            self.mrom.store64(addr, data)
        } else if self.clint.in_range(addr){
            self.clint.store64(addr, data)
        } else if self.dram.in_range(addr){
            self.dram.store64(addr, data)
        } else {
            Err((
                Some(addr),
                TrapCause::StoreAMOPageFault,
                "addr out of range at store64".to_string()
            ))
        }
    }


    // load
    pub fn load8(&self, addr: u32) -> Result<u32, (Option<u32>, TrapCause, String)> {
        if self.mrom.in_range(addr) {
            self.mrom.load8(addr)
        } else if self.clint.in_range(addr){
            self.clint.load8(addr)
        } else if self.dram.in_range(addr){
            self.dram.load8(addr)
        } else {
            Err((
                Some(addr),
                TrapCause::LoadPageFault,
                "addr out of range at load8".to_string()
            ))
        }
    }

    pub fn load16(&self, addr: u32) -> Result<u32, (Option<u32>, TrapCause, String)> {
        if self.mrom.in_range(addr) {
            self.mrom.load16(addr)
        } else if self.clint.in_range(addr){
            self.clint.load16(addr)
        } else if self.dram.in_range(addr){
            self.dram.load16(addr)
        } else {
            Err((
                Some(addr),
                TrapCause::LoadPageFault,
                "addr out of range at load16".to_string()
            ))
        }
    }

    pub fn load32(&self, addr: u32) -> Result<u32, (Option<u32>, TrapCause, String)> {
        if self.mrom.in_range(addr) {
            self.mrom.load32(addr)
        } else if self.clint.in_range(addr){
            self.clint.load32(addr)
        } else if self.dram.in_range(addr){
            self.dram.load32(addr)
        } else {
            Err((
                Some(addr),
                TrapCause::LoadPageFault,
                "addr out of range at load32".to_string()
            ))
        }
    }

    pub fn load64(&self, addr: u32) -> Result<u64, (Option<u32>, TrapCause, String)> {
        if self.mrom.in_range(addr) {
            self.mrom.load64(addr)
        } else if self.clint.in_range(addr){
            self.clint.load64(addr)
        } else if self.dram.in_range(addr){
            self.dram.load64(addr)
        } else {
            Err((
                Some(addr),
                TrapCause::LoadPageFault,
                "addr out of range at load64".to_string()
            ))
        }
    }

    pub fn load_u8(&self, addr: u32) -> Result<u32, (Option<u32>, TrapCause, String)> {
        if self.mrom.in_range(addr) {
            self.mrom.load_u8(addr)
        } else if self.clint.in_range(addr){
            self.clint.load_u8(addr)
        } else if self.dram.in_range(addr){
            self.dram.load_u8(addr)
        } else {
            Err((
                Some(addr),
                TrapCause::LoadPageFault,
                "addr out of range at load_u8".to_string()
            ))
        }
    }

    pub fn load_u16(&self, addr: u32) -> Result<u32, (Option<u32>, TrapCause, String)> {
        if self.mrom.in_range(addr) {
            self.mrom.load_u16(addr)
        } else if self.clint.in_range(addr){
            self.clint.load_u16(addr)
        } else if self.dram.in_range(addr){
            self.dram.load_u16(addr)
        } else {
            Err((
                Some(addr),
                TrapCause::LoadPageFault,
                "addr out of range at load_u16".to_string()
            ))
        }
    }
}

pub trait Device {
    fn in_range(&self, addr: u32) -> bool;
    fn addr2index(&self, addr: u32) -> usize;
    fn raw_byte(&self, addr: u32) -> u8;
    fn store8(&mut self, addr: u32, data: u32) -> Result<(), (Option<u32>, TrapCause, String)>;
    fn store16(&mut self, addr: u32, data: u32) -> Result<(), (Option<u32>, TrapCause, String)>;
    fn store32(&mut self, addr: u32, data: u32) -> Result<(), (Option<u32>, TrapCause, String)>;
    fn store64(&mut self, addr: u32, data: i64) -> Result<(), (Option<u32>, TrapCause, String)>;
    fn load8(&self, addr: u32) -> Result<u32, (Option<u32>, TrapCause, String)>;
    fn load16(&self, addr: u32) -> Result<u32, (Option<u32>, TrapCause, String)>;
    fn load32(&self, addr: u32) -> Result<u32, (Option<u32>, TrapCause, String)>;
    fn load64(&self, addr: u32) -> Result<u64, (Option<u32>, TrapCause, String)>;
    fn load_u8(&self, addr: u32) -> Result<u32, (Option<u32>, TrapCause, String)>;
    fn load_u16(&self, addr: u32) -> Result<u32, (Option<u32>, TrapCause, String)>;
}

