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
        loop {
            if is_cinst(mmap, inst_head as usize) {
                get_u16(mmap, inst_head as usize)
                    .decode()
                    .execution();
                inst_head += 2;
            }else{
                get_u32(mmap, inst_head as usize)
                    .decode()
                    .execution();
                inst_head += 4;
            }
        }
    }
} 
