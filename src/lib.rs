pub mod bus;
pub mod cmdline;
pub mod cpu;
pub mod elfload;
mod fesvr;
pub mod log;

use cmdline::Arguments;
use cpu::fetch::fetch;
use cpu::{TrapCause, CPU};
use fesvr::FrontendServer;

pub struct Emulator {
    pub cpu: cpu::CPU,
    frontend_server: FrontendServer,
    tohost_addr: Option<u32>,
    fromhost_addr: Option<u32>,
    args: Arguments,
    exit_code: Option<i32>,
}

impl Emulator {
    pub fn new(loader: elfload::ElfLoader, args: Arguments) -> Emulator {
        let (tohost_addr, fromhost_addr) = loader.get_host_addr();

        Emulator {
            cpu: CPU::new(loader, args.init_pc),
            frontend_server: FrontendServer::new(),
            tohost_addr,
            fromhost_addr,
            args,
            exit_code: None,
        }
    }

    fn exec_one_cycle(&mut self) -> Result<(), (Option<u32>, TrapCause, String)> {
        use crate::cpu::execution::Execution;

        self.cpu.check_interrupt()?;

        fetch(&mut self.cpu)?.decode()?.execution(&mut self.cpu)
    }

    pub fn emulation(&mut self) {
        loop {
            match self.exec_one_cycle() {
                Ok(()) => (),
                Err((addr, cause, msg)) => {
                    log::infoln!("[exception] {}", msg);
                    self.cpu.trap(addr.unwrap_or(self.cpu.pc), cause);
                }
            }

            if self.tohost_addr.is_some() && self.fromhost_addr.is_some() && self.check_tohost() {
                self.handle_syscall();
            }

            if let Some(break_point) = self.args.break_point {
                if break_point == self.cpu.pc {
                    self.exit_code =
                        Some(self.cpu.regs.read(Some(self.args.result_reg.unwrap_or(0))) as i32);
                }
            }

            if let Some(exit_code) = self.exit_code {
                std::process::exit(exit_code);
            }
        }
    }
}
