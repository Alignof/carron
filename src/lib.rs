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
        let entry_address = loader.elf_header.e_entry;

        Simulator {
            cpu: CPU::new(entry_address),
            bus: Bus::new(loader),
        }
    }

    pub fn simulation(&mut self) {
        loop {
            // before
            if is_cinst(mmap, self.cpu.pc) {
                get_u16(mmap, self.cpu.pc)
                    .decode()
                    .execution(&mut self.cpu, &mut self.bus.dram);
            }else{
                get_u32(mmap, self.cpu.pc as usize)
                    .decode()
                    .execution(&mut self.cpu, &mut self.bus.dram);
            }

            // after
            fetch(self.bus.dram, &mut self.cpu.pc) // return u16 or u32
                .decode()
                .execution(&mut self.cpu, &mut self.bus.dram);
        }
    }
} 
