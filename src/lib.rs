pub mod system;
pub mod cpu;
pub mod bus;
pub mod elfload;
mod fesvr;

use cpu::{CPU, TrapCause};
use cpu::fetch::fetch;
use system::Arguments;

pub struct Emulator {
    pub cpu: cpu::CPU,
    tohost_addr: Option<u32>,
    fromhost_addr: Option<u32>,
    args: Arguments,
}

impl Emulator {
    pub fn new(loader: elfload::ElfLoader, pk_load: Option<elfload::ElfLoader>,
               args: Arguments) -> Emulator {

        let (tohost_addr, fromhost_addr) = if let Some(ref pk) = pk_load {
            pk.get_host_addr()
        } else {
            loader.get_host_addr()
        };

        Emulator {
            cpu: CPU::new(loader, pk_load, args.init_pc),
            tohost_addr,
            fromhost_addr,
            args,
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
            if self.tohost_addr.is_some() && self.fromhost_addr.is_some() {
                if self.check_tohost() {
                    self.handle_syscall();
                }
            }

            if let Some(break_point) = self.args.break_point {
                if break_point == self.cpu.pc {
                    return_to_host = true;
                }
            }

            if return_to_host {
                if self.args.result_reg.is_some() {
                    std::process::exit(self.cpu.regs.read(self.args.result_reg));
                } else {
                    std::process::exit(0);
                }
            }
        }
    }
} 

