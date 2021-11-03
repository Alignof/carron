pub mod system;
pub mod cpu;
pub mod bus;
pub mod elfload;

use cpu::CPU;
use cpu::fetch::fetch;

pub struct Simulator {
    pub cpu: cpu::CPU,
}

impl Simulator {
    pub fn new(loader: elfload::ElfLoader) -> Simulator {
        Simulator {
            cpu: CPU::new(loader),
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
