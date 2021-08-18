pub mod system;
pub mod cpu;
pub mod bus;
pub mod elfload;

use cpu::CPU;
use cpu::{fetch, fetch_compressed};
use bus::Bus;

pub struct Simulator {
    pub cpu: cpu::CPU,
    pub bus: bus::Bus,
}

impl Simulator {
    pub fn new(loader: elfload::ElfLoader) -> Simulator {
        let entry_address = loader.elf_header.e_entry as usize;

        Simulator {
            cpu: CPU::new(entry_address),
            bus: Bus::new(loader),
        }
    }

    pub fn simulation(&mut self) {
        use crate::cpu::decode::Decode;
        use crate::cpu::execution::Execution;

        loop {
            let is_cinst: bool = self.bus.dram.raw_byte(self.cpu.pc) & 0x3 != 0x3;

            if is_cinst {
                fetch_compressed(&self.bus.dram, self.cpu.pc)
                    .decode()
                    .execution(&mut self.cpu, &mut self.bus.dram);
            } else {
                fetch(&self.bus.dram, self.cpu.pc)
                    .decode()
                    .execution(&mut self.cpu, &mut self.bus.dram);
            }
        }
    }
} 
