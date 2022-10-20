use crate::log;
use super::{TransFor, TrapCause};
use super::decode::Decode;

pub fn fetch(cpu: &mut super::CPU) -> Result<Box<dyn Decode>, (Option<u32>, TrapCause, String)> {
    log::infoln!("pc: 0x{:08x}", cpu.pc);
    let index_pc: u32 = cpu.trans_addr(TransFor::Fetch, cpu.pc)?;
    let is_cinst: bool = cpu.bus.raw_byte(index_pc) & 0x3 != 0x3;

    if is_cinst {
        let new_inst: u16 = 
            (cpu.bus.raw_byte(index_pc + 1) as u16) <<  8 |
            (cpu.bus.raw_byte(index_pc) as u16);
        Ok(Box::new(new_inst))
    } else {
        let index_pc2: u32 = cpu.trans_addr(TransFor::Fetch, cpu.pc + 2)?;
        let new_inst: u32 =
            (cpu.bus.raw_byte(index_pc2 + 1) as u32) << 24 |
            (cpu.bus.raw_byte(index_pc2) as u32) << 16 |
            (cpu.bus.raw_byte(index_pc + 1) as u32) <<  8 |
            (cpu.bus.raw_byte(index_pc) as u32);
        Ok(Box::new(new_inst))
    }
}

