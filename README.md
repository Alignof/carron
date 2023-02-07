# carron
[![Rust](https://github.com/Alignof/carron/actions/workflows/rust.yml/badge.svg)](https://github.com/Alignof/Carron/actions/workflows/rust.yml)  
[![riscv_tests](https://github.com/Alignof/carron/actions/workflows/riscv_tests.yml/badge.svg)](https://github.com/Alignof/carron/actions/workflows/riscv_tests.yml)  
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

## Install
```zsh
git clone https://github.com/Alignof/carron.git
cd carron
cargo build --release
```

## Usage
```zsh
$ ./carron --help
carron 0.10.0
n.takana <Alignof@outlook.com>
RV32IMAC emulator

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

$ ./carron --pk $RISCV/riscv32-unknown-elf/bin/pk ./HelloWorld

In file HelloWorld
elfcheck: OK

bbl loader
hello world!
```
