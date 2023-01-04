use super::decode::Decode;
use super::{TransAlign, TransFor, TrapCause};
use crate::cpu::CPU;
use crate::log;

pub fn fetch(cpu: &mut super::Cpu64) -> Result<Box<dyn Decode>, (Option<u64>, TrapCause, String)> {
    let index_pc: u64 = cpu.trans_addr(TransFor::Fetch, TransAlign::Size8, cpu.pc)?;
    let is_cinst: bool = cpu.bus.raw_byte(index_pc) & 0x3 != 0x3;

    if is_cinst {
        log::infoln!("pc: 0x{:08x}", cpu.pc);
        let new_inst: u16 =
            (cpu.bus.raw_byte(index_pc + 1) as u16) << 8 | (cpu.bus.raw_byte(index_pc) as u16);
        Ok(Box::new(new_inst))
    } else {
        let index_pc2: u64 = cpu.trans_addr(TransFor::Fetch, TransAlign::Size8, cpu.pc + 2)?;
        log::infoln!("pc: 0x{:08x}", cpu.pc);
        let new_inst: u64 = (cpu.bus.raw_byte(index_pc2 + 1) as u64) << 24
            | (cpu.bus.raw_byte(index_pc2) as u64) << 16
            | (cpu.bus.raw_byte(index_pc + 1) as u64) << 8
            | (cpu.bus.raw_byte(index_pc) as u64);
        Ok(Box::new(new_inst))
    }
}
