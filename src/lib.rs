pub mod system;
pub mod cpu;
pub mod bus;
pub mod elfload;

use cpu::{CPU, get_u16, get_u32, is_cinst};
use cpu::decode::Decode;
use cpu::execution::Execution;
use bus::Bus;

pub struct Simulator {
    pub loader: elfload::ElfLoader,
    pub cpu: cpu::CPU,
    pub bus: bus::Bus,
}

impl Simulator {
    pub fn try_new(filename: &str) -> Simulator {
        let file = File::open(filename)?;
        let mapped_data = unsafe{Mmap::map(&file)?};

        let loader = match elfload::ElfLoader::try_new(&args.filename) {
            Ok(loader) => loader,
            Err(error) => {
                panic!("There was a problem opening the file: {:?}", error);
            }
        };
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
                    .execution(&mut self.cpu, &mut self.bus.dram);
            }else{
                get_u32(mmap, self.cpu.pc as usize)
                    .decode()
                    .execution(&mut self.cpu, &mut self.bus.dram);
            }
        }
    }
} 
