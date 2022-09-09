mod syscall;
use crate::Emulator;

impl Emulator {
    pub fn check_tohost(&self) -> bool {
        let tohost_addr = self.tohost_addr.unwrap();
        self.cpu.bus.load32(tohost_addr).expect("load from tohost addr failed") != 0
    }

    fn exec_syscall(&mut self, sysargs: [u64; 8]) -> i64 {
        match sysargs[0] {
            17 => panic!("sys_getcwd is not implemented"),
            25 => panic!("sys_fcntl is not implemented"),
            34 => panic!("sys_mkdirat is not implemented"),
            35 => panic!("sys_unlinkat is not implemented"),
            37 => panic!("sys_linkat is not implemented"),
            38 => panic!("sys_renameat is not implemented"),
            46 => panic!("sys_ftruncate is not implemented"),
            48 => panic!("sys_faccessat is not implemented"),
            49 => panic!("sys_chdir is not implemented"),
            56 => syscall::openat(&self.cpu, sysargs[1], sysargs[2], sysargs[3], sysargs[4], sysargs[5]),
            57 => syscall::close(sysargs[1]),
            62 => panic!("sys_lseek is not implemented"),
            63 => syscall::read(&mut self.cpu, sysargs[1], sysargs[2], sysargs[3]),
            64 => syscall::write(&self.cpu, sysargs[1], sysargs[2], sysargs[3]),
            67 => syscall::pread(&mut self.cpu, sysargs[1], sysargs[2], sysargs[3], sysargs[4]),
            68 => syscall::pwrite(&mut self.cpu, sysargs[1], sysargs[2], sysargs[3], sysargs[4]),
            79 => syscall::fstatat(&mut self.cpu, sysargs[1], sysargs[2], sysargs[3], sysargs[4], sysargs[5]),
            80 => syscall::fstat(&mut self.cpu, sysargs[1], sysargs[2]),
            93 => syscall::exit(&mut self.exit_code, sysargs[1]),
            291 => panic!("sys_statx is not implemented"),
            1039 => panic!("sys_lstat is not implemented"),
            2011 => syscall::getmainvars(&mut self.cpu, &self.args, sysargs[1], sysargs[2]),
            _ => panic!("illegal syscall number"),
        }
    }

    pub fn handle_syscall(&mut self) {
        let tohost_addr = self.tohost_addr.unwrap();
        let fromhost_addr = self.fromhost_addr.unwrap();
        let tohost: u64 = self.cpu.bus.load64(tohost_addr).unwrap() as u64;
        self.cpu.bus.store64(tohost_addr, 0).unwrap();

        if tohost & 1 == 1 {
            self.exit_code = Some(tohost as i32);
        } else {
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
}
