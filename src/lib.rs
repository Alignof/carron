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

impl Simulator {
    pub fn new(loader: elfload::ElfLoader) -> Simulator {
        //let entry_address = loader.elf_header.e_entry as usize;
        let entry_address = 0 as usize;

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
