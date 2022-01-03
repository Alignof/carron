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

        // rv32ui-p: 0x80000044, gp(3)
        // rv32ui-v: 0xffc02308, a0(10)
        let break_point: Option<u32> = Some(0xffc02308);
        let reg_result = 10;

        loop {
            fetch(&mut self.cpu)
                .decode()
                .execution(&mut self.cpu);

            // debug code
            if break_point.unwrap_or(u32::MAX) == self.cpu.pc {
                std::process::exit(self.cpu.regs.read(Some(reg_result)));
            }
        }
    }
} 
