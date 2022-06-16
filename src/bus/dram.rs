use crate::elfload;
use super::Device;

struct Segment {
    start: u32,
    end: u32,
    data: Vec<u8>,
}

pub struct Dram {
        dram: Vec<Segment>,
    pub base_addr: u32,
}

impl Dram {
    pub fn new(loader: elfload::ElfLoader) -> (u32, Dram) {
        let mut new_dram: Vec<Segment> = Vec::new();
        let virt_entry = match loader.get_entry_point() {
            Ok(addr) => addr,
            Err(()) => panic!("entry point not found."),
        };

        // load elf memory mapping 
        for segment in loader.prog_headers.iter() {
            if segment.is_loadable() {
                let seg_size = ((segment.p_memsz + (segment.p_align - 1)) / segment.p_align) * segment.p_align;
                let dram_start = segment.p_paddr;
                let dram_end = segment.p_paddr + seg_size;
                let mmap_start = segment.p_offset;
                let mmap_end = segment.p_offset + segment.p_filesz;
                dbg_hex::dbg_hex!(dram_start + seg_size);
                let mut data: Vec<u8> = vec![0; seg_size as usize];
                data.splice(
                    0 .. segment.p_filesz as usize,
                    loader.mem_data[mmap_start as usize .. mmap_end as usize].iter().cloned(),
                );

                new_dram.push(
                    Segment {
                        start: dram_start,
                        end: dram_end,
                        data, 
                    }
                );
            }
        }

        (virt_entry, // entry address
         Dram {
             dram: new_dram,
             base_addr: virt_entry,
         })
    }

    pub fn new_with_pk(loader: elfload::ElfLoader, pk_load: &elfload::ElfLoader) -> (u32, Dram) {
        let mut new_dram: Vec<Segment> = Vec::new();
        let pk_virt_entry = match pk_load.get_entry_point() {
            Ok(addr) => addr,
            Err(()) => panic!("entry point not found."),
        };


        // load proxy kernel 
        dbg!(pk_load.mem_data.len());
        for segment in pk_load.prog_headers.iter() {
            if segment.is_loadable() {
                let dram_start = segment.p_paddr;
                let dram_end = segment.p_paddr + segment.p_filesz;
                let mmap_start = segment.p_offset;
                let mmap_end = segment.p_offset + segment.p_filesz;
                let mut data: Vec<u8> = vec![0; segment.p_memsz as usize];
                data.splice(
                    dram_start as usize .. dram_end as usize,
                    pk_load.mem_data[mmap_start as usize .. mmap_end as usize].iter().cloned(),
                );

                new_dram.push(
                    Segment {
                        start: dram_start,
                        end: dram_end,
                        data, 
                    }
                );
            }
        }

        let virt_entry = match loader.get_entry_point() {
            Ok(addr) => addr,
            Err(()) => panic!("entry point not found."),
        };

        // load user program 
        dbg!(loader.mem_data.len());
        for segment in loader.prog_headers.iter() {
            if segment.is_loadable() {
                let dram_start = segment.p_paddr;
                let dram_end = segment.p_paddr + segment.p_filesz;
                let mmap_start = segment.p_offset;
                let mmap_end = segment.p_offset + segment.p_filesz;
                let mut data: Vec<u8> = vec![0; segment.p_memsz as usize];
                data.splice(
                    dram_start as usize .. dram_end as usize,
                    loader.mem_data[mmap_start as usize .. mmap_end as usize].iter().cloned(),
                );

                new_dram.push(
                    Segment {
                        start: dram_start,
                        end: dram_end,
                        data, 
                    }
                );
            }
        }

        (pk_virt_entry, // entry address
         Dram {
             dram: new_dram,
             base_addr: virt_entry,
         })
    }
}

impl Device for Dram {
    // set 1 byte
    fn store_byte(&mut self, addr: u32, data: u8) {
        for seg in &mut self.dram {
            if seg.start <= addr && addr <= seg.end {
                seg.data[(addr - seg.start) as usize] = data;
                return;
            }
        }

        panic!("invalid address for Dram: 0x{:x}", addr);
    }

    // store
    fn store8(&mut self, addr: u32, data: i32) {
        self.store_byte(addr + 0, ((data >> 0) & 0xFF) as u8);
    }

    fn store16(&mut self, addr: u32, data: i32) {
        self.store_byte(addr + 1, ((data >> 8) & 0xFF) as u8);
        self.store_byte(addr + 0, ((data >> 0) & 0xFF) as u8);
    }

    fn store32(&mut self, addr: u32, data: i32) {
        self.store_byte(addr + 3, ((data >> 24) & 0xFF) as u8);
        self.store_byte(addr + 2, ((data >> 16) & 0xFF) as u8);
        self.store_byte(addr + 1, ((data >>  8) & 0xFF) as u8);
        self.store_byte(addr + 0, ((data >>  0) & 0xFF) as u8);
    }

    // get 1 byte
    fn load_byte(&self, addr: u32) -> u8 {
        for seg in &self.dram {
            if seg.start <= addr && addr <= seg.end {
                return seg.data[(addr - seg.start) as usize];
            }
        }
        panic!("invalid address for Dram: 0x{:x}", addr);
    }

    // load
    fn load8(&self, addr: u32) -> i32 {
        self.load_byte(addr) as i8 as i32
    }

    fn load16(&self, addr: u32) -> i32 {
        ((self.load_byte(addr + 1) as u16) << 8 |
         (self.load_byte(addr + 0) as u16)) as i16 as i32
    }

    fn load32(&self, addr: u32) -> i32 {
        ((self.load_byte(addr + 3) as u32) << 24 |
         (self.load_byte(addr + 2) as u32) << 16 |
         (self.load_byte(addr + 1) as u32) <<  8 |
         (self.load_byte(addr + 0) as u32)) as i32
    }

    fn load_u8(&self, addr: u32) -> i32 {
        self.load_byte(addr) as i32
    }

    fn load_u16(&self, addr: u32) -> i32 {
        ((self.load_byte(addr + 1) as u32) << 8 |
         (self.load_byte(addr + 0) as u32)) as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    const DRAM_SIZE: usize = 1024 * 1024 * 128; // 2^27

    #[test]
    fn load_store_u8_test() {
        let dram = &mut Dram{
            dram: vec![Segment {
                start: 0,
                end: 0,
                data: vec![0; DRAM_SIZE],
            }]
        };
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
        let dram = &mut Dram{
            dram: vec![Segment {
                start: 0,
                end: 0,
                data: vec![0; DRAM_SIZE],
            }]
        };
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
        let dram = &mut Dram{
            dram: vec![Segment {
                start: 0,
                end: 0,
                data: vec![0; DRAM_SIZE],
            }]
        };
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
        let dram = &mut Dram{
            dram: vec![Segment {
                start: 0,
                end: 0,
                data: vec![0; DRAM_SIZE],
            }]
        };
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
        let dram = &mut Dram{
            dram: vec![Segment {
                start: 0,
                end: 0,
                data: vec![0; DRAM_SIZE],
            }]
        };
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
