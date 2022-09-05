use crate::CPU;

const RISCV_AT_FDCWD: i32 = -100;

fn memread(cpu: &CPU, addr: u32, len: u64) -> Vec<u8> {
    let mut buf = Vec::new();
    for off in 0 .. len as u32 {
        buf.push(cpu.bus.load8(addr + off).unwrap() as u8);
    }

    buf
}


pub fn write(cpu: &CPU, fd: u64, dst_addr: u64, len: u64) {
    let buf = memread(cpu, dst_addr as u32, len);
    eprintln!("do sys_write(64)");
}
