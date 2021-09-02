use super::CPU;

#[allow(non_camel_case_types)]
pub enum CSRname {
    mstatus = 0x300,
    mepc = 0x341, 
    mcause = 0x342,
    mtval = 0x343,
}

impl CSRname {
    pub fn wrap(self) -> Option<usize> {
        Some(self as usize)
    }
}

impl CPU {
    pub fn read_csr(&self, src: Option<usize>) -> u32 {
        self.csrs[src.unwrap()]
    }

    pub fn write_csr(&mut self, dist: Option<usize>, src: i32) {
        self.csrs[dist.unwrap()] = src as u32;
    }

    pub fn bitset_csr(&mut self, dist: Option<usize>, src: i32) {
        let mask = src as u32;
        if mask != 0 {
            self.csrs[dist.unwrap()] |= mask;
        }
    }

    pub fn bitclr_csr(&mut self, dist: Option<usize>, src: i32) {
        let mask = src as u32;
        if mask != 0 {
            self.csrs[dist.unwrap()] &= !mask;
        }
    }
}
