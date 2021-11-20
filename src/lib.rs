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

        // rv32ui-p: 0x80000044
        // rv32ui-v: 0x80002308
        let break_point: Option<u32> = Some(0x80002308);

        loop {
            fetch(&self.cpu)
                .decode()
                .execution(&mut self.cpu);

            // debug code
            if break_point.unwrap_or(u32::MAX) == self.cpu.pc {
                std::process::exit(self.cpu.regs.read(Some(3)));
            }
        }
    }
} 
