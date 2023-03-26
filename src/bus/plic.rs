use super::Device;
use crate::cpu::PrivilegedLevel;
use crate::TrapCause;

const PRIORITY_BASE: usize = 0x0;
const ENABLE_BASE: usize = 0x2000;
const ENABLE_PER_HART: usize = 0x80;
const CONTEXT_BASE: usize = 0x20_0000;
const CONTEXT_PER_HART: usize = 0x1000;
const CONTEXT_THRESHOLD: usize = 0x0;
const CONTEXT_CLAIM: usize = 0x4;

const PLIC_SIZE: usize = 0x100_0000;
const PLIC_MAX_DEVICES: usize = 1024;
const NDEV: usize = 0x1f;
const NUM_IDS: usize = NDEV + 1;
const NUM_IDS_WORD: usize = ((NDEV + 1) + (32 - 1)) / 32;
const CONTEXT_NUM: usize = 2;

pub struct PlicContext {
    priority_thresould: u8,
    enable: Vec<u32>,
    pending: Vec<u32>,
    pending_priority: Vec<u8>,
    claimed: Vec<u32>,
    context_priv: PrivilegedLevel,
}

impl PlicContext {
    fn new(context_priv: PrivilegedLevel) -> Self {
        PlicContext {
            priority_thresould: 0,
            enable: vec![0; PLIC_MAX_DEVICES / 32],
            pending: vec![0; PLIC_MAX_DEVICES / 32],
            pending_priority: vec![0; PLIC_MAX_DEVICES],
            claimed: vec![0; PLIC_MAX_DEVICES / 32],
            context_priv,
        }
    }
}

