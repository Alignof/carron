use super::decode::Decode;

pub fn fetch(cpu: &super::CPU) -> Box<dyn Decode> {
    let bus = &cpu.bus;
    let index_pc: u32 = cpu.pc - bus.dram.base_addr;
    let is_cinst: bool = bus.raw_byte(index_pc) & 0x3 != 0x3;

    if is_cinst {
        let new_inst: u16 = 
            (bus.raw_byte(index_pc + 1) as u16) <<  8 |
            (bus.raw_byte(index_pc + 0) as u16);
        Box::new(new_inst)
    } else {
        let new_inst: u32 =
            (bus.raw_byte(index_pc + 3) as u32) << 24 |
            (bus.raw_byte(index_pc + 2) as u32) << 16 |
            (bus.raw_byte(index_pc + 1) as u32) <<  8 |
            (bus.raw_byte(index_pc + 0) as u32);
        Box::new(new_inst)
    }
}



