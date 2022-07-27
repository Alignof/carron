use crate::{elfload, TrapCause};
use super::Device;

pub struct Dram {
        dram: Vec<u8>,
    pub base_addr: u32,
        size: usize,
}

impl Dram {
    pub fn new(loader: elfload::ElfLoader) -> Dram {
        const DRAM_SIZE: u32 = 1024 * 1024 * 128; // 2^27
        let virt_entry = match loader.get_entry_point() {
            Ok(addr) => addr,
            Err(()) => panic!("entry point not found."),
        };

        // create new dram 
        let mut new_dram = vec![0; DRAM_SIZE as usize];

        // load elf memory mapping 
        for segment in loader.prog_headers.iter() {
            if segment.is_loadable() {
                let dram_start = (segment.p_paddr - virt_entry) as usize;
                let mmap_start = (segment.p_offset) as usize;
                let dram_end = dram_start + segment.p_filesz as usize;
                let mmap_end = (segment.p_offset + segment.p_filesz) as usize;

                new_dram.splice(
                    dram_start .. dram_end,
                    loader.mem_data[mmap_start .. mmap_end].iter().cloned()
                );
            }
        }

        let dram_size = new_dram.len();
        Dram {
            dram: new_dram,
            base_addr: virt_entry,
            size: dram_size,
        }
    }

    pub fn new_with_pk(loader: elfload::ElfLoader, pk_load: &elfload::ElfLoader) -> Dram {
        const DRAM_SIZE: u32 = 1024 * 1024 * 128; // 2^27
        let pk_virt_entry = match pk_load.get_entry_point() {
            Ok(addr) => addr,
            Err(()) => panic!("entry point not found."),
        };

        // create new dram 
        let mut new_dram = vec![0; DRAM_SIZE as usize];

        // load proxy kernel 
        for segment in pk_load.prog_headers.iter() {
            if segment.is_loadable() {
                let dram_start = (segment.p_paddr - pk_virt_entry) as usize;
                let mmap_start = segment.p_offset as usize;
                let dram_end = dram_start + segment.p_filesz as usize;
                let mmap_end = (segment.p_offset + segment.p_filesz) as usize;

                new_dram.splice(
                    dram_start .. dram_end,
                    pk_load.mem_data[mmap_start .. mmap_end].iter().cloned()
                );
            }
        }

        let final_segment = pk_load.prog_headers.last().unwrap();
        let user_base_addr = (final_segment.p_offset + final_segment.p_filesz) as u32;
        let align = 0x1000;
        let user_base_addr = ((user_base_addr + (align - 1)) / align) * align;
        let virt_entry = match loader.get_entry_point() {
            Ok(addr) => addr,
            Err(()) => panic!("entry point not found."),
        };

        // load user program 
        for segment in loader.prog_headers.iter() {
            if segment.is_loadable() {
                let dram_start = segment.p_paddr - virt_entry + user_base_addr;
                let mmap_start = segment.p_offset as usize;
                let dram_end = dram_start + segment.p_filesz + user_base_addr;
                let mmap_end = (segment.p_offset + segment.p_filesz) as usize;

                new_dram.splice(
                    dram_start as usize .. dram_end as usize,
                    loader.mem_data[mmap_start .. mmap_end].iter().cloned()
                );
            }
        }

        let dram_size = new_dram.len();
        Dram {
            dram: new_dram,
            base_addr: pk_virt_entry,
            size: dram_size,
        }
    }
}

impl Device for Dram {
    // address to raw index
    fn addr2index(&self, addr: u32, cause: TrapCause) -> Result<usize, (Option<i32>, TrapCause, String)> {
        if self.base_addr <= addr && addr <= self.base_addr + self.size as u32 {
            Ok((addr - self.base_addr) as usize)
        } else {
            Err((
                Some(addr as i32),
                cause,
                format!("addr is out of dram address space 0x{:x}/0x{:x}", addr, self.base_addr + self.size as u32)
            ))
        }
    }

    // get 1 byte
    fn raw_byte(&self, addr: u32) -> u8 {
        let addr = self.addr2index(addr, TrapCause::InstPageFault).unwrap();
        self.dram[addr]
    }

    // store
    fn store8(&mut self, addr: u32, data: i32) -> Result<(), (Option<i32>, TrapCause, String)> {
        let addr = self.addr2index(addr, TrapCause::StoreAMOPageFault)?;
        self.dram[addr + 0] = ((data >> 0) & 0xFF) as u8;
        Ok(())
    }

    fn store16(&mut self, addr: u32, data: i32) -> Result<(), (Option<i32>, TrapCause, String)> {
        let addr = self.addr2index(addr, TrapCause::StoreAMOPageFault)?;
        self.dram[addr + 1] = ((data >> 8) & 0xFF) as u8;
        self.dram[addr + 0] = ((data >> 0) & 0xFF) as u8;
        Ok(())
    }

    fn store32(&mut self, addr: u32, data: i32) -> Result<(), (Option<i32>, TrapCause, String)> {
        let addr = self.addr2index(addr, TrapCause::StoreAMOPageFault)?;
        self.dram[addr + 3] = ((data >> 24) & 0xFF) as u8;
        self.dram[addr + 2] = ((data >> 16) & 0xFF) as u8;
        self.dram[addr + 1] = ((data >>  8) & 0xFF) as u8;
        self.dram[addr + 0] = ((data >>  0) & 0xFF) as u8;
        Ok(())
    }


    // load
    fn load8(&self, addr: u32) -> Result<i32, (Option<i32>, TrapCause, String)> {
        let addr = self.addr2index(addr, TrapCause::LoadPageFault)?;
        Ok(self.dram[addr] as i8 as i32)
    }

    fn load16(&self, addr: u32) -> Result<i32, (Option<i32>, TrapCause, String)> {
        let addr = self.addr2index(addr, TrapCause::LoadPageFault)?;
        Ok(((self.dram[addr + 1] as u16) << 8 |
         (self.dram[addr + 0] as u16)) as i16 as i32)
    }

    fn load32(&self, addr: u32) -> Result<i32, (Option<i32>, TrapCause, String)> {
        let addr = self.addr2index(addr, TrapCause::LoadPageFault)?;
        Ok(((self.dram[addr + 3] as u32) << 24 |
         (self.dram[addr + 2] as u32) << 16 |
         (self.dram[addr + 1] as u32) <<  8 |
         (self.dram[addr + 0] as u32)) as i32)
    }

    fn load_u8(&self, addr: u32) -> Result<i32, (Option<i32>, TrapCause, String)> {
        let addr = self.addr2index(addr, TrapCause::LoadPageFault)?;
        Ok(self.dram[addr] as i32)
    }

    fn load_u16(&self, addr: u32) -> Result<i32, (Option<i32>, TrapCause, String)> {
        let addr = self.addr2index(addr, TrapCause::LoadPageFault)?;
        Ok(((self.dram[addr + 1] as u32) << 8 |
         (self.dram[addr + 0] as u32)) as i32)
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
