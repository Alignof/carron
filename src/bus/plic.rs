use super::Device;
use crate::TrapCause;

const PRIORITY_BASE: usize = 0x0;
const PRIORITY_PER_ID: usize = 0x4;
const ENABLE_BASE: usize = 0x2000;
const ENABLE_PER_HART: usize = 0x80;
const CONTEXT_BASE: usize = 0x200000;
const CONTEXT_PER_HART: usize = 0x1000;
const CONTEXT_THRESHOLD: usize = 0x0;
const CONTEXT_CLAIM: usize = 0x0;

const PLIC_SIZE: usize = 0x0100_0000;
const PLIC_MAX_DEVICES: usize = 1024;

pub struct Plic {
    priority: Vec<u8>,
    level: Vec<u32>,
    enable: Vec<u32>,
    pending: Vec<u32>,
    pending_priority: Vec<u8>,
    priority_thresould: u8,
    claimed: Vec<u32>,
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
        Plic {
            priority: vec![0; PLIC_MAX_DEVICES],
            level: vec![0; PLIC_MAX_DEVICES],
            enable: vec![0; PLIC_MAX_DEVICES / 32],
            pending: vec![0; PLIC_MAX_DEVICES / 32],
            pending_priority: vec![0; PLIC_MAX_DEVICES],
            priority_thresould: 0,
            claimed: vec![0; PLIC_MAX_DEVICES / 32],
            base_addr: 0x0c00_0000,
            size: PLIC_SIZE,
        }
    }

    fn context_best_pending(&self) -> u32 {
        const NUM_IDS_WORD: usize = PLIC_MAX_DEVICES / 32;
        let mut best_id_prio = 0;
        let mut best_id = 0;
        for i in 0..NUM_IDS_WORD {
            if self.pending[i] == 0 {
                continue;
            }

            for off in 0..32 {
                let id = i * 32 + off;
                if PLIC_MAX_DEVICES <= id
                    || self.pending[i] & 1 << off == 0
                    || self.claimed[i] & 1 << off != 0
                {
                    continue;
                }

                if best_id == 0 || best_id_prio < self.pending_priority[id] {
                    best_id = id as u32;
                    best_id_prio = self.pending_priority[id];
                }
            }
        }

        best_id
    }

    fn context_update(&self) {
        const MIP_MEIP: u64 = 1 << 11;
        let best_id = self.context_best_pending();
        let mask = MIP_MEIP;
    }

    fn context_claim(&self) {
        let best_id = self.context_best_pending();
        let best_id_word = (best_id / 32) as usize;
        let best_id_mask = 1 << (best_id % 32);

        if best_id != 0 {
            self.claimed[best_id_word] |= best_id_mask;
        }

        self.context_update();
    }

    fn priority_read(&self, offset: usize) -> u32 {
        let index = (offset >> 2) as usize;
        if index > 0 && index < PLIC_SIZE {
            self.priority[index] as u32
        } else {
            0
        }
    }

    fn priority_write(&self, offset: usize, val: u32) {
        const PLIC_PRIO_MASK: u32 = 0b1111;
        let index = (offset >> 2) as usize;
        if index > 0 && index < PLIC_SIZE {
            self.priority[index] = (val & PLIC_PRIO_MASK) as u8;
        }
    }

    fn context_enable_read(&self, offset: usize) -> u32 {
        let id_word = (offset >> 2) as usize;
        if id_word > 0 && id_word < PLIC_SIZE {
            self.priority[id_word] as u32
        } else {
            0
        }
    }

    fn context_enable_write(&self, offset: usize, val: u32) {
        let id_word = (offset >> 2) as usize;
        if id_word >= PLIC_MAX_DEVICES / 32 {
            return;
        }

        let old_val = self.enable[id_word];
        let new_val = if id_word == 0 { val & 0xfffffffe } else { val };
        let xor_val = old_val ^ new_val;

        self.enable[id_word] = new_val;

        for i in 0..32 {
            let id = id_word * 32 + i;
            let id_mask = 1 << i;
            let id_prio = self.priority[id as usize];

            if xor_val & id_mask == 0 {
                continue;
            }

            if new_val & id_mask != 0 && self.level[id_word] & id_mask != 0 {
                self.pending[id_word] |= id_mask;
                self.pending_priority[id] = id_prio;
            } else if new_val & id_mask == 0 {
                self.pending[id_word] &= !id_mask;
                self.pending_priority[id] = 0;
                self.claimed[id_word] &= !id_mask;
            }
        }

        self.context_update();
    }

    fn context_read(&self, offset: usize) -> u32 {
        match offset {
            CONTEXT_THRESHOLD => self.priority_thresould as u32,
            CONTEXT_CLAIM => self.context_claim(),
        }
    }

    fn context_write(&self, offset: usize, val: u32) {
        match offset {
            CONTEXT_THRESHOLD => {
                const PLIC_PRIO_MASK: u32 = 0b1111;
                let val = val & PLIC_PRIO_MASK;
                if val <= PLIC_PRIO_MASK {
                    self.priority_thresould = val as u8;
                    self.context_update();
                }
            }
            CONTEXT_CLAIM => {
                let id_word = (val / 32) as usize;
                let id_mask = 1 << (val % 32);
                if val < PLIC_MAX_DEVICES as u32 && self.enable[id_word] & id_mask != 0 {
                    self.claimed[id_word] &= !id_mask;
                    self.context_update();
                }
            }
        }
    }

    fn set_interrupt_level(&self, id: u32, level: u32) {
        if id <= 0 || PLIC_MAX_DEVICES as u32 <= id {
            return;
        }

        let id = id as usize;
        let id_word = (id / 32) as usize;
        let id_mask = 1 << (id % 32);

        if level != 0 {
            self.level[id_word] |= id_mask;
        } else {
            self.level[id_word] &= !id_mask;
        }

        if self.enable[id_word] & id_mask != 0 {
            if level != 0 {
                self.pending[id_word] |= id_mask;
                self.pending_priority[id] = self.priority[id];
            } else {
                self.pending[id_word] &= !id_mask;
                self.pending_priority[id] = 0;
                self.claimed[id_word] &= !id_mask;
            }

            self.context_update();
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
            "plic only allows load/store32,64 but try store8".to_string(),
        ))
    }

    fn store16(&mut self, addr: u64, _data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::StoreAMOPageFault,
            "plic only allows load/store32,64 but try store16".to_string(),
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
        let addr = self.addr2index(addr);
        self.plic[addr + 7] = ((data >> 56) & 0xFF) as u8;
        self.plic[addr + 6] = ((data >> 48) & 0xFF) as u8;
        self.plic[addr + 5] = ((data >> 40) & 0xFF) as u8;
        self.plic[addr + 4] = ((data >> 32) & 0xFF) as u8;
        self.plic[addr + 3] = ((data >> 24) & 0xFF) as u8;
        self.plic[addr + 2] = ((data >> 16) & 0xFF) as u8;
        self.plic[addr + 1] = ((data >> 8) & 0xFF) as u8;
        self.plic[addr + 0] = ((data >> 0) & 0xFF) as u8;
        Ok(())
    }

    // load
    fn load8(&mut self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "plic only allows load/store32,64 but try load8".to_string(),
        ))
    }

    fn load16(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "plic only allows load/store32,64 but try load16".to_string(),
        ))
    }

    fn load32(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        const ENABLE_BASE_MINUS_ONE: usize = ENABLE_BASE - 1;
        const CONTEXT_BASE_MINUS_ONE: usize = CONTEXT_BASE - 1;
        const PLIC_SIZE_MINUS_ONE: usize = PLIC_SIZE - 1;
        let addr = self.addr2index(addr);
        match addr {
            PRIORITY_BASE..=ENABLE_BASE_MINUS_ONE => Ok(self.priority_read(addr) as u64),
            ENABLE_BASE..=CONTEXT_BASE_MINUS_ONE => Ok(self.context_enable_read(addr) as u64),
            CONTEXT_BASE..=PLIC_SIZE_MINUS_ONE => Ok(self.context_read(addr) as u64),
        }
    }

    fn load64(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        self.load32(addr)?;
        self.load32(addr + 4)
    }

    fn load_u8(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "plic only allows load/store32,64 but try load_u8".to_string(),
        ))
    }

    fn load_u16(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "plic only allows load/store32,64 but try load_u16".to_string(),
        ))
    }

    fn load_u32(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "plic only allows load/store32,64 but try load_u32".to_string(),
        ))
    }
}
