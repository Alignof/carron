use crate::elfload;

pub enum AddrTransMode {
    Bare,
    Sv32,
}

pub struct MMU {
    state: AddrTransMode,
}

impl MMU {
    pub fn new(loader: elfload::ElfLoader) -> MMU {
        MMU {
            state: AddrTransMode::Bare,
        }
    }

    fn trans_addr(&self, addr: usize) -> usize {
        match self.state {
            AddrTransMode::Bare => addr,
            AddrTransMode::Sv32 => {
                addr
            },
        }
    }
}
