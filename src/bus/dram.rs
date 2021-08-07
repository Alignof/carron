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

    // load
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

    pub fn load_u8(&self, addr: usize) -> u32 {
        self.dram[addr] as u32
    }

    pub fn load_u16(&self, addr: usize) -> u32 {
        (self.dram[addr + 1] << 8 |
         self.dram[addr + 0]) as u32
    }

    // store
    pub fn store8(&mut self, addr: usize, data: i32) {
        self.dram[addr + 0] = ((data >> 0) & 0xFF) as u8;
    }

    pub fn store16(&mut self, addr: usize, data: i32) {
        self.dram[addr + 1] = ((data >> 8) & 0xFF) as u8;
        self.dram[addr + 0] = ((data >> 0) & 0xFF) as u8;
    }

    pub fn store32(&mut self, addr: usize, data: i32) {
        self.dram[addr + 3] = ((data >> 24) & 0xFF) as u8;
        self.dram[addr + 2] = ((data >> 16) & 0xFF) as u8;
        self.dram[addr + 1] = ((data >>  8) & 0xFF) as u8;
        self.dram[addr + 0] = ((data >>  0) & 0xFF) as u8;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_store_test() {
        let dram = Dram::new();

        Dram::store16(dram, 13, 157);
        assert_eq!(157, Dram::load16(dram, 13));
                
    }
}
