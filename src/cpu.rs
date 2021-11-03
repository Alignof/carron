pub mod fetch;
pub mod decode;
pub mod execution;
pub mod csr;
mod reg;
mod instruction;

use std::rc::Rc;
use std::cell::RefCell;
use crate::bus;
use crate::elfload;

pub enum PrivilegedLevel {
    User = 0b00,
    Supervisor = 0b01,
    Reserved = 0b10,
    Machine = 0b11,
}

pub struct CPU {
    pub pc: u32,
    pub regs: reg::Register,
        csrs: Rc<RefCell<csr::CSRs>>,
        bus: bus::Bus,
    pub priv_lv: Rc<RefCell<PrivilegedLevel>>,
}

impl CPU {
    pub fn new(entry_address: usize, loader: elfload::ElfLoader) -> CPU {
        let new_lv = Rc::new(RefCell::new(PrivilegedLevel::Machine));
        let new_csrs = Rc::new(RefCell::new(csr::CSRs::new()));
        let new_lv_ref = Rc::clone(&new_lv);
        let new_csrs_ref = Rc::clone(&new_csrs);

        CPU {
            pc: entry_address,
            regs: reg::Register::new(),
            csrs: new_csrs,
            bus: bus::Bus::new(loader, new_csrs_ref, new_lv_ref),
            priv_lv: new_lv, 
        }
    }

    pub fn add2pc(&mut self, addval: i32) {
        self.pc = (self.pc as i32 + addval) as u32;
    }

    pub fn update_pc(&mut self, newval: i32) {
        self.pc = newval as u32;
    }
}

