pub mod system;
pub mod cpu;
pub mod bus;
pub mod elfload;

use cpu::{CPU, get_u16, get_u32, is_cinst};
use cpu::decode::Decode;
use cpu::execution::Execution;
use bus::Bus;
use bus::dram::Dram;

pub struct Simulator {
    pub loader: elfload::ElfLoader,
    pub cpu: cpu::CPU,
    pub bus: bus::Bus,
}

impl Simulator {
    pub fn new(loader: elfload::ElfLoader) -> Simulator {
        let entry_address = loader.elf_header.e_entry;

        Simulator {
            loader: loader,
            cpu: CPU::new(entry_address),
            bus: Bus::new(),
        }
    }

    pub fn simulation(&mut self) {
        let mmap = &mut self.loader.mem_data;

        loop {
            if is_cinst(mmap, self.cpu.pc as usize) {
                get_u16(mmap, self.cpu.pc as usize)
                    .decode()
                    .execution(&mut self.cpu, self.bus.dram);
            }else{
                get_u32(mmap, self.cpu.pc as usize)
                    .decode()
                    .execution(&mut self.cpu, self.bus.dram);
            }
        }
    }
} 
