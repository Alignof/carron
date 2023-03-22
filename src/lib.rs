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
    exit_code: Option<i32>,
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
            exit_code: None,
        }
    }

    pub fn emulation(&mut self) {
        loop {
            for _ in 0..INTERLEAVE {
                *crate::log::INST_COUNT.lock().unwrap() += 1;
                log::diffln!("0x{:016x}", self.cpu.pc());

                //if *crate::log::INST_COUNT.lock().unwrap() == 1_3123_0113 + 1 {
                //    eprintln!("pc: {:#018x}", self.cpu.pc());
                //    eprintln!(
                //        "PLIC_BASE + 0x4(source 1 priority): {:#018x}",
                //        self.cpu.bus.load32(0xc00_0004).unwrap()
                //    );
                //    eprintln!(
                //        "PLIC_BASE + 0x1000(pending context 0): {:#018x}",
                //        self.cpu.bus.load32(0xc00_1000).unwrap()
                //    );
                //    eprintln!(
                //        "PLIC_BASE + 0x1004(pending context 1): {:#018x}",
                //        self.cpu.bus.load32(0xc00_1004).unwrap()
                //    );
                //    eprintln!(
                //        "PLIC_BASE + 0x2000(enable bits context 0): {:#018x}",
                //        self.cpu.bus.load32(0xc00_2000).unwrap()
                //    );
                //    eprintln!(
                //        "PLIC_BASE + 0x2080(enable bits context 1): {:#018x}",
                //        self.cpu.bus.load32(0xc00_2080).unwrap()
                //    );
                //    eprintln!(
                //        "PLIC_BASE + 0x20_0000(priority threshold context 0): {:#018x}",
                //        self.cpu.bus.load32(0xc20_0000).unwrap()
                //    );
                //    eprintln!(
                //        "PLIC_BASE + 0x20_1000(priority threshold context 1): {:#018x}",
                //        self.cpu.bus.load32(0xc20_1000).unwrap()
                //    );
                //    eprintln!(
                //        "PLIC_BASE + 0x20_0004(claim context 0): {:#018x}",
                //        self.cpu.bus.load32(0xc20_0004).unwrap()
                //    );
                //    eprintln!(
                //        "PLIC_BASE + 0x20_1004(claim context 1): {:#018x}",
                //        self.cpu.bus.load32(0xc20_1004).unwrap()
                //    );
                //    eprintln!("========================================");
                //}
                //if *crate::log::INST_COUNT.lock().unwrap() == 1_3000_0000 + 1 {
                //    eprintln!("pc: {:#018x}", self.cpu.pc());
                //}
                //if *crate::log::INST_COUNT.lock().unwrap() == 1_3250_0000 + 1 {
                //    eprintln!("pc: {:#018x}", self.cpu.pc());
                //}
                //if *crate::log::INST_COUNT.lock().unwrap() == 1_3500_0000 + 1 {
                //    eprintln!("pc: {:#018x}", self.cpu.pc());
                //}
                //if *crate::log::INST_COUNT.lock().unwrap() == 1_3750_0000 + 1 {
                //    eprintln!("pc: {:#018x}", self.cpu.pc());
                //}
                //if *crate::log::INST_COUNT.lock().unwrap() == 1_4000_0000 + 1 {
                //    eprintln!("pc: {:#018x}", self.cpu.pc());
                //}
                //if *crate::log::INST_COUNT.lock().unwrap() == 1_4250_0000 + 1 {
                //    eprintln!("pc: {:#018x}", self.cpu.pc());
                //}
                //if *crate::log::INST_COUNT.lock().unwrap() == 1_4500_0000 + 1 {
                //    eprintln!("pc: {:#018x}", self.cpu.pc());
                //}
                //if *crate::log::INST_COUNT.lock().unwrap() == 1_4750_0000 + 1 {
                //    eprintln!("pc: {:#018x}", self.cpu.pc());
                //}
                //if *crate::log::INST_COUNT.lock().unwrap() == 1_5000_0000 + 1 {
                //    eprintln!("pc: {:#018x}", self.cpu.pc());
                //}
                //if *crate::log::INST_COUNT.lock().unwrap() == 1_5250_0000 + 1 {
                //    eprintln!("pc: {:#018x}", self.cpu.pc());
                //    panic!("for debug");
                //}

                match self.cpu.exec_one_cycle() {
                    Ok(()) => (),
                    Err((addr, cause, msg)) => {
                        log::infoln!("[exception: {:?}] {}", cause, msg);
                        self.cpu.trap(addr.unwrap_or(self.cpu.pc()), cause);
                    }
                }

                log::diffln!(":");
                self.cpu.regs.show();

                if self.tohost_addr.is_some() && self.fromhost_addr.is_some() && self.check_tohost()
                {
                    self.handle_syscall();
                }

                if let Some(break_point) = self.args.break_point {
                    if break_point == self.cpu.pc() {
                        self.exit_code = Some(0);
                    }
                }

                if let Some(exit_code) = self.exit_code {
                    std::process::exit(exit_code);
                }
            }

            self.cpu.reservation_set = None;
            self.cpu.timer_increment(INTERLEAVE / INSNS_PER_RTC_TICK);
            self.cpu.bus.uart.tick();
        }
    }
}
