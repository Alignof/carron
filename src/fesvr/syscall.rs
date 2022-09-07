use std::io::Write;
use std::fs::File;
use std::os::unix::io::{FromRawFd, AsRawFd};

use crate::CPU;
use crate::Arguments;

const RISCV_AT_FDCWD: i32 = -100;

fn memread(cpu: &CPU, addr: u32, len: u64) -> Vec<u8> {
    let mut buf = Vec::new();
    for off in 0 .. len as u32 {
        buf.push(cpu.bus.load8(addr + off).unwrap() as u8);
    }

    buf
}

fn memwrite(cpu: &mut CPU, addr: u32, len: usize, data: Vec<u8>) {
    for off in 0 .. len as u32 {
        cpu.bus.store8(addr + off, data[off as usize] as i32).unwrap();
    }
}

pub fn openat(cpu: &CPU, _dirfd: u64, name_addr: u64, len: u64, _flags: u64, _mode: u64) -> Result<i64, std::io::Error> {
    eprintln!("do sys_openat(56)");
    let name: Vec<u8> = memread(&cpu, name_addr as u32, len);
    let name: String = std::str::from_utf8(name.split_last().unwrap().1).unwrap();
    let file = File::open(name)?;
    let fd = file.as_raw_fd();
    std::mem::forget(file);

    if dbg!(fd) < 0 {
        Err(std::io::Error::from(std::io::ErrorKind::NotFound))
    } else {
        Ok(fd as i64)
    }
}

pub fn write(cpu: &CPU, fd: u64, dst_addr: u64, len: u64) -> Result<i64, std::io::Error> {
    eprintln!("do sys_write(64)");
    let buf = memread(cpu, dst_addr as u32, len);
    let mut file = unsafe { File::from_raw_fd(fd as i32) };
    file.write_all(&buf)?;
    std::mem::forget(file);

    Ok(len as i64)
}

pub fn getmainvars(cpu: &mut CPU, args: &Arguments, dst_addr: u64, limit: u64) -> Result<i64, ()> {
    eprintln!("do sys_getmainvars(2011)");

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
        return Err(());
    }

    memwrite(cpu, dst_addr as u32, buf.len(), buf);
    Ok(0)
}
