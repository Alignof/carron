pub mod bus;
pub mod cmdline;
pub mod cpu;
pub mod elfload;
mod fesvr;
pub mod log;

use cmdline::Arguments;
use cpu::{Cpu, TrapCause};
use fesvr::FrontendServer;

#[derive(Copy, Clone)]
pub enum Isa {
    Rv32,
    Rv64,
}

pub struct Emulator {
    pub cpu: Cpu,
    frontend_server: FrontendServer,
    tohost_addr: Option<u64>,
    fromhost_addr: Option<u64>,
    args: Arguments,
    exit_code: Option<i32>,
}

impl Emulator {
    pub fn new(loader: elfload::ElfLoader, args: Arguments) -> Self {
        let isa = loader.target_arch();
        let (tohost_addr, fromhost_addr) = loader.get_host_addr(isa);

        Emulator {
            cpu: Cpu::new(loader, args.init_pc, isa),
            frontend_server: FrontendServer::new(),
            tohost_addr,
            fromhost_addr,
            args,
            exit_code: None,
        }
    }

    pub fn emulation(&mut self) {
        loop {
            match self.cpu.exec_one_cycle() {
                Ok(()) => (),
                Err((addr, cause, msg)) => {
                    log::infoln!("[exception: {:?}] {}", cause, msg);
                    self.cpu.trap(addr.unwrap_or(self.cpu.pc), cause);
                }
            }

            if self.tohost_addr.is_some() && self.fromhost_addr.is_some() && self.check_tohost() {
                self.handle_syscall();
            }

            if let Some(break_point) = self.args.break_point {
                if break_point == self.cpu.pc {
                    self.exit_code = Some(0);
                }
            }

            if let Some(exit_code) = self.exit_code {
                std::process::exit(exit_code);
            }
        }
    }
}
