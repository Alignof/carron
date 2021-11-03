use crate::elfload;
use super::Device;

fn find_entry_addr(loader: &elfload::ElfLoader) -> Result<usize, &'static str> {
    let e_entry = loader.elf_header.e_entry;

    for segment in loader.prog_headers.iter() {
        //                PT_LOAD
        if segment.p_type == 1 && segment.p_paddr == e_entry {
            return Ok(segment.p_offset as usize);
        }
    }

    Err("entry address is not found.")
}

pub struct Dram {
    dram: Vec<u8>,
}

impl Dram {
    pub fn new(loader: elfload::ElfLoader) -> Dram {
        const DRAM_SIZE: u32 = 1024 * 1024 * 128; // 2^27
        let entry_address: usize = match find_entry_addr(&loader) {
            Ok(addr) => addr,
            Err(msg) => panic!("{}", msg),
        };
        let mmap_start = 0 as usize;
        let mmap_end = mmap_start + loader.mem_data.len()as usize;


        // load elf memory mapping 
        let mut new_dram = vec![0; DRAM_SIZE as usize];
        new_dram.splice(mmap_start..mmap_end, loader.mem_data[entry_address..mmap_end].iter().cloned());

        Dram {
            dram: new_dram,
        }
    }
}

impl Device for Dram {
    // get 1 byte
    fn raw_byte(&self, addr: usize) -> u8 {
        self.dram[addr]
    }

    // store
    fn store8(&mut self, addr: usize, data: i32) {
        self.dram[addr + 0] = ((data >> 0) & 0xFF) as u8;
    }

    fn store16(&mut self, addr: usize, data: i32) {
        self.dram[addr + 1] = ((data >> 8) & 0xFF) as u8;
        self.dram[addr + 0] = ((data >> 0) & 0xFF) as u8;
    }

    fn store32(&mut self, addr: usize, data: i32) {
        self.dram[addr + 3] = ((data >> 24) & 0xFF) as u8;
        self.dram[addr + 2] = ((data >> 16) & 0xFF) as u8;
        self.dram[addr + 1] = ((data >>  8) & 0xFF) as u8;
        self.dram[addr + 0] = ((data >>  0) & 0xFF) as u8;
    }


    // load
    fn load8(&self, addr: usize) -> i32 {
        self.dram[addr] as i8 as i32
    }

    fn load16(&self, addr: usize) -> i32 {
        ((self.dram[addr + 1] as u16) << 8 |
         (self.dram[addr + 0] as u16)) as i16 as i32
    }

    fn load32(&self, addr: usize) -> i32 {
        ((self.dram[addr + 3] as u32) << 24 |
         (self.dram[addr + 2] as u32) << 16 |
         (self.dram[addr + 1] as u32) <<  8 |
         (self.dram[addr + 0] as u32)) as i32
    }

    fn load_u8(&self, addr: usize) -> i32 {
        self.dram[addr] as i32
    }

    fn load_u16(&self, addr: usize) -> i32 {
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
        let dram = &mut Dram{ dram: vec![0; DRAM_SIZE as usize] };
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
        let dram = &mut Dram{ dram: vec![0; DRAM_SIZE as usize] };
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
        let dram = &mut Dram{ dram: vec![0; DRAM_SIZE as usize] };
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
        let dram = &mut Dram{ dram: vec![0; DRAM_SIZE as usize] };
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
        let dram = &mut Dram{ dram: vec![0; DRAM_SIZE as usize] };
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
