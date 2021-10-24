pub mod system;
pub mod cpu;
pub mod bus;
pub mod elfload;

use cpu::CPU;
use cpu::fetch::fetch;

pub struct Simulator<'a> {
    pub cpu: cpu::CPU<'a>,
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

impl Simulator<'_> {
    pub fn new(loader: elfload::ElfLoader) -> Simulator<'static> {
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
        let break_point: Option<usize> = Some(0x2308);

        loop {
            fetch(&self.cpu)
                .decode()
                .execution(&mut self.cpu);

            // debug code
            if break_point.unwrap_or(usize::MAX) == self.cpu.pc {
                std::process::exit(self.cpu.regs.read(Some(3)));
            }
        }
    }
} 
