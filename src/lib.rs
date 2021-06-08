pub mod system;
pub mod cpu;
pub mod elfload;

use cpu::CPU;
use cpu::{get_u16, get_u32, is_cinst};

pub struct Simulator {
    pub loader: elfload::ElfLoader,
    pub cpu: CPU,
}

impl Simulator {
    pub fn simulation(&self) {
        let mmap = &(self.loader.mem_data);
        let inst_head = 0;

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
