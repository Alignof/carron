pub mod system;
pub mod cpu;
pub mod elfload;

use cpu::{CPU, get_u16, get_u32, is_cinst};
use cpu::decode::Decode;
use cpu::execution::Execution;

pub struct Simulator {
    pub loader: elfload::ElfLoader,
    pub cpu: CPU,
}

impl Simulator {
    pub fn simulation(&mut self) {
        let mmap = &(self.loader.mem_data);
        let mut inst_head = 0;

        loop {
            if is_cinst(mmap, inst_head as usize) {
                get_u16(mmap, inst_head as usize)
                    .decode()
                    .execution(&mut self.cpu);
                inst_head += 2;
            }else{
                get_u32(mmap, inst_head as usize)
                    .decode()
                    .execution(&mut self.cpu);
                inst_head += 4;
            }
        }
    }
} 
