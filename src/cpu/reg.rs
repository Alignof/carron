use super::instruction::reg2str;
use crate::{log, Isa};

pub struct Register {
    regs: [u64; 32],
    mask: u64,
}

impl Register {
    pub fn new(isa: Isa) -> Register {
        Register {
            regs: [0; 32],
            mask: match isa {
                Isa::Rv32 => 0xFFFF,
                Isa::Rv64 => 0xFFFFFFFF,
            },
        }
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

    pub fn read(&self, src: Option<usize>) -> u64 {
        let src = src.unwrap();
        if src == 0 {
            0
        } else {
            self.mask | self.regs[src]
        }
    }

    pub fn write(&mut self, dist: Option<usize>, src: u64) {
        let dist = dist.unwrap();
        if dist != 0 {
            self.regs[dist] = self.mask | src;
        }
    }
}

impl Default for Register {
    fn default() -> Self {
        Self::new(Isa::Rv64)
    }
}
