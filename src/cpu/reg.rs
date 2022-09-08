use super::instruction::reg2str;
pub struct Register {
    regs: [u32; 32],
}

impl Register {
    pub fn new() -> Register {
        Register {
            regs: [0; 32],
        }
    }

    pub fn show(&self) {
        eprintln!("=========================================== dump ============================================");
        for (num, reg) in self.regs.iter().enumerate() {
            eprint!("{:>4}: 0x{:08x}\t", reg2str(num), reg);
            if (num + 1) % 4 == 0 { eprintln!() }
        }
        eprintln!("=============================================================================================");
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
