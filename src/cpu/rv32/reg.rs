use crate::cpu::instruction::reg2str;
use crate::log;

pub struct Register {
    regs: [u32; 32],
}

impl Register {
    pub fn new() -> Self {
        Register { regs: [0; 32] }
    }

    pub fn show(&self) {
        log::debugln!("=========================================== dump ============================================");
        for (num, reg) in self.regs.iter().enumerate() {
            log::debug!("{:>4}: 0x{:08x}\t", reg2str(num), reg);
            if (num + 1) % 4 == 0 {
                log::debugln!("")
            }
        }
        log::debugln!("=============================================================================================");
    }

    pub fn read(&self, src: Option<usize>) -> u32 {
        let src = src.unwrap();
        if src == 0 {
            0
        } else {
            self.regs[src]
        }
    }

    pub fn write(&mut self, dist: Option<usize>, src: u32) {
        let dist = dist.unwrap();
        if dist != 0 {
            self.regs[dist] = src;
        }
    }
}

impl Default for Register {
    fn default() -> Self {
        Self::new()
    }
}
