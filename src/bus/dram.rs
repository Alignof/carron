pub struct Dram {
    dram: Vec<u8>,
}

impl Dram {
    pub fn new() -> Dram {
        const DRAM_SIZE: u32 = 65536; // 2^16
        let mut new_dram = vec![0; DRAM_SIZE as usize];

        Dram {
            dram: new_dram,
        }
    }
    
    pub fn load8(&self, addr: usize) -> i32 {
        self.dram[addr] as i32
    }

    pub fn load16(&self, addr: usize) -> i32 {
        (self.dram[addr + 1] << 8 |
         self.dram[addr + 0]) as i32
    }

    pub fn load32(&self, addr: usize) -> i32 {
        (self.dram[addr + 3] << 24 |
         self.dram[addr + 2] << 16 |
         self.dram[addr + 1] <<  8 |
         self.dram[addr + 0]) as i32
    }

    pub fn store8(&mut self, addr: usize, data: i32) {
        self.dram[addr + 0] = data as u8;
    }
}
