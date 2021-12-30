use crate::elfload;
use super::Device;

pub struct Dram {
        dram: Vec<u8>,
    pub base_addr: u32,
}

impl Dram {
    pub fn new(loader: elfload::ElfLoader) -> Dram {
        const DRAM_SIZE: u32 = 1024 * 1024 * 128; // 2^27
        let vart_entry = loader.elf_header.e_entry;

        // create new dram 
        let mut new_dram = vec![0; DRAM_SIZE as usize];

        // load elf memory mapping 
        for segment in loader.prog_headers.iter() {
            let dram_start = (segment.p_paddr - vart_entry) as usize;
            let mmap_start = (segment.p_offset) as usize;
            let dram_end = dram_start + segment.p_filesz as usize;
            let mmap_end = (segment.p_offset + segment.p_filesz) as usize;
            dbg!(loader.mem_data.len());
            dbg!(dram_start);
            dbg!(dram_end);
            dbg!(mmap_start);
            dbg!(mmap_end);

            new_dram.splice(
                dram_start .. dram_end,
                loader.mem_data[mmap_start .. mmap_end].iter().cloned()
            );
        }

        Dram {
            dram: new_dram,
            base_addr: vart_entry,
        }
    }
}

impl Device for Dram {
    // address to raw index
    fn addr2index(&self, addr: u32) -> usize {
        if addr < self.base_addr {
            panic!("invalid address for Dram: {}", addr);
        }

        (addr - self.base_addr) as usize
    }

    // get 1 byte
    fn raw_byte(&self, addr: u32) -> u8 {
        let addr = self.addr2index(addr);
        self.dram[addr]
    }

    // store
    fn store8(&mut self, addr: u32, data: i32) {
        let addr = self.addr2index(addr);
        self.dram[addr + 0] = ((data >> 0) & 0xFF) as u8;
    }

    fn store16(&mut self, addr: u32, data: i32) {
        let addr = self.addr2index(addr);
        self.dram[addr + 1] = ((data >> 8) & 0xFF) as u8;
        self.dram[addr + 0] = ((data >> 0) & 0xFF) as u8;
    }

    fn store32(&mut self, addr: u32, data: i32) {
        let addr = self.addr2index(addr);
        self.dram[addr + 3] = ((data >> 24) & 0xFF) as u8;
        self.dram[addr + 2] = ((data >> 16) & 0xFF) as u8;
        self.dram[addr + 1] = ((data >>  8) & 0xFF) as u8;
        self.dram[addr + 0] = ((data >>  0) & 0xFF) as u8;
    }


    // load
    fn load8(&self, addr: u32) -> i32 {
        let addr = self.addr2index(addr);
        self.dram[addr] as i8 as i32
    }

    fn load16(&self, addr: u32) -> i32 {
        let addr = self.addr2index(addr);
        ((self.dram[addr + 1] as u16) << 8 |
         (self.dram[addr + 0] as u16)) as i16 as i32
    }

    fn load32(&self, addr: u32) -> i32 {
        let addr = self.addr2index(addr);
        ((self.dram[addr + 3] as u32) << 24 |
         (self.dram[addr + 2] as u32) << 16 |
         (self.dram[addr + 1] as u32) <<  8 |
         (self.dram[addr + 0] as u32)) as i32
    }

    fn load_u8(&self, addr: u32) -> i32 {
        let addr = self.addr2index(addr);
        self.dram[addr] as i32
    }

    fn load_u16(&self, addr: u32) -> i32 {
        let addr = self.addr2index(addr);
        ((self.dram[addr + 1] as u32) << 8 |
         (self.dram[addr + 0] as u32)) as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    const DRAM_SIZE: u32 = 1024 * 1024 * 128; // 2^27

    #[test]
    fn load_store_u8_test() {
        let dram = &mut Dram{ dram: vec![0; DRAM_SIZE as usize], base_addr: 0 };
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
        let dram = &mut Dram{ dram: vec![0; DRAM_SIZE as usize], base_addr: 0 };
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
        let dram = &mut Dram{ dram: vec![0; DRAM_SIZE as usize], base_addr: 0 };
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
        let dram = &mut Dram{ dram: vec![0; DRAM_SIZE as usize], base_addr: 0 };
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
        let dram = &mut Dram{ dram: vec![0; DRAM_SIZE as usize], base_addr: 0 };
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
