pub mod system;
pub mod cpu;
pub mod bus;
pub mod elfload;
mod fesvr;

use cpu::{CPU, TrapCause};
use cpu::fetch::fetch;

pub struct Emulator {
    pub cpu: cpu::CPU,
    tohost_addr: Option<u32>,
    break_point: Option<u32>,
    result_reg: Option<usize>,
}

impl Emulator {
    pub fn new(loader: elfload::ElfLoader, pk_load: Option<elfload::ElfLoader>,
               pc_from_cli: Option<u32>, break_point: Option<u32> , result_reg: Option<usize>) -> Emulator {

        let tohost_addr = if let Some(ref pk) = pk_load {
            pk.get_tohost_addr()
        } else {
            loader.get_tohost_addr()
        };

        Emulator {
            cpu: CPU::new(loader, pk_load, pc_from_cli),
            tohost_addr,
            break_point,
            result_reg,
        }
    }

    fn exec_one_cycle(&mut self) -> Result<(), (Option<u32>, TrapCause, String)> {
        use crate::cpu::execution::Execution;
    
        self.cpu.check_interrupt()?;

        fetch(&mut self.cpu)?
            .decode()?
            .execution(&mut self.cpu)
    }

    pub fn emulation(&mut self) {
        // rv32ui-p: 0x80000044, gp(3)
        // rv32ui-v: 0xffc02308, a0(10)

        loop {
            match self.exec_one_cycle() {
                Ok(()) => (),
                Err((addr, cause, msg)) => {
                    self.cpu.trap(addr.unwrap_or(self.cpu.pc), cause);
                    eprintln!("{}", msg);
                },
            }

            let mut return_to_host = false;
            if let Some(tohost_addr) = self.tohost_addr {
                if self.check_tohost(tohost_addr) {
                    return_to_host = true;
                }
            }

            if let Some(break_point) = self.break_point {
                if break_point == self.cpu.pc {
                    return_to_host = true;
                }
            }

            if return_to_host {
                if self.result_reg.is_some() {
                    std::process::exit(self.cpu.regs.read(self.result_reg));
                } else {
                    std::process::exit(0);
                }
            }
        }
    }
} 

