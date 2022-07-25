use super::Device;
use crate::TrapCause;

pub struct Mrom {
    pub mrom: Vec<u8>,
    pub base_addr: u32,
        size: usize,
}

impl Mrom {
    #[allow(arithmetic_overflow)]
    pub fn new(entry_point: u32) -> Mrom {
        let reset_vector: Vec<u32> = vec![
            0x00000297, // auipc   t0, 0x0
            0x02028593, // addi    a1, t0, 32
            0xf1402573, // csrr    a0, mhartid
            0x0182a283, // lw      t0, 24(t0)
            0x00028067, // jr      t0    |
            0x0,        //               |
            entry_point,// <-------------+
            entry_point >> 32,
        ];

        // Vec<u32> -> Vec<u8>
        let mrom: Vec<u8> = reset_vector 
            .iter()
            .flat_map(|val| val.to_le_bytes().to_vec())
            .collect();

        let mrom_size = mrom.len();
        Mrom {
            mrom,
            base_addr: 0x1000,
            size: mrom_size
        }
    }
}

impl Device for Mrom {
    // address to raw index
    fn addr2index(&self, addr: u32) -> usize {
        if addr < self.base_addr {
            panic!("invalid address for mrom: {}", addr);
        }

        (addr - self.base_addr) as usize
    }

    // get 1 byte
    fn raw_byte(&self, addr: u32) -> u8 {
        let addr = self.addr2index(addr);
        self.mrom[addr]
    }

    // store
    fn store8(&mut self, addr: u32, _data: i32) -> Result<(), (Option<i32>, TrapCause, String)> {
        Err((
            Some(addr as i32),
            TrapCause::StoreAMOPageFault,
            "mrom is read only.".to_string()
        ))
    }

    fn store16(&mut self, addr: u32, _data: i32) -> Result<(), (Option<i32>, TrapCause, String)> {
        Err((
            Some(addr as i32),
            TrapCause::StoreAMOPageFault,
            "mrom is read only.".to_string()
        ))
    }

    fn store32(&mut self, addr: u32, _data: i32) -> Result<(), (Option<i32>, TrapCause, String)> {
        Err((
            Some(addr as i32),
            TrapCause::StoreAMOPageFault,
            "mrom is read only.".to_string()
        ))
    }


    // load
    fn load8(&self, addr: u32) -> Result<i32, (Option<i32>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        Ok(self.mrom[addr] as i8 as i32)
    }

    fn load16(&self, addr: u32) -> Result<i32, (Option<i32>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        Ok(((self.mrom[addr + 1] as u16) << 8 |
         (self.mrom[addr + 0] as u16)) as i16 as i32)
    }

    fn load32(&self, addr: u32) -> Result<i32, (Option<i32>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        Ok(((self.mrom[addr + 3] as u32) << 24 |
         (self.mrom[addr + 2] as u32) << 16 |
         (self.mrom[addr + 1] as u32) <<  8 |
         (self.mrom[addr + 0] as u32)) as i32)
    }

    fn load_u8(&self, addr: u32) -> Result<i32, (Option<i32>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        Ok(self.mrom[addr] as i32)
    }

    fn load_u16(&self, addr: u32) -> Result<i32, (Option<i32>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        Ok(((self.mrom[addr + 1] as u32) << 8 |
         (self.mrom[addr + 0] as u32)) as i32)
    }
}
