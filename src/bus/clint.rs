use super::Device;
use crate::TrapCause;

pub struct Clint {
    pub clint: Vec<u8>,
    pub base_addr: u64,
    size: usize,
}

impl Default for Clint {
    fn default() -> Self {
        Self::new()
    }
}

impl Clint {
    #[allow(arithmetic_overflow)]
    pub fn new() -> Self {
        const CLINT_SIZE: usize = 0xFFFF;

        Clint {
            clint: vec![0; CLINT_SIZE],
            base_addr: 0x0200_0000,
            size: CLINT_SIZE,
        }
    }
}

#[allow(clippy::identity_op)]
impl Device for Clint {
    // is addr in device address space
    fn in_range(&self, addr: u64) -> bool {
        (self.base_addr..=self.base_addr + self.size as u64).contains(&addr)
    }

    // address to raw index
    fn addr2index(&self, addr: u64) -> usize {
        (addr - self.base_addr) as usize
    }

    // store
    fn store8(&mut self, addr: u64, _data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::StoreAMOPageFault,
            "clint is allow load/store32 but try store8".to_string(),
        ))
    }

    fn store16(&mut self, addr: u64, _data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::StoreAMOPageFault,
            "clint is allow load/store32 but try store16".to_string(),
        ))
    }

    fn store32(&mut self, addr: u64, data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        self.clint[addr + 3] = ((data >> 24) & 0xFF) as u8;
        self.clint[addr + 2] = ((data >> 16) & 0xFF) as u8;
        self.clint[addr + 1] = ((data >> 8) & 0xFF) as u8;
        self.clint[addr + 0] = ((data >> 0) & 0xFF) as u8;
        Ok(())
    }

    fn store64(&mut self, addr: u64, data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        self.clint[addr + 7] = ((data >> 56) & 0xFF) as u8;
        self.clint[addr + 6] = ((data >> 48) & 0xFF) as u8;
        self.clint[addr + 5] = ((data >> 40) & 0xFF) as u8;
        self.clint[addr + 4] = ((data >> 32) & 0xFF) as u8;
        self.clint[addr + 3] = ((data >> 24) & 0xFF) as u8;
        self.clint[addr + 2] = ((data >> 16) & 0xFF) as u8;
        self.clint[addr + 1] = ((data >> 8) & 0xFF) as u8;
        self.clint[addr + 0] = ((data >> 0) & 0xFF) as u8;
        Ok(())
    }

    // load
    fn load8(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "clint is allow load/store32 but try load8".to_string(),
        ))
    }

    fn load16(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "clint is allow load/store32 but try load16".to_string(),
        ))
    }

    fn load32(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        Ok(((self.clint[addr + 3] as i32) << 24
            | (self.clint[addr + 2] as i32) << 16
            | (self.clint[addr + 1] as i32) << 8
            | (self.clint[addr + 0] as i32)) as i64 as u64)
    }

    fn load64(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        Ok((self.clint[addr + 7] as u64) << 56
            | (self.clint[addr + 6] as u64) << 48
            | (self.clint[addr + 5] as u64) << 40
            | (self.clint[addr + 4] as u64) << 32
            | (self.clint[addr + 3] as u64) << 24
            | (self.clint[addr + 2] as u64) << 16
            | (self.clint[addr + 1] as u64) << 8
            | (self.clint[addr + 0] as u64))
    }
    fn load_u8(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "clint is allow load/store32 but try load_u8".to_string(),
        ))
    }

    fn load_u16(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "clint is allow load/store32 but try load_u16".to_string(),
        ))
    }

    fn load_u32(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "clint is allow load/store32 but try load_u32".to_string(),
        ))
    }
}
