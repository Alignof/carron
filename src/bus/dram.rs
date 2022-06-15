use crate::elfload;
use super::Device;

struct Segment {
    start: u32,
    end: u32,
    data: Vec<u8>,
}

pub struct Dram {
    dram: Vec<Segment>,
}

impl Dram {
    pub fn new(loader: elfload::ElfLoader) -> (u32, Dram) {
        let new_dram: Vec<Segment>;
        let virt_entry = match loader.get_entry_point() {
            Ok(addr) => addr,
            Err(()) => panic!("entry point not found."),
        };

        // load elf memory mapping 
        for segment in loader.prog_headers.iter() {
            let start = segment.p_offset;
            let end = segment.p_offset + segment.p_filesz;

            new_dram.push(
                Segment {
                    start,
                    end,
                    data: loader.mem_data[start as usize .. end as usize].to_vec()
                }
            );
        }

        (virt_entry, // entry address
         Dram {
             dram: new_dram,
         })
    }

    pub fn new_with_pk(loader: elfload::ElfLoader, pk_load: &elfload::ElfLoader) -> (u32, Dram) {
        let new_dram: Vec<Segment>;
        let pk_virt_entry = match pk_load.get_entry_point() {
            Ok(addr) => addr,
            Err(()) => panic!("entry point not found."),
        };


        // load proxy kernel 
        dbg!(pk_load.mem_data.len());
        for segment in pk_load.prog_headers.iter() {
            let start = segment.p_offset;
            let end = segment.p_offset + segment.p_filesz;

            new_dram.push(
                Segment {
                    start,
                    end,
                    data: loader.mem_data[start as usize .. end as usize].to_vec()
                }
            );
        }

        let virt_entry = match loader.get_entry_point() {
            Ok(addr) => addr,
            Err(()) => panic!("entry point not found."),
        };

        // load user program 
        dbg!(loader.mem_data.len());
        for segment in loader.prog_headers.iter() {
            let start = segment.p_offset;
            let end = segment.p_offset + segment.p_filesz;

            new_dram.push(
                Segment {
                    start,
                    end,
                    data: loader.mem_data[start as usize .. end as usize].to_vec()
                }
            );
        }

        (pk_virt_entry, // entry address
         Dram {
             dram: new_dram,
         })
    }
}

impl Device for Dram {
    // set 1 byte
    fn store_byte(&self, addr: u32, data: u8) {
        for seg in self.dram {
            if seg.start <= addr && seg.end <= seg.end {
                seg[addr - seg.start] = data;
                return;
            }
        }
        panic!("invalid address for Dram: {}", addr);
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
        for seg in self.dram {
            if seg.start <= addr && seg.end <= seg.end {
                return seg[addr - seg.start];
            }
        }
        panic!("invalid address for Dram: {}", addr);
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
