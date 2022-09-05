use crate::Emulator;

impl Emulator {
    pub fn check_tohost(&self) -> bool {
        let tohost_addr = self.tohost_addr.unwrap();
        self.cpu.bus.load32(tohost_addr).expect("load from tohost addr failed") != 0
    }

    fn exec_syscall(&self, syscall: [u64; 8]) {
        match syscall[0] {
            56 => eprintln!("do sys_openat(56)"),
            57 => eprintln!("do sys_close(57)"),
            64 => eprintln!("do sys_write(64)"),
            67 => eprintln!("do sys_pread(67)"),
            93 => eprintln!("do sys_exit(93)"),
            2011 => eprintln!("do sys_getmainvars(2011)"),
            _ => panic!("illegal syscall number"),
        }
    }

    pub fn handle_syscall(&mut self) {
        let tohost_addr = self.tohost_addr.unwrap();
        let fromhost_addr = self.fromhost_addr.unwrap();
        let tohost: u64 = self.cpu.bus.load64(tohost_addr).unwrap() as u64;
        self.cpu.bus.store64(tohost_addr, 0).unwrap();

        let syscall_addr: u32 = (tohost << 16 >> 16) as u32;
        let syscall: [u64; 8] = [
            self.cpu.bus.load64(syscall_addr).unwrap() as u64,
            self.cpu.bus.load64(syscall_addr + 1).unwrap() as u64,
            self.cpu.bus.load64(syscall_addr + 2).unwrap() as u64,
            self.cpu.bus.load64(syscall_addr + 3).unwrap() as u64,
            self.cpu.bus.load64(syscall_addr + 4).unwrap() as u64,
            self.cpu.bus.load64(syscall_addr + 5).unwrap() as u64,
            self.cpu.bus.load64(syscall_addr + 6).unwrap() as u64,
            self.cpu.bus.load64(syscall_addr + 7).unwrap() as u64,
        ];

        self.exec_syscall(syscall);

        // store syscall to tohost
        for (i, s) in syscall.iter().enumerate() {
            self.cpu.bus.store64(syscall_addr + i as u32, *s as i64).unwrap();
        } 

        self.cpu.bus.store64(fromhost_addr, (tohost << 48 >> 48) as i64 | 1).unwrap();
    }
}
