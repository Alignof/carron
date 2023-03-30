# carron
[![Rust](https://github.com/Alignof/carron/actions/workflows/rust.yml/badge.svg)](https://github.com/Alignof/Carron/actions/workflows/rust.yml) 
[![riscv\_tests](https://github.com/Alignof/carron/actions/workflows/riscv_tests.yml/badge.svg)](https://github.com/Alignof/carron/actions/workflows/riscv_tests.yml)  
RV64IMAC emulator in rust

## Feature
This emulator parses an ELF file that supports rv32imac/rv64imac and emulate execution environment of it.    

The emulator passed riscv-tests shown below.

- [x] rv32mi-p
- [x] rv32si-p
- [x] rv32ui-p
- [x] rv32ui-v
- [x] rv32um-p
- [x] rv32um-v
- [x] rv32uc-p
- [x] rv32uc-v
- [x] rv32ua-p
- [x] rv32ua-v
- [x] rv64mi-p
- [x] rv64si-p
- [x] rv64ui-p
- [x] rv64ui-v
- [x] rv64um-p
- [x] rv64um-v
- [x] rv64uc-p
- [x] rv64uc-v
- [x] rv64ua-p
- [x] rv64ua-v

This emulator can also run **Linux** and **self-hosted binaries of [cc_sakura](https://github.com/Alignof/cc_sakura) my handmade C compiler**.

## Install
```zsh
git clone https://github.com/Alignof/carron.git
cd carron
cargo build --release
```

## Usage
```zsh
$ ./carron --help
carron 1.1.0
n.takana <Alignof@outlook.com>
RV64IMAC emulator

USAGE:
    carron [OPTIONS] <filename> [main_args]...

ARGS:
    <filename>        ELF file path
    <main_args>...

OPTIONS:
    -e, --elfhead                         Show ELF header
    -p, --program                         Show all segments
    -s, --section                         Show all sections
    -d, --disasem                         Disassemble ELF
    -a, --all                             Show all ELF data
        --pk <proxy_kernel>               Run with proxy kernel
        --pc <init_pc>                    Set entry address as hex
        --break_point <address>           Set break point as hex
        --result_reg <register_number>    Set result register
        --loglv <log_level>               Set log level
    -h, --help                            Print help information
    -V, --version                         Print version information
```


Hello World
```zsh
$ ./carron --pk $RISCV/riscv32-unknown-elf/bin/pk ./HelloWorld

In file HelloWorld
elfcheck: OK

bbl loader
hello world!
```

boot Linux
```
$ git clone https://github.com/buildroot/buildroot
$ cd buildroot
$ make spike_riscv64_defconfig
$ make menuconfig # disable F extension
$ make -j $(nproc)
$ ./carron --release -- --kernel /path/to/Image --initrd /path/to/rootfs.cpio /path/to/fw_jump.elf
In file /home/takana/riscv-toolchain/buildroot/output/images/fw_jump.elf
elfcheck: OK


OpenSBI v1.2
   ____                    _____ ____ _____
  / __ \                  / ____|  _ \_   _|
 | |  | |_ __   ___ _ __ | (___ | |_) || |
 | |  | | '_ \ / _ \ '_ \ \___ \|  _ < | |
 | |__| | |_) |  __/ | | |____) | |_) || |_
  \____/| .__/ \___|_| |_|_____/|____/_____|
        | |
        |_|

......

[    0.091080] Freeing unused kernel image (initmem) memory: 2144K
[    0.097300] Run /init as init process
Starting syslogd: OK
Starting klogd: OK
Running sysctl: OK
Saving 256 bits of non-creditable seed for next boot
Starting network: OK

Welcome to Buildroot
buildroot login:
```
