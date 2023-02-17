use crate::cpu::Cpu;
use crate::fesvr::FrontendServer;
use crate::log;
use crate::Arguments;
use libc::c_void;

fn memread(cpu: &mut Cpu, addr: u64, len: u64) -> Vec<u8> {
    let mut buf = Vec::new();
    for off in 0..len {
        buf.push(cpu.bus.load8(addr + off).unwrap() as u8);
    }

    buf
}

fn memwrite(cpu: &mut Cpu, addr: u64, len: usize, data: Vec<u8>) {
    for off in 0..len as u64 {
        cpu.bus
            .store8(addr + off, u64::from(data[off as usize]))
            .unwrap();
    }
}

fn sysret_errno(ret: i64) -> i64 {
    if ret == -1 {
        unsafe { -(*libc::__errno_location() as i64) }
    } else {
        ret
    }
}

impl FrontendServer {
    pub fn openat(
        &mut self,
        cpu: &mut Cpu,
        dirfd: u64,
        name_addr: u64,
        len: u64,
        flags: u64,
        mode: u64,
    ) -> i64 {
        log::infoln!("sys_openat(56)");
        let name: Vec<u8> = memread(cpu, name_addr, len);
        let name: &str = std::str::from_utf8(name.split_last().unwrap().1).unwrap();
        let fd = sysret_errno(unsafe {
            libc::openat(
                dirfd as i32,
                name.as_ptr() as *const i8,
                flags as i32,
                mode as i32,
            ) as i64
        });

        if fd < 0 {
            sysret_errno(-1)
        } else {
            self.fd_alloc(fd as u64)
        }
    }

    pub fn close(&mut self, fd: u64) -> i64 {
        log::infoln!("sys_close(57)");
        if unsafe { libc::close(self.fd_lookup(fd) as i32) } < 0 {
            return sysret_errno(-1);
        }

        self.fd_dealloc(fd);
        0
    }

    pub fn lseek(&self, fd: u64, ptr: u64, dir: u64) -> i64 {
        log::infoln!("sys_lseek(62)");

        sysret_errno(unsafe { libc::lseek(self.fd_lookup(fd) as i32, ptr as i64, dir as i32) })
    }

    pub fn read(&self, cpu: &mut Cpu, fd: u64, dst_addr: u64, len: u64) -> i64 {
        log::infoln!("sys_read(63)");
        let buf: Vec<u8> = vec![0; len as usize];
        let read_len = unsafe {
            libc::read(
                self.fd_lookup(fd) as i32,
                buf.as_ptr() as *mut c_void,
                len as usize,
            )
        };

        let ret_errno = sysret_errno(read_len as i64);
        if read_len > 0 {
            memwrite(cpu, dst_addr, read_len as usize, buf);
        }

        ret_errno
    }

    pub fn write(&self, cpu: &mut Cpu, fd: u64, dst_addr: u64, len: u64) -> i64 {
        log::infoln!("sys_write(64)");
        let buf = memread(cpu, dst_addr, len);
        let wrote_len = unsafe {
            libc::write(
                self.fd_lookup(fd) as i32,
                buf.as_ptr() as *const c_void,
                len as usize,
            )
        };

        sysret_errno(wrote_len as i64)
    }

    pub fn pread(&self, cpu: &mut Cpu, fd: u64, dst_addr: u64, len: u64, off: u64) -> i64 {
        log::infoln!("sys_pread(67)");
        let buf: Vec<u8> = vec![0; len as usize];
        let read_len = unsafe {
            libc::pread(
                self.fd_lookup(fd) as i32,
                buf.as_ptr() as *mut c_void,
                len as usize,
                off as i64,
            )
        };
        let ret_errno = sysret_errno(read_len as i64);
        if read_len > 0 {
            memwrite(cpu, dst_addr, read_len as usize, buf);
        }

        ret_errno
    }

    pub fn pwrite(&self, cpu: &mut Cpu, fd: u64, dst_addr: u64, len: u64, off: u64) -> i64 {
        log::infoln!("sys_pwrite(68)");
        let buf = memread(cpu, dst_addr, len);
        let wrote_len = unsafe {
            libc::pwrite(
                self.fd_lookup(fd) as i32,
                buf.as_ptr() as *const c_void,
                len as usize,
                off as i64,
            )
        };

        wrote_len as i64
    }

