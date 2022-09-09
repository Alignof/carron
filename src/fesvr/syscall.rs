use libc::c_void;
use crate::CPU;
use crate::Arguments;

fn memread(cpu: &CPU, addr: u32, len: u64) -> Vec<u8> {
    let mut buf = Vec::new();
    for off in 0 .. len as u32 {
        buf.push(cpu.bus.load8(addr + off).unwrap() as u8);
    }

    buf
}

fn memwrite(cpu: &mut CPU, addr: u32, len: usize, data: Vec<u8>) {
    for off in 0 .. len as u32 {
        cpu.bus.store8(addr + off, data[off as usize] as u32).unwrap();
    }
}

pub fn openat(cpu: &CPU, dirfd: u64, name_addr: u64, len: u64, flags: u64, mode: u64) -> i64 {
    eprintln!("sys_openat(56)");
    let name: Vec<u8> = memread(cpu, name_addr as u32, len);
    let name: &str = std::str::from_utf8(name.split_last().unwrap().1).unwrap();
    let fd = unsafe { libc::openat(dirfd as i32, name.as_ptr() as *const i8, flags as i32, mode as i32) };

    if fd < 0 {
        -1
    } else {
        fd as i64
    }
}

pub fn close(fd: u64) -> i64 {
    eprintln!("sys_close(57)");
    unsafe { libc::close(fd as i32) };
    0
}

pub fn read(cpu: &mut CPU, fd: u64, dst_addr: u64, len: u64) -> i64 {
    eprintln!("sys_read(63)");
    let buf: Vec<u8> = vec![0; len as usize];
    let ret = unsafe { libc::read(fd as i32, buf.as_ptr() as *mut c_void, len as usize) };
    if ret > 0 {
        memwrite(cpu, dst_addr as u32, buf.len(), buf);
    }

    len as i64
}

pub fn write(cpu: &CPU, fd: u64, dst_addr: u64, len: u64) -> i64 {
    eprintln!("sys_write(64)");
    let buf = memread(cpu, dst_addr as u32, len);
    let len = unsafe { libc::write(fd as i32, buf.as_ptr() as *const c_void, len as usize) };

    len as i64
}

pub fn pread(cpu: &mut CPU, fd: u64, dst_addr: u64, len: u64, off: u64) -> i64 {
    eprintln!("sys_pread(67)");
    let buf: Vec<u8> = vec![0; len as usize];
    let ret = unsafe { libc::pread(fd as i32, buf.as_ptr() as *mut c_void, len as usize, off as i64) };
    if ret > 0 {
        memwrite(cpu, dst_addr as u32, buf.len(), buf);
    }

    len as i64
}

pub fn pwrite(cpu: &CPU, fd: u64, dst_addr: u64, len: u64, off: u64) -> i64 {
    eprintln!("sys_pwrite(68)");
    let buf = memread(cpu, dst_addr as u32, len);
    let len = unsafe { libc::pwrite(fd as i32, buf.as_ptr() as *const c_void, len as usize, off as i64) };

    len as i64
}

pub fn fstatat(cpu: &mut CPU, dirfd: u64, name_addr: u64, len: u64, dst_addr: u64, flags: u64) -> i64 {
    eprintln!("sys_fstatat(79)");
    let name: Vec<u8> = memread(cpu, name_addr as u32, len);
    let name: &str = std::str::from_utf8(name.split_last().unwrap().1).unwrap();
    let (ret, rbuf) = unsafe {
        let mut buf: libc::stat = std::mem::zeroed();
        let ret = libc::fstatat(dirfd as i32, name.as_ptr() as *const i8, &mut buf as *mut libc::stat, flags as i32);
        const PADDING: u64 = 0;
        (
            ret,
            vec![
                buf.st_dev,
                buf.st_ino,
                (buf.st_mode as u64) << 32 | buf.st_nlink as u64,
                (buf.st_uid as u64) << 32 | buf.st_gid as u64,
                buf.st_rdev,
                PADDING,
                buf.st_size as u64,
                (buf.st_blksize as u64) << 32 | PADDING,
                buf.st_blocks as u64,
                buf.st_atime as u64,
                PADDING,
                buf.st_mtime as u64,
                PADDING,
                buf.st_ctime as u64,
                PADDING,
                PADDING,
                PADDING,
            ]
        )
    };

    if ret != -1 {
        let rbuf = rbuf
        .iter()
        .flat_map(|w| w.to_le_bytes().to_vec())
        .collect::<Vec<u8>>();

        memwrite(cpu, dst_addr as u32, rbuf.len(), rbuf);
    }

    ret as i64
}

pub fn fstat(cpu: &mut CPU, fd: u64, dst_addr: u64) -> i64 {
    eprintln!("sys_fstat(80)");
    let (ret, rbuf) = unsafe {
        let mut buf: libc::stat = std::mem::zeroed();
        let ret = libc::fstat(fd as i32, &mut buf as *mut libc::stat);
        const PADDING: u64 = 0;
        (
            ret,
            vec![
                buf.st_dev,
                buf.st_ino,
                (buf.st_mode as u64) << 32 | buf.st_nlink as u64,
                (buf.st_uid as u64) << 32 | buf.st_gid as u64,
                buf.st_rdev,
                PADDING,
                buf.st_size as u64,
                (buf.st_blksize as u64) << 32 | PADDING,
                buf.st_blocks as u64,
                buf.st_atime as u64,
                PADDING,
                buf.st_mtime as u64,
                PADDING,
                buf.st_ctime as u64,
                PADDING,
                PADDING,
                PADDING,
            ]
        )
    };

    if ret != -1 {
        let rbuf = rbuf
        .iter()
        .flat_map(|w| w.to_le_bytes().to_vec())
        .collect::<Vec<u8>>();

        memwrite(cpu, dst_addr as u32, rbuf.len(), rbuf);
    }

    ret as i64
}

pub fn exit(exit_code: &mut Option<i32>, code: u64) -> i64 {
    eprintln!("sys_exit(93)");
    *exit_code = Some(code as i32);

    0
}

pub fn getmainvars(cpu: &mut CPU, args: &Arguments, dst_addr: u64, limit: u64) -> i64 {
    eprintln!("sys_getmainvars(2011)");

    let elfpath = args.filename.clone();
    let pkpath = args.pkpath.as_ref().unwrap().clone();
    let mut words: Vec<u64> = vec![0; 5];
    words[0] = 2; // argc
    words[1] = dst_addr + 8*5;
    words[2] = dst_addr + 8*5 + pkpath.len() as u64;
    words[3] = 0; // argv[argc] = NULL
    words[4] = 0; // envp[0] = NULL
    
    let mut buf: Vec<u8> = words
        .iter()
        .flat_map(|w| w.to_le_bytes().to_vec())
        .collect::<Vec<u8>>();
    buf.append(&mut pkpath.into_bytes());
    buf.append(&mut elfpath.into_bytes());

    if buf.len() > limit as usize {
        return -12;
    }

    memwrite(cpu, dst_addr as u32, buf.len(), buf);
    0
}
