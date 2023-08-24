pub mod bus;
pub mod cmdline;
pub mod cpu;
pub mod elfload;
mod fesvr;
pub mod log;

use cmdline::Arguments;
use cpu::{Cpu, TrapCause};
use fesvr::FrontendServer;

const INTERLEAVE: u64 = 5000;
const INSNS_PER_RTC_TICK: u64 = 100;

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
}

impl Emulator {
    pub fn new(loader: elfload::ElfLoader, args: Arguments) -> Self {
        let isa = loader.target_arch();
        let (tohost_addr, fromhost_addr) = loader.get_host_addr(isa);

        Emulator {
            cpu: Cpu::new(loader, &args, isa),
            frontend_server: FrontendServer::new(),
            tohost_addr,
            fromhost_addr,
            args,
        }
    }

    pub fn emulation(&mut self) {
        loop {
            for _ in 0..INTERLEAVE {
                //*crate::log::INST_COUNT.lock().unwrap() += 1;
                //log::diffln!("0x{:016x}", self.cpu.pc());

                match self.cpu.exec_one_cycle() {
                    Ok(()) => (),
                    Err((addr, cause, msg)) => {
                        log::infoln!("[exception: {:?}] {}", cause, msg);
                        self.cpu.trap(addr.unwrap_or(self.cpu.pc()), cause);
                    }
                }

                //self.cpu.regs.show();

                //if self.tohost_addr.is_some() && self.fromhost_addr.is_some() && self.check_tohost()
                //{
                //    self.handle_syscall();
                //}
            }

            self.cpu.reservation_set = None;
            self.cpu.timer_increment(INTERLEAVE / INSNS_PER_RTC_TICK);
            self.cpu.bus.uart.tick(&mut self.cpu.bus.plic);
        }
    }
}
