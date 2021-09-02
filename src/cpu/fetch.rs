use crate::cpu;
use crate::cpu::decode::Decode;

pub fn fetch(cpu: &cpu::CPU) -> Box<dyn Decode> {
    let dram = &cpu.bus.dram;
    let index_pc : usize = cpu.pc;
    let is_cinst: bool = dram.raw_byte(index_pc) & 0x3 != 0x3;

    if is_cinst {
        let new_inst: u16 = 
            (dram.raw_byte(index_pc + 1) as u16) <<  8 |
            (dram.raw_byte(index_pc + 0) as u16);
        Box::new(new_inst)
    } else {
        let new_inst: u32 =
            (dram.raw_byte(index_pc + 3) as u32) << 24 |
            (dram.raw_byte(index_pc + 2) as u32) << 16 |
            (dram.raw_byte(index_pc + 1) as u32) <<  8 |
            (dram.raw_byte(index_pc + 0) as u32);
        Box::new(new_inst)
    }
}

