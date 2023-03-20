use super::decode::Decode;
use super::{TransAlign, TransFor, TrapCause};
use crate::cpu::Cpu;
use crate::{log, Isa};

pub fn fetch(cpu: &mut Cpu) -> Result<Box<dyn Decode>, (Option<u64>, TrapCause, String)> {
    let index_pc: u64 = cpu.trans_addr(TransFor::Fetch, TransAlign::Size8, cpu.pc())?;
    let is_cinst: bool = match cpu.bus.load_u8(index_pc) {
        Ok(inst_byte) => inst_byte & 0x3 != 0x3,
        Err((inst, _, msg)) => return Err((inst, TrapCause::InstAccessFault, msg)),
    };

    if is_cinst {
        match *cpu.isa {
            Isa::Rv32 => log::infoln!("pc: 0x{:08x}", cpu.pc()),
            Isa::Rv64 => {
                log::infoln!("pc: 0x{:016x}", cpu.pc());
            }
        };
        match cpu.bus.load_u16(index_pc) {
            Ok(new_inst) => Ok(Box::new(new_inst as u16)),
            Err((inst, _, msg)) => Err((inst, TrapCause::InstAccessFault, msg)),
        }
    } else {
        let index_pc2: u64 = cpu.trans_addr(TransFor::Fetch, TransAlign::Size8, cpu.pc() + 2)?;
        let inst_upper: u32 = match cpu.bus.load_u16(index_pc2) {
            Ok(inst) => inst as u32,
            Err((inst, _, msg)) => return Err((inst, TrapCause::InstAccessFault, msg)),
        };
        let inst_lower: u32 = match cpu.bus.load_u16(index_pc) {
            Ok(inst) => inst as u32,
            Err((inst, _, msg)) => return Err((inst, TrapCause::InstAccessFault, msg)),
        };
        match *cpu.isa {
            Isa::Rv32 => log::infoln!("pc: 0x{:08x}", cpu.pc()),
            Isa::Rv64 => {
                log::infoln!("pc: 0x{:016x}", cpu.pc());
            }
        };
        Ok(Box::new(inst_upper << 16 | inst_lower))
    }
}
