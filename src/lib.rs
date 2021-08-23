pub mod system;
pub mod cpu;
pub mod bus;
pub mod elfload;

use cpu::CPU;
use cpu::fetch;
use bus::Bus;

pub struct Simulator {
    pub cpu: cpu::CPU,
}

fn find_entry_addr(loader: &elfload::ElfLoader) -> Result<usize, &'static str> {
    let e_entry = loader.elf_header.e_entry;

    for segment in loader.prog_headers.iter() {
        //                PT_LOAD
        if segment.p_type == 1 && segment.p_paddr == e_entry {
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
            cpu: CPU::new(entry_address, loader),
        }
    }

    pub fn simulation(&mut self) {
        use crate::cpu::execution::Execution;

        loop {
            fetch(&self.cpu)
                .decode()
                .execution(&mut self.cpu, &mut self.bus.dram);
        }
    }
} 
