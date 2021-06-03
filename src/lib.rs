pub mod system;
pub mod elfload;
pub mod decode;

pub struct CPU {
    pub pc: u32,
    pub reg: [u32; 32],
}

pub struct Simulator {
    pub loader: elfload::ElfLoader,
    pub cpu: CPU,
}

impl Simulator {
    pub fn simulation(&self) {
        println!("Simulation...");
    }
} 
