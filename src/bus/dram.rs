use memmap::Mmap;
use std::fs::File;

use super::Device;
use crate::{elfload, Arguments, Isa, TrapCause};

pub struct Dram {
    dram: Vec<u8>,
    pub base_addr: u64,
    size: usize,
    pub initrd_start: Option<usize>,
    pub initrd_end: Option<usize>,
}

impl Dram {
    pub fn new(loader: elfload::ElfLoader, args: &Arguments, isa: Isa) -> Self {
        const DRAM_SIZE: usize = 1024 * 1024 * 128; // 2^27
        let virt_entry = loader.get_entry_point().expect("entry point not found.");

        // create new dram
        let mut new_dram = vec![0; DRAM_SIZE];

        // load elf memory mapping
        for segment in loader.prog_headers.iter() {
            if segment.is_loadable() {
                let (offset, paddr) = segment.offset_and_addr();
                let dram_start = (paddr - virt_entry) as usize;
                let mmap_start = (offset) as usize;
                let dram_end = dram_start + segment.p_filesz() as usize;
                let mmap_end = (offset + segment.p_filesz()) as usize;

                new_dram.splice(
                    dram_start..dram_end,
                    loader.mem_data[mmap_start..mmap_end].iter().cloned(),
                );
            }
        }

        if let Some(path) = &args.kernel_path {
            let file = File::open(path).unwrap();
            let mapped_kernel = unsafe { Mmap::map(&file).unwrap() };
            let kernel_offset = match isa {
                Isa::Rv32 => 0x400000,
                Isa::Rv64 => 0x200000,
            };
            new_dram.splice(
                kernel_offset..kernel_offset + mapped_kernel.len(),
                mapped_kernel.iter().cloned(),
            );
        }

        let mut initrd_start: Option<usize> = None;
        let mut initrd_end: Option<usize> = None;
        if let Some(path) = &args.initrd_path {
            let file = File::open(path).unwrap();
            let mapped_initrd = unsafe { Mmap::map(&file).unwrap() };

            const INITRD_END: usize = 0x8000_0000 + DRAM_SIZE - 0x1000;
            let initrd_head = INITRD_END - mapped_initrd.len();
            let initrd_offset = dbg!(initrd_head - virt_entry as usize);
            initrd_start = Some(INITRD_END - mapped_initrd.len());
            initrd_end = Some(INITRD_END);
            new_dram.splice(
                initrd_offset..initrd_offset + mapped_initrd.len(),
                mapped_initrd.iter().cloned(),
            );
        }

        let dram_size = dbg!(new_dram.len());
        dbg!(&new_dram[0x200000..0x200200]);
        dbg!(&new_dram[0x0719_2600..0x0719_2800]);
        Dram {
            dram: new_dram,
            base_addr: virt_entry,
            size: dram_size,
            initrd_start,
            initrd_end,
        }
    }
}

#[allow(clippy::identity_op)]
impl Device for Dram {
    // is addr in device address space
    fn in_range(&self, addr: u64) -> bool {
        (self.base_addr..=self.base_addr + self.size as u64).contains(&addr)
    }

    // address to raw index
    fn addr2index(&self, addr: u64) -> usize {
        (addr - self.base_addr) as usize
    }

    // store
    fn store8(&mut self, addr: u64, data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        let index = self.addr2index(addr);
        self.dram[index] = (data & 0xFF) as u8;
        Ok(())
    }

    fn store16(&mut self, addr: u64, data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        let index = self.addr2index(addr);
        self.dram[index + 1] = ((data >> 8) & 0xFF) as u8;
        self.dram[index + 0] = ((data >> 0) & 0xFF) as u8;
        Ok(())
    }

    fn store32(&mut self, addr: u64, data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        let index = self.addr2index(addr);
        self.dram[index + 3] = ((data >> 24) & 0xFF) as u8;
        self.dram[index + 2] = ((data >> 16) & 0xFF) as u8;
        self.dram[index + 1] = ((data >> 8) & 0xFF) as u8;
        self.dram[index + 0] = ((data >> 0) & 0xFF) as u8;
        Ok(())
    }

    fn store64(&mut self, addr: u64, data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        let index = self.addr2index(addr);
        self.dram[index + 7] = ((data >> 56) & 0xFF) as u8;
        self.dram[index + 6] = ((data >> 48) & 0xFF) as u8;
        self.dram[index + 5] = ((data >> 40) & 0xFF) as u8;
        self.dram[index + 4] = ((data >> 32) & 0xFF) as u8;
        self.dram[index + 3] = ((data >> 24) & 0xFF) as u8;
        self.dram[index + 2] = ((data >> 16) & 0xFF) as u8;
        self.dram[index + 1] = ((data >> 8) & 0xFF) as u8;
        self.dram[index + 0] = ((data >> 0) & 0xFF) as u8;
        Ok(())
    }

    // load
    fn load8(&mut self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        let index = self.addr2index(addr);
        Ok(self.dram[index] as i8 as i64 as u64)
    }

    fn load16(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        let index = self.addr2index(addr);
        Ok(((self.dram[index + 1] as i16) << 8 | (self.dram[index + 0] as i16)) as i64 as u64)
    }

