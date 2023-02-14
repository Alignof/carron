mod syscall;
use crate::Emulator;

pub struct FrontendServer {
    fd_table: Vec<Option<u64>>,
}

impl FrontendServer {
    pub fn new() -> Self {
        FrontendServer {
            fd_table: vec![Some(0), Some(1), Some(2)],
        }
    }

    pub fn fd_alloc(&mut self, fd: u64) -> i64 {
        if self.fd_table.iter().all(|x| x.is_some()) {
            self.fd_table.push(Some(fd));
            (self.fd_table.len() - 1) as i64
        } else {
            let index = self.fd_table.iter().position(|x| x.is_none()).unwrap();
            self.fd_table[index] = Some(fd);
            index as i64
        }
    }

    pub fn fd_dealloc(&mut self, fd: u64) {
        self.fd_table[fd as usize] = None;
    }

    pub fn fd_lookup(&self, fd: u64) -> i64 {
        if fd >= self.fd_table.len() as u64 {
            -1
        } else {
            self.fd_table[fd as usize].unwrap_or(u64::MAX) as i64
        }
    }
}

impl Emulator {
    pub fn check_tohost(&mut self) -> bool {
        let tohost_addr = self.tohost_addr.unwrap();
        self.cpu
            .bus
            .load32(tohost_addr)
            .expect("load from tohost addr failed")
            != 0
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
            56 => self.frontend_server.openat(
                &mut self.cpu,
                sysargs[1],
                sysargs[2],
                sysargs[3],
                sysargs[4],
                sysargs[5],
            ),
            57 => self.frontend_server.close(sysargs[1]),
            62 => self
                .frontend_server
                .lseek(sysargs[1], sysargs[2], sysargs[3]),
            63 => self
                .frontend_server
                .read(&mut self.cpu, sysargs[1], sysargs[2], sysargs[3]),
            64 => self
                .frontend_server
                .write(&mut self.cpu, sysargs[1], sysargs[2], sysargs[3]),
            67 => self.frontend_server.pread(
                &mut self.cpu,
                sysargs[1],
                sysargs[2],
                sysargs[3],
                sysargs[4],
            ),
            68 => self.frontend_server.pwrite(
                &mut self.cpu,
                sysargs[1],
                sysargs[2],
                sysargs[3],
                sysargs[4],
            ),
            79 => self.frontend_server.fstatat(
                &mut self.cpu,
                sysargs[1],
                sysargs[2],
                sysargs[3],
                sysargs[4],
                sysargs[5],
            ),
            80 => self
                .frontend_server
                .fstat(&mut self.cpu, sysargs[1], sysargs[2]),
            93 => self.frontend_server.exit(&mut self.exit_code, sysargs[1]),
            291 => panic!("sys_statx is not implemented"),
            1039 => panic!("sys_lstat is not implemented"),
            2011 => {
                self.frontend_server
                    .getmainvars(&mut self.cpu, &self.args, sysargs[1], sysargs[2])
            }
            _ => panic!("illegal syscall number"),
        }
    }

    pub fn handle_syscall(&mut self) {
        let tohost_addr = self.tohost_addr.unwrap();
        let fromhost_addr = self.fromhost_addr.unwrap();
        let tohost: u64 = self.cpu.bus.load64(tohost_addr).unwrap();
        self.cpu.bus.store64(tohost_addr, 0).unwrap();

        if tohost & 1 == 1 {
            self.exit_code = Some(tohost as i32);
        } else {
            let syscall_addr: u64 = tohost << 16 >> 16;
            let mut syscall_args: [u64; 8] = [
                self.cpu.bus.load64(syscall_addr).unwrap(),
                self.cpu.bus.load64(syscall_addr + 8).unwrap(),
                self.cpu.bus.load64(syscall_addr + 16).unwrap(),
                self.cpu.bus.load64(syscall_addr + 24).unwrap(),
                self.cpu.bus.load64(syscall_addr + 32).unwrap(),
                self.cpu.bus.load64(syscall_addr + 40).unwrap(),
                self.cpu.bus.load64(syscall_addr + 48).unwrap(),
                self.cpu.bus.load64(syscall_addr + 56).unwrap(),
            ];

            syscall_args[0] = self.exec_syscall(syscall_args) as u64;

            // store syscall to tohost
            for (i, s) in syscall_args.iter().enumerate() {
                self.cpu
                    .bus
                    .store64(syscall_addr + (i * 8) as u64, *s)
                    .unwrap();
            }

            if self.cpu.bus.load64(fromhost_addr).unwrap() == 0 {
                self.cpu
                    .bus
                    .store64(fromhost_addr, (tohost << 48 >> 48) | 1)
                    .unwrap();
            }
        }
    }
}
