pub mod system;
pub mod elfload;
pub mod cpu;
pub mod decode;

pub struct Simulator {
    pub loader: elfload::ElfLoader,
    pub cpu: cpu::CPU,
}