    pub fn fstatat(
        &self,
        cpu: &mut Cpu,
        dirfd: u64,
        name_addr: u64,
        len: u64,
        dst_addr: u64,
        flags: u64,
    ) -> i64 {
        log::infoln!("sys_fstatat(79)");
        let name: Vec<u8> = memread(cpu, name_addr, len);
        let name: &str = std::str::from_utf8(name.split_last().unwrap().1).unwrap();
        let (ret, rbuf) = unsafe {
            const PADDING: u64 = 0;
            let mut buf: libc::stat = std::mem::zeroed();
            let ret = libc::fstatat(
                dirfd as i32,
                name.as_ptr() as *const i8,
                &mut buf as *mut libc::stat,
                flags as i32,
            );
            (
                ret,
                vec![
                    buf.st_dev,
                    buf.st_ino,
                    buf.st_nlink << 32 | buf.st_mode as u64,
                    (buf.st_uid as u64) << 32 | buf.st_gid as u64,
                    buf.st_rdev,
                    PADDING,
                    buf.st_size as u64,
                    (buf.st_blksize as u32 as u64) << 32 | PADDING,
                    buf.st_blocks as u64,
                    buf.st_atime as u64,
                    PADDING,
                    buf.st_mtime as u64,
                    PADDING,
                    buf.st_ctime as u64,
                    PADDING,
                    (PADDING as u32 as u64) << 32 | PADDING as u32 as u64,
                ],
            )
        };

        let ret = sysret_errno(ret.into());
        if ret != -1 {
            let rbuf = rbuf
                .iter()
                .flat_map(|w| w.to_le_bytes().to_vec())
                .collect::<Vec<u8>>();

            memwrite(cpu, dst_addr, rbuf.len(), rbuf);
        }

        ret
    }

    pub fn fstat(&self, cpu: &mut Cpu, fd: u64, dst_addr: u64) -> i64 {
        log::infoln!("sys_fstat(80)");
        let (ret, rbuf) = unsafe {
            const PADDING: u64 = 0;
            let mut buf: libc::stat = std::mem::zeroed();
            let ret = libc::fstat(self.fd_lookup(fd) as i32, &mut buf as *mut libc::stat);
            (
                ret,
                vec![
                    buf.st_dev,
                    buf.st_ino,
                    buf.st_nlink << 32 | buf.st_mode as u64,
                    (buf.st_uid as u64) << 32 | buf.st_gid as u64,
                    buf.st_rdev,
                    PADDING,
                    buf.st_size as u64,
                    (buf.st_blksize as u32 as u64) << 32 | PADDING,
                    buf.st_blocks as u64,
                    buf.st_atime as u64,
                    PADDING,
                    buf.st_mtime as u64,
                    PADDING,
                    buf.st_ctime as u64,
                    PADDING,
                    (PADDING as u32 as u64) << 32 | PADDING as u32 as u64,
                ],
            )
        };

        let ret = sysret_errno(ret as i64);
        if ret != -1 {
            let rbuf = rbuf
                .iter()
                .flat_map(|w| w.to_le_bytes().to_vec())
                .collect::<Vec<u8>>();

            memwrite(cpu, dst_addr, rbuf.len(), rbuf);
        }

        ret
    }

    pub fn exit(&self, exit_code: &mut Option<i32>, code: u64) -> i64 {
        log::infoln!("sys_exit(93)");
        *exit_code = Some(code as i32);

        0
    }

    pub fn getmainvars(&self, cpu: &mut Cpu, args: &Arguments, dst_addr: u64, limit: u64) -> i64 {
        log::infoln!("sys_getmainvars(2011)");

        let arg_size = args.main_args.as_ref().map(|x| x.len()).unwrap_or(0);
        let mut words: Vec<u64> = vec![0; arg_size + 3];
        words[0] = arg_size as u64; // argc
        words[arg_size + 1] = 0; // argv[argc] = NULL
        words[arg_size + 2] = 0; // envp[0] = NULL

        if let Some(main_args) = &args.main_args {
            let mut sz = (arg_size as u64 + 3) * 8;
            for (i, arg) in main_args.iter().enumerate() {
                words[i + 1] = dst_addr + sz;
                sz += arg.len() as u64 + 1;
            }
        }

        let mut buf: Vec<u8> = words
            .iter()
            .flat_map(|w| w.to_le_bytes().to_vec())
            .collect::<Vec<u8>>();
        if let Some(argv) = &args.main_args {
            buf.append(
                &mut argv
                    .iter()
                    .cloned()
                    .flat_map(|x| format!("{x}\0").into_bytes())
                    .collect(),
            );
        }

        if buf.len() > limit as usize {
            return -12; // ENOMEM
        }

        memwrite(cpu, dst_addr, buf.len(), buf);
        0
    }
}
