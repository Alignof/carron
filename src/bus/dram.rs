use crate::elfload;

pub struct Dram {
    dram: Vec<u8>,
}

impl Dram {
    pub fn new(loader: elfload::ElfLoader) -> Dram {
        const DRAM_SIZE: u32 = 1024 * 1024 * 128; // 2^27
        let mmap_start = loader.elf_header.e_entry as usize;
        let mmap_end = mmap_start + loader.mem_data.len() as usize;

        // load elf memory mapping 
        let new_dram = vec![0; DRAM_SIZE as usize];
        new_dram.splice(mmap_start..mmap_end, loader.mem_data.iter().cloned());

        Dram {
            dram: new_dram,
        }
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


    // load
    pub fn load8(&self, addr: usize) -> i32 {
        self.dram[addr] as i8 as i32
    }

    pub fn load16(&self, addr: usize) -> i32 {
        ((self.dram[addr + 1] as u16) << 8 |
         (self.dram[addr + 0] as u16)) as i16 as i32
    }

    pub fn load32(&self, addr: usize) -> i32 {
        ((self.dram[addr + 3] as u32) << 24 |
         (self.dram[addr + 2] as u32) << 16 |
         (self.dram[addr + 1] as u32) <<  8 |
         (self.dram[addr + 0] as u32)) as i32
    }

    pub fn load_u8(&self, addr: usize) -> i32 {
        self.dram[addr] as i32
    }

    pub fn load_u16(&self, addr: usize) -> i32 {
        ((self.dram[addr + 1] as u32) << 8 |
         (self.dram[addr + 0] as u32)) as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_store_u8_test() {
        let dram = &mut Dram::new();
        let mut addr = 0;
        let mut test_8 = |data: i32| {
            Dram::store8(dram, addr, data);
            assert_eq!(data, Dram::load_u8(dram, addr));
            addr += 2;
        };

        test_8(0);
        test_8(17);
        test_8(0b01111111);

        Dram::store8(dram, addr, -42);
        assert_ne!(-42, Dram::load_u8(dram, addr));
        Dram::store16(dram, addr, -42);
        assert_eq!(214, Dram::load_u8(dram, addr));
    }

    #[test]
    fn load_store_8_test() {
        let dram = &mut Dram::new();
        let mut addr = 0;
        let mut test_8 = |data: i32| {
            Dram::store8(dram, addr, data);
            assert_eq!(data, Dram::load8(dram, addr));
            addr += 2;
        };

        test_8(0);
        test_8(17);
        test_8(0b01111111);
        test_8(-42);

        Dram::store8(dram, addr, 0b10000000);
        assert_ne!(0b10000000, Dram::load8(dram, addr));
    }

    #[test]
    fn load_store_16_test() {
        let dram = &mut Dram::new();
        let mut addr = 0;
        let mut test_16 = |data: i32| {
            Dram::store16(dram, addr, data);
            assert_eq!(data, Dram::load16(dram, addr));
            addr += 2;
        };

        test_16(0);
        test_16(157);
        test_16(255);
        test_16(-42);
        test_16(0b0111111111111111);

        Dram::store16(dram, addr, 0b1000000010000000);
        assert_ne!(0b1000000010000000, Dram::load16(dram, addr));
    }

    #[test]
    fn load_store_u16_test() {
        let dram = &mut Dram::new();
        let mut addr = 0;
        let mut test_u16 = |data: i32| {
            Dram::store16(dram, addr, data);
            assert_eq!(data, Dram::load_u16(dram, addr));
            addr += 2;
        };

        test_u16(0);
        test_u16(157);
        test_u16(255);
        test_u16(0b0111111111111111);

        Dram::store16(dram, addr, -42);
        assert_ne!(-42, Dram::load_u16(dram, addr));
        Dram::store16(dram, addr, -42);
        assert_eq!(65494, Dram::load_u16(dram, addr));
    }

    #[test]
    #[allow(overflowing_literals)]
    fn load_store_32_test() {
        let dram = &mut Dram::new();
        let mut addr = 0;
        let mut test_32 = |data: i32| {
            Dram::store32(dram, addr, data);
            assert_eq!(data, Dram::load32(dram, addr));
            addr += 2;
        };

        test_32(0);
        test_32(157);
        test_32(255);
        test_32(-42);
        test_32(0b1000000010000000);
        test_32(0b10000000100000001000000010000000);
    }
}
