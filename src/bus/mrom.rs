use super::Device;
use crate::{Isa, TrapCause};

pub struct Mrom {
    pub mrom: Vec<u8>,
    pub base_addr: u64,
    size: usize,
}

impl Mrom {
    #[allow(arithmetic_overflow)]
    pub fn new(entry_point: u64, isa: Isa) -> Self {
        let entry_upper = (entry_point >> 32) as u32;
        let entry_lower = (entry_point & 0xffffffff) as u32;
        let reset_vector: Vec<u32> = vec![
            0x00000297, // auipc   t0, 0x0
            0x02028593, // addi    a1, t0, 32
            0xf1402573, // csrr    a0, mhartid
            match isa {
                Isa::Rv32 => 0x0182a283, // lw      t0, 24(t0)
                Isa::Rv64 => 0x0182b283, // ld      t0, 24(t0)
            }, //                            |
            0x00028067, // jr      t0        |
            0x0,        //                   |
            entry_lower, // <----------------+
            entry_upper >> 32,
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
            size: mrom_size,
        }
    }

    pub fn set_size(&mut self) {
        self.size = self.mrom.len();
    }
}

#[allow(clippy::identity_op)]
impl Device for Mrom {
    // is addr in device address space
    fn in_range(&self, addr: u64) -> bool {
        (self.base_addr..=self.base_addr + self.size as u64).contains(&addr)
    }

    // address to raw index
    fn addr2index(&self, addr: u64) -> usize {
        (addr - self.base_addr) as usize
    }

    // get 1 byte
    fn raw_byte(&self, addr: u64) -> u8 {
        let addr = self.addr2index(addr);
        self.mrom[addr]
    }

    // store
    fn store8(&mut self, addr: u64, _data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::StoreAMOPageFault,
            format!("mrom is read only: {:x}", addr),
        ))
    }

    fn store16(&mut self, addr: u64, _data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::StoreAMOPageFault,
            format!("mrom is read only: {:x}", addr),
        ))
    }

    fn store32(&mut self, addr: u64, _data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::StoreAMOPageFault,
            format!("mrom is read only: {:x}", addr),
        ))
    }

    fn store64(&mut self, addr: u64, _data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::StoreAMOPageFault,
            format!("mrom is read only: {:x}", addr),
        ))
    }

    // load
    fn load8(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        Ok(self.mrom[addr] as i8 as i32 as u64)
    }

    fn load16(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        Ok(((self.mrom[addr + 1] as i16) << 8 | (self.mrom[addr + 0] as i16)) as i32 as u64)
    }

    fn load32(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        Ok(((self.mrom[addr + 3] as i32) << 24
            | (self.mrom[addr + 2] as i32) << 16
            | (self.mrom[addr + 1] as i32) << 8
            | (self.mrom[addr + 0] as i32)) as i64 as u64)
    }

    fn load64(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        Ok((self.mrom[addr + 7] as u64) << 56
            | (self.mrom[addr + 6] as u64) << 48
            | (self.mrom[addr + 5] as u64) << 40
            | (self.mrom[addr + 4] as u64) << 32
            | (self.mrom[addr + 3] as u64) << 24
            | (self.mrom[addr + 2] as u64) << 16
            | (self.mrom[addr + 1] as u64) << 8
            | (self.mrom[addr + 0] as u64))
    }
    fn load_u8(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        Ok(self.mrom[addr] as u64)
    }

    fn load_u16(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        Ok((self.mrom[addr + 1] as u64) << 8 | (self.mrom[addr + 0] as u64))
    }
}
