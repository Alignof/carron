mod syscall;
use crate::Emulator;

impl Emulator {
    pub fn check_tohost(&self) -> bool {
        let tohost_addr = self.tohost_addr.unwrap();
        self.cpu.bus.load32(tohost_addr).expect("load from tohost addr failed") != 0
    }

    fn exec_syscall(&mut self, sysargs: [u64; 8]) -> i64 {
        match sysargs[0] {
            56 => syscall::openat(&self.cpu, sysargs[1], sysargs[2], sysargs[3], sysargs[4], sysargs[5]).unwrap_or(-1),
            57 => {eprintln!("do sys_close(57)"); 0},
            64 => syscall::write(&self.cpu, sysargs[1], sysargs[2], sysargs[3]).unwrap_or(-1),
            67 => syscall::pread(&mut self.cpu, sysargs[1], sysargs[2], sysargs[3], sysargs[4]).unwrap_or(-1),
            93 => {eprintln!("do sys_exit(93)"); 0},
            2011 => syscall::getmainvars(&mut self.cpu, &self.args, sysargs[1], sysargs[2]).unwrap_or(-12),
            _ => panic!("illegal syscall number"),
        }
    }

    pub fn handle_syscall(&mut self) {
        let tohost_addr = self.tohost_addr.unwrap();
        let fromhost_addr = self.fromhost_addr.unwrap();
        let tohost: u64 = self.cpu.bus.load64(tohost_addr).unwrap() as u64;
        self.cpu.bus.store64(tohost_addr, 0).unwrap();

        let syscall_addr: u32 = (tohost << 16 >> 16) as u32;
        let mut syscall_args: [u64; 8] = [
            self.cpu.bus.load64(syscall_addr).unwrap() as u64,
            self.cpu.bus.load64(syscall_addr +  8).unwrap() as u64,
            self.cpu.bus.load64(syscall_addr + 16).unwrap() as u64,
            self.cpu.bus.load64(syscall_addr + 24).unwrap() as u64,
            self.cpu.bus.load64(syscall_addr + 32).unwrap() as u64,
            self.cpu.bus.load64(syscall_addr + 40).unwrap() as u64,
            self.cpu.bus.load64(syscall_addr + 48).unwrap() as u64,
            self.cpu.bus.load64(syscall_addr + 56).unwrap() as u64,
        ];

        syscall_args[0] = self.exec_syscall(syscall_args) as u64;

        // store syscall to tohost
        for (i, s) in syscall_args.iter().enumerate() {
            self.cpu.bus.store64(syscall_addr + (i*8) as u32, *s as i64).unwrap();
        } 

        self.cpu.bus.store64(fromhost_addr, (tohost << 48 >> 48) as i64 | 1).unwrap();
    }
}
