use super::Device;
use crate::TrapCause;

pub struct Plic {
    pub plic: Vec<u8>,
    pub base_addr: u64,
    size: usize,
}

impl Default for Plic {
    fn default() -> Self {
        Self::new()
    }
}

impl Plic {
    #[allow(arithmetic_overflow)]
    pub fn new() -> Self {
        const PLIC_SIZE: usize = 0x0100_0000;

        Plic {
            plic: vec![0; PLIC_SIZE],
            base_addr: 0x0c00_0000,
            size: PLIC_SIZE,
        }
    }
}

#[allow(clippy::identity_op)]
impl Device for Plic {
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
            "plic only allows load/store32 but try store8".to_string(),
        ))
    }

    fn store16(&mut self, addr: u64, _data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::StoreAMOPageFault,
            "plic only allows load/store32 but try store16".to_string(),
        ))
    }

    fn store32(&mut self, addr: u64, data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        self.plic[addr + 3] = ((data >> 24) & 0xFF) as u8;
        self.plic[addr + 2] = ((data >> 16) & 0xFF) as u8;
        self.plic[addr + 1] = ((data >> 8) & 0xFF) as u8;
        self.plic[addr + 0] = ((data >> 0) & 0xFF) as u8;
        Ok(())
    }

    fn store64(&mut self, addr: u64, data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "plic only allows load/store32 but try store64".to_string(),
        ))
    }

    // load
    fn load8(&mut self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "plic only allows load/store32 but try load8".to_string(),
        ))
    }

    fn load16(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "plic only allows load/store32 but try load16".to_string(),
        ))
    }

    fn load32(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        Ok(((self.plic[addr + 3] as i32) << 24
            | (self.plic[addr + 2] as i32) << 16
            | (self.plic[addr + 1] as i32) << 8
            | (self.plic[addr + 0] as i32)) as i64 as u64)
    }

    fn load64(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "plic only allows load/store32 but try load64".to_string(),
        ))
    }
    fn load_u8(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "plic only allows load/store32 but try load_u8".to_string(),
        ))
    }

    fn load_u16(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "plic only allows load/store32 but try load_u16".to_string(),
        ))
    }

    fn load_u32(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "plic only allows load/store32 but try load_u32".to_string(),
        ))
    }
}
