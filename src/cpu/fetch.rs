use super::decode::Decode;
use super::{TransAlign, TransFor, TrapCause};
use crate::cpu::Cpu;
use crate::{log, Isa};

pub fn fetch(cpu: &mut Cpu) -> Result<Box<dyn Decode>, (Option<u64>, TrapCause, String)> {
    let index_pc: u64 = cpu.trans_addr(TransFor::Fetch, TransAlign::Size8, cpu.pc)?;
    let is_cinst: bool = cpu.bus.load_u8(index_pc)? & 0x3 != 0x3;

    if is_cinst {
        match *cpu.isa {
            Isa::Rv32 => log::infoln!("pc: 0x{:08x}", cpu.pc),
            Isa::Rv64 => log::infoln!("pc: 0x{:016x}", cpu.pc),
        };
        let new_inst: u16 = cpu.bus.load_u16(index_pc)? as u16;
        Ok(Box::new(new_inst))
    } else {
        let index_pc2: u64 = cpu.trans_addr(TransFor::Fetch, TransAlign::Size8, cpu.pc + 2)?;
        match *cpu.isa {
            Isa::Rv32 => log::infoln!("pc: 0x{:08x}", cpu.pc),
            Isa::Rv64 => log::infoln!("pc: 0x{:016x}", cpu.pc),
        };
        let new_inst: u32 =
            (cpu.bus.load_u16(index_pc2)? as u32) << 16 | cpu.bus.load_u16(index_pc)? as u32;
        Ok(Box::new(new_inst))
    }
}