pub struct Plic {
    priority: Vec<u8>,
    level: Vec<u32>,
    contexts: Vec<PlicContext>,
    pub mip_mask: u64,
    pub mip_value: u64,
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
            contexts: vec![
                PlicContext::new(PrivilegedLevel::Machine),
                PlicContext::new(PrivilegedLevel::Supervisor),
            ],
            mip_mask: 0,
            mip_value: 0,
            base_addr: 0x0c00_0000,
            size: PLIC_SIZE,
        }
    }

    fn context_best_pending(&self, context_id: usize) -> u32 {
        let mut best_id_prio = 0;
        let mut best_id = 0;
        for i in 0..NUM_IDS_WORD {
            if self.contexts[context_id].pending[i] == 0 {
                continue;
            }

            for off in 0..32 {
                let id = i * 32 + off;
                if (NUM_IDS <= id)
                    || ((self.contexts[context_id].pending[i] & (1 << off)) == 0)
                    || ((self.contexts[context_id].claimed[i] & (1 << off)) != 0)
                {
                    continue;
                }

                if (best_id == 0) || (best_id_prio < self.contexts[context_id].pending_priority[id])
                {
                    best_id = id as u32;
                    best_id_prio = self.contexts[context_id].pending_priority[id];
                }
            }
        }

        best_id
    }

    fn context_update(&mut self, context_id: usize) {
        const MIP_MEIP_MASK: u64 = 1 << 11;
        const MIP_SEIP_MASK: u64 = 1 << 9;
        let best_id = self.context_best_pending(context_id);
        self.mip_mask = match self.contexts[context_id].context_priv {
            PrivilegedLevel::Machine => MIP_MEIP_MASK,
            PrivilegedLevel::Supervisor => MIP_SEIP_MASK,
            _ => unreachable!(),
        };
        self.mip_value = if best_id == 0 {
            0
        } else {
            match self.contexts[context_id].context_priv {
                PrivilegedLevel::Machine => MIP_MEIP_MASK,
                PrivilegedLevel::Supervisor => MIP_SEIP_MASK,
                _ => unreachable!(),
            }
        };
    }

    fn context_claim(&mut self, context_id: usize) -> u32 {
        let best_id = self.context_best_pending(context_id);
        let best_id_word = (best_id / 32) as usize;
        let best_id_mask = 1 << (best_id % 32);

        if best_id != 0 {
            self.contexts[context_id].claimed[best_id_word] |= best_id_mask;
        }

        self.context_update(context_id);
        best_id
    }

    fn priority_read(&self, offset: usize) -> u32 {
        let id = offset >> 2;
        if id > 0 && id < NUM_IDS {
            self.priority[id] as u32
        } else {
            0
        }
    }

    fn priority_write(&mut self, offset: usize, val: u32) {
        const PLIC_PRIO_MASK: u32 = 0b1111;
        let id = offset >> 2;
        if id > 0 && id < NUM_IDS {
            self.priority[id] = (val & PLIC_PRIO_MASK) as u8;
        }
    }

    fn context_enable_read(&self, context_id: usize, offset: usize) -> u32 {
        let id_word = offset >> 2;
        if id_word < NUM_IDS_WORD {
            self.contexts[context_id].enable[id_word]
        } else {
            0
        }
    }

    fn context_enable_write(&mut self, context_id: usize, offset: usize, val: u32) {
        let id_word = offset >> 2;
        if id_word >= NUM_IDS_WORD {
            return;
        }

        let old_val = self.contexts[context_id].enable[id_word];
        let new_val = if id_word == 0 { val & 0xffff_fffe } else { val };
        let xor_val = old_val ^ new_val;

        self.contexts[context_id].enable[id_word] = new_val;

        for i in 0..32 {
            let id = id_word * 32 + i;
            let id_mask = 1 << i;
            let id_prio = self.priority[id];

            if xor_val & id_mask == 0 {
                continue;
            }

            if new_val & id_mask != 0 && self.level[id_word] & id_mask != 0 {
                self.contexts[context_id].pending[id_word] |= id_mask;
                self.contexts[context_id].pending_priority[id] = id_prio;
            } else if new_val & id_mask == 0 {
                self.contexts[context_id].pending[id_word] &= !id_mask;
                self.contexts[context_id].pending_priority[id] = 0;
                self.contexts[context_id].claimed[id_word] &= !id_mask;
            }
        }

        self.context_update(context_id);
    }

    fn context_read(&mut self, context_id: usize, offset: usize) -> u32 {
        match offset {
            CONTEXT_THRESHOLD => self.contexts[context_id].priority_thresould as u32,
            CONTEXT_CLAIM => self.context_claim(context_id),
            _ => unreachable!(),
        }
    }

    fn context_write(&mut self, context_id: usize, offset: usize, val: u32) {
        match offset {
            CONTEXT_THRESHOLD => {
                const PLIC_PRIO_MASK: u32 = 0b1111;
                let val = val & PLIC_PRIO_MASK;
                if val <= PLIC_PRIO_MASK {
                    self.contexts[context_id].priority_thresould = val as u8;
                    self.context_update(context_id);
                }
            }
            CONTEXT_CLAIM => {
                let id_word = (val / 32) as usize;
                let id_mask = 1 << (val % 32);
                if val < NUM_IDS as u32 && self.contexts[context_id].enable[id_word] & id_mask != 0
                {
                    self.contexts[context_id].claimed[id_word] &= !id_mask;
                    self.context_update(context_id);
                }
            }
            _ => unreachable!("offset: {:#x}", offset),
        }
    }

    pub fn set_interrupt_level(&mut self, id: u32, level: u32) {
        if id == 0 || NUM_IDS as u32 <= id {
            return;
        }

        let id_prio = self.priority[id as usize];
        let id_word = (id / 32) as usize;
        let id_mask = 1 << (id % 32);

        if level != 0 {
            self.level[id_word] |= id_mask;
        } else {
            self.level[id_word] &= !id_mask;
        }

        for c in 0..CONTEXT_NUM {
            if self.contexts[c].enable[id_word] & id_mask != 0 {
                if level != 0 {
                    self.contexts[c].pending[id_word] |= id_mask;
                    self.contexts[c].pending_priority[id as usize] = id_prio;
                } else {
                    self.contexts[c].pending[id_word] &= !id_mask;
                    self.contexts[c].pending_priority[id as usize] = 0;
                    self.contexts[c].claimed[id_word] &= !id_mask;
                }

                self.context_update(c);
                break;
            }
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
            TrapCause::StoreAMOAccessFault,
            "plic only allows load/store32,64 but try store8".to_string(),
        ))
    }

    fn store16(&mut self, addr: u64, _data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::StoreAMOAccessFault,
            "plic only allows load/store32,64 but try store16".to_string(),
        ))
    }

    fn store32(&mut self, addr: u64, data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        const ENABLE_BASE_MINUS_ONE: usize = ENABLE_BASE - 1;
        const CONTEXT_BASE_MINUS_ONE: usize = CONTEXT_BASE - 1;
        const PLIC_SIZE_MINUS_ONE: usize = PLIC_SIZE - 1;
        let addr = self.addr2index(addr);
        match addr {
            PRIORITY_BASE..=ENABLE_BASE_MINUS_ONE => {
                self.priority_write(addr, data as u32);
                Ok(())
            }
            ENABLE_BASE..=CONTEXT_BASE_MINUS_ONE => {
                let cntx = (addr - ENABLE_BASE) / ENABLE_PER_HART;
                let addr = addr - (cntx * ENABLE_PER_HART + ENABLE_BASE);
                if cntx < CONTEXT_NUM {
                    self.context_enable_write(cntx, addr, data as u32);
                    Ok(())
                } else {
                    Err((
                        Some(addr as u64),
                        TrapCause::StoreAMOAccessFault,
                        "plic only allows load/store32,64 but try store16".to_string(),
                    ))
                }
            }
            CONTEXT_BASE..=PLIC_SIZE_MINUS_ONE => {
                let cntx = (addr - CONTEXT_BASE) / CONTEXT_PER_HART;
                let addr = addr - (cntx * CONTEXT_PER_HART + CONTEXT_BASE);
                if cntx < CONTEXT_NUM {
                    self.context_write(cntx, addr, data as u32);
                    Ok(())
                } else {
                    Err((
                        Some(addr as u64),
                        TrapCause::StoreAMOAccessFault,
                        "plic only allows load/store32,64 but try store16".to_string(),
                    ))
                }
            }
            _ => unreachable!(),
        }
    }

    fn store64(&mut self, addr: u64, data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        self.store32(addr, data & 0xffff)?;
        self.store32(addr + 4, data >> 32 & 0xffff)
    }

    // load
    fn load8(&mut self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadAccessFault,
            "plic only allows load/store32,64 but try load8".to_string(),
        ))
    }

    fn load16(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadAccessFault,
            "plic only allows load/store32,64 but try load16".to_string(),
        ))
    }

    fn load32(&mut self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        const ENABLE_BASE_MINUS_ONE: usize = ENABLE_BASE - 1;
        const CONTEXT_BASE_MINUS_ONE: usize = CONTEXT_BASE - 1;
        const PLIC_SIZE_MINUS_ONE: usize = PLIC_SIZE - 1;
        let addr = self.addr2index(addr);
        match addr {
            PRIORITY_BASE..=ENABLE_BASE_MINUS_ONE => Ok(self.priority_read(addr) as u64),
            ENABLE_BASE..=CONTEXT_BASE_MINUS_ONE => {
                let cntx = (addr - ENABLE_BASE) / ENABLE_PER_HART;
                let addr = addr - (cntx * ENABLE_PER_HART + ENABLE_BASE);
                if cntx < CONTEXT_NUM {
                    Ok(self.context_enable_read(cntx, addr) as u64)
                } else {
                    Err((
                        Some(addr as u64),
                        TrapCause::LoadAccessFault,
                        "plic only allows load/store32,64 but try store16".to_string(),
                    ))
                }
            }
            CONTEXT_BASE..=PLIC_SIZE_MINUS_ONE => {
                let cntx = (addr - CONTEXT_BASE) / CONTEXT_PER_HART;
                let addr = addr - (cntx * CONTEXT_PER_HART + CONTEXT_BASE);
                if cntx < CONTEXT_NUM {
                    Ok(self.context_read(cntx, addr) as u64)
                } else {
                    Err((
                        Some(addr as u64),
                        TrapCause::LoadAccessFault,
                        "plic only allows load/store32,64 but try store16".to_string(),
                    ))
                }
            }
            _ => unreachable!(),
        }
    }

    fn load64(&mut self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        self.load32(addr)?;
        self.load32(addr + 4)
    }

    fn load_u8(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadAccessFault,
            "plic only allows load/store32,64 but try load_u8".to_string(),
        ))
    }

    fn load_u16(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadAccessFault,
            "plic only allows load/store32,64 but try load_u16".to_string(),
        ))
    }

    fn load_u32(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadAccessFault,
            "plic only allows load/store32,64 but try load_u32".to_string(),
        ))
    }
}
