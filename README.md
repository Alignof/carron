# RV32IM\_simulator
[![Rust](https://github.com/Alignof/rv32im_simulator/actions/workflows/rust.yml/badge.svg)](https://github.com/Alignof/rv32im_simulator/actions/workflows/rust.yml)  
RV32IMC simulator in rust

## Feature
This simulator parses an ELF file that supports rv32imc and simulate execution environment of it.  
The simulator passed riscv-tests shown below.

- [x] rv32ui-p
- [x] rv32ui-v
- [x] rv32uc-p
- [ ] rv32uc-v
- [ ] rv32ua-p
- [ ] rv32ua-v

## Install
```zsh
git clone https://github.com/Alignof/rv32im_simulator.git
cd rv32im_simulator
cargo build --release
```

## Usage
```zsh
./rv32im_sim --help
rv32im_sim 0.9.0
n.takana <Alignof@outlook.com>
RV32IMC simulator

USAGE:
    rv32im_sim [OPTIONS] <filename>

ARGS:
    <filename>    ELF file path

OPTIONS:
    -e, --elfhead         Show ELF header
    -p, --program         Show all segments
    -s, --section         Show all sections
    -d, --disasem         Disassemble ELF
    -a, --all             Show all ELF data
        --pc <init_pc>    Set entry address as hex
    -h, --help            Print help information
    -V, --version         Print version information
```
