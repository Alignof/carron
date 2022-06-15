use dbg_hex::dbg_hex;
use super::TransFor;
use super::decode::Decode;

pub fn fetch(cpu: &mut super::CPU) -> Box<dyn Decode> {
    dbg_hex!(cpu.pc);
    let index_pc: u32 = match cpu.trans_addr(TransFor::Fetch, cpu.pc as i32) {
        Some(addr) => addr,
        None => cpu.trans_addr(TransFor::Deleg, cpu.pc as i32).unwrap(), // skip following process and retry it
    };
    let is_cinst: bool = cpu.bus.load_byte(index_pc) & 0x3 != 0x3;

    if is_cinst {
        let new_inst: u16 = 
            (cpu.bus.load_byte(index_pc + 1) as u16) <<  8 |
            (cpu.bus.load_byte(index_pc + 0) as u16);
        Box::new(new_inst)
    } else {
        let (index_pc, index_pc2): (u32, u32) = match cpu.trans_addr(TransFor::Fetch, (cpu.pc + 2) as i32) {
            Some(addr) => (index_pc, addr),
            None => {
                (cpu.trans_addr(TransFor::Deleg, cpu.pc as i32).unwrap(),
                 cpu.trans_addr(TransFor::Fetch, (cpu.pc + 2) as i32).unwrap())
            }
        };
        let new_inst: u32 =
            (cpu.bus.load_byte(index_pc2 + 1) as u32) << 24 |
            (cpu.bus.load_byte(index_pc2 + 0) as u32) << 16 |
            (cpu.bus.load_byte(index_pc + 1) as u32) <<  8 |
            (cpu.bus.load_byte(index_pc + 0) as u32);
        Box::new(new_inst)
    }
}