    fn load32(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        let index = self.addr2index(addr);
        Ok(((self.dram[index + 3] as i32) << 24
            | (self.dram[index + 2] as i32) << 16
            | (self.dram[index + 1] as i32) << 8
            | (self.dram[index + 0] as i32)) as i64 as u64)
    }

    fn load64(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        let index = self.addr2index(addr);
        Ok((self.dram[index + 7] as u64) << 56
            | (self.dram[index + 6] as u64) << 48
            | (self.dram[index + 5] as u64) << 40
            | (self.dram[index + 4] as u64) << 32
            | (self.dram[index + 3] as u64) << 24
            | (self.dram[index + 2] as u64) << 16
            | (self.dram[index + 1] as u64) << 8
            | (self.dram[index + 0] as u64))
    }

    fn load_u8(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        let index = self.addr2index(addr);
        Ok(self.dram[index] as u64)
    }

    fn load_u16(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        let index = self.addr2index(addr);
        Ok(((self.dram[index + 1] as u16) << 8 | (self.dram[index + 0] as u16)) as u64)
    }

    fn load_u32(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        let index = self.addr2index(addr);
        Ok(((self.dram[index + 3] as u32) << 24
            | (self.dram[index + 2] as u32) << 16
            | (self.dram[index + 1] as u32) << 8
            | (self.dram[index + 0] as u32)) as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const DRAM_SIZE: usize = 1024 * 1024 * 128; // 2^27

    #[test]
    fn load_store_u8_test() {
        let dram = &mut Dram {
            dram: vec![0; DRAM_SIZE],
            base_addr: 0,
            size: DRAM_SIZE,
        };
        let mut addr = 0;
        let mut test_8 = |data: i32| {
            Dram::store8(dram, addr, data as u64).unwrap();
            assert_eq!(data as u64, Dram::load_u8(dram, addr).unwrap());
            addr += 2;
        };

        test_8(0);
        test_8(17);
        test_8(0b01111111);

        let minus_num = -42;
        Dram::store8(dram, addr, minus_num as u64).unwrap();
        assert_ne!(minus_num, Dram::load_u8(dram, addr).unwrap() as i32);
        Dram::store16(dram, addr, minus_num as u64).unwrap();
        assert_eq!(214, Dram::load_u8(dram, addr).unwrap());
    }

    #[test]
    fn load_store_8_test() {
        let dram = &mut Dram {
            dram: vec![0; DRAM_SIZE],
            base_addr: 0,
            size: DRAM_SIZE,
        };
        let mut addr = 0;
        let mut test_8 = |data: i32| {
            Dram::store8(dram, addr, data as u64).unwrap();
            assert_eq!(data as u64, Dram::load8(dram, addr).unwrap());
            addr += 2;
        };

        test_8(0);
        test_8(17);
        test_8(0b01111111);
        test_8(-42);

        Dram::store8(dram, addr, 0b10000000).unwrap();
        assert_ne!(0b10000000, Dram::load8(dram, addr).unwrap());
    }

    #[test]
    fn load_store_16_test() {
        let dram = &mut Dram {
            dram: vec![0; DRAM_SIZE],
            base_addr: 0,
            size: DRAM_SIZE,
        };
        let mut addr = 0;
        let mut test_16 = |data: i32| {
            Dram::store16(dram, addr, data as u64).unwrap();
            assert_eq!(data as u64, Dram::load16(dram, addr).unwrap());
            addr += 2;
        };

        test_16(0);
        test_16(157);
        test_16(255);
        test_16(-42);
        test_16(0b0111111111111111);

        Dram::store16(dram, addr, 0b1000000010000000).unwrap();
        assert_ne!(0b1000000010000000, Dram::load16(dram, addr).unwrap());
    }

    #[test]
    fn load_store_u16_test() {
        let dram = &mut Dram {
            dram: vec![0; DRAM_SIZE],
            base_addr: 0,
            size: DRAM_SIZE,
        };
        let mut addr = 0;
        let mut test_u16 = |data: i32| {
            Dram::store16(dram, addr, data as u64).unwrap();
            assert_eq!(data as u64, Dram::load_u16(dram, addr).unwrap());
            addr += 2;
        };

        test_u16(0);
        test_u16(157);
        test_u16(255);
        test_u16(0b0111111111111111);

        let minus_num = -42;
        Dram::store16(dram, addr, minus_num as u64).unwrap();
        assert_ne!(minus_num, Dram::load_u16(dram, addr).unwrap() as i32);
        Dram::store16(dram, addr, minus_num as u64).unwrap();
        assert_eq!(65494, Dram::load_u16(dram, addr).unwrap());
    }

    #[test]
    #[allow(overflowing_literals)]
    fn load_store_32_test() {
        let dram = &mut Dram {
            dram: vec![0; DRAM_SIZE],
            base_addr: 0,
            size: DRAM_SIZE,
        };
        let mut addr = 0;
        let mut test_32 = |data: i32| {
            Dram::store32(dram, addr, data as u64).unwrap();
            assert_eq!(data as u64, Dram::load32(dram, addr).unwrap());
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
