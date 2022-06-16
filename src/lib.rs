pub mod system;
pub mod cpu;
pub mod bus;
pub mod elfload;

use cpu::CPU;
use cpu::fetch::fetch;

pub struct Emulator {
    pub cpu: cpu::CPU,
    break_point: Option<u32>,
    result_reg: Option<usize>,
}

impl Emulator {
    pub fn new(loader: elfload::ElfLoader, pk_load: Option<elfload::ElfLoader>,
               pc_from_cli: Option<u32>, break_point: Option<u32> , result_reg: Option<usize>) -> Emulator {
        Emulator {
            cpu: CPU::new(loader, pk_load, pc_from_cli),
            break_point,
            result_reg,
        }
    }

    pub fn emulation(&mut self) {
        use crate::cpu::execution::Execution;

        // rv32ui-p: 0x80000044, gp(3)
        // rv32ui-v: 0xffc02308, a0(10)

        loop {
            fetch(&mut self.cpu)
                .decode()
                .execution(&mut self.cpu);

            // debug code
            if self.break_point.unwrap_or(u32::MAX) == self.cpu.pc {
                if self.result_reg.is_some() {
                    std::process::exit(self.cpu.regs.read(self.result_reg));
                } else {
                    std::process::exit(0);
                }
            }
        }
    }
} 
