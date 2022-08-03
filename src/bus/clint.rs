use super::Device;
use crate::TrapCause;

pub struct Clint {
    pub clint: Vec<u8>,
    pub base_addr: u32,
        size: usize,
}

impl Clint {
    #[allow(arithmetic_overflow)]
    pub fn new() -> Clint {
        const CLINT_SIZE: usize = 0xFFFF;

        Clint {
            clint: vec![0; CLINT_SIZE],
            base_addr: 0x0200_0000,
            size: CLINT_SIZE
        }
    }
}

#[allow(clippy::identity_op)]
impl Device for Clint {
    // address to raw index
    fn addr2index(&self, addr: u32, cause: TrapCause) -> Result<usize, (Option<i32>, TrapCause, String)> {
        if self.base_addr <= addr && addr <= self.base_addr + self.size as u32 {
            Ok((addr - self.base_addr) as usize)
        } else {
            Err((
                Some(addr as i32),
                cause,
                format!("addr is out of mrom address space 0x{:x}/0x{:x}", addr, self.base_addr + self.size as u32)
            ))
        }
    }

    // get 1 byte
    fn raw_byte(&self, addr: u32) -> u8 {
        let addr = self.addr2index(addr, TrapCause::InstPageFault).unwrap();
        self.clint[addr]
    }

    // store
    fn store8(&mut self, addr: u32, _data: i32) -> Result<(), (Option<i32>, TrapCause, String)> {
        Err((
            Some(addr as i32),
            TrapCause::StoreAMOPageFault,
            format!("clint is allow load/store32 but try store8")
        ))
    }

    fn store16(&mut self, addr: u32, _data: i32) -> Result<(), (Option<i32>, TrapCause, String)> {
        Err((
            Some(addr as i32),
            TrapCause::StoreAMOPageFault,
            format!("clint is allow load/store32 but try store16")
        ))
    }

    fn store32(&mut self, addr: u32, data: i32) -> Result<(), (Option<i32>, TrapCause, String)> {
        let addr = self.addr2index(addr, TrapCause::StoreAMOPageFault)?;
        self.clint[addr + 3] = ((data >> 24) & 0xFF) as u8;
        self.clint[addr + 2] = ((data >> 16) & 0xFF) as u8;
        self.clint[addr + 1] = ((data >>  8) & 0xFF) as u8;
        self.clint[addr + 0] = ((data >>  0) & 0xFF) as u8;
        Ok(())
    }


    // load
    fn load8(&self, addr: u32) -> Result<i32, (Option<i32>, TrapCause, String)> {
        Err((
            Some(addr as i32),
            TrapCause::LoadPageFault,
            format!("clint is allow load/store32 but try load8")
        ))
    }

    fn load16(&self, addr: u32) -> Result<i32, (Option<i32>, TrapCause, String)> {
        Err((
            Some(addr as i32),
            TrapCause::LoadPageFault,
            format!("clint is allow load/store32 but try load16")
        ))
    }

    fn load32(&self, addr: u32) -> Result<i32, (Option<i32>, TrapCause, String)> {
        let addr = self.addr2index(addr, TrapCause::LoadPageFault)?;
        Ok((
         (self.clint[addr + 3] as u32) << 24 |
         (self.clint[addr + 2] as u32) << 16 |
         (self.clint[addr + 1] as u32) <<  8 |
         (self.clint[addr + 0] as u32)
        ) as i32)
    }

    fn load_u8(&self, addr: u32) -> Result<i32, (Option<i32>, TrapCause, String)> {
        Err((
            Some(addr as i32),
            TrapCause::LoadPageFault,
            format!("clint is allow load/store32 but try load_u8")
        ))
    }

    fn load_u16(&self, addr: u32) -> Result<i32, (Option<i32>, TrapCause, String)> {
        Err((
            Some(addr as i32),
            TrapCause::LoadPageFault,
            format!("clint is allow load/store32 but try load_u16")
        ))
    }
}
