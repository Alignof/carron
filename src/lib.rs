pub mod system;
pub mod cpu;
pub mod bus;
pub mod elfload;

use cpu::CPU;
use cpu::fetch;
use bus::Bus;

pub struct Simulator {
    pub cpu: cpu::CPU,
    pub bus: bus::Bus,
}

fn find_entry_addr(loader: &elfload::ElfLoader) -> Result<usize, &'static str> {
    let e_entry = loader.elf_header.e_entry;

    for segment in loader.prog_headers.iter() {
        // segment.p_type == 1 <--- 1 means PT_LOAD
        if segment.p_paddr == e_entry && segment.p_type == 1 {
            return Ok(segment.p_offset as usize);
        }
    }

    Err("entry address is not found.")
}

impl Simulator {
    pub fn new(loader: elfload::ElfLoader) -> Simulator {
        let entry_address: usize = match find_entry_addr(&loader) {
            Ok(addr) => addr,
            Err(msg) => panic!("{}", msg),
        };

        Simulator {
            cpu: CPU::new(entry_address),
            bus: Bus::new(loader),
        }
    }

    pub fn simulation(&mut self) {
        use crate::cpu::execution::Execution;

        loop {
            fetch(&self.bus.dram, self.cpu.pc)
                .decode()
                .execution(&mut self.cpu, &mut self.bus.dram);
        }
    }
} 
