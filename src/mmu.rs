pub mod bus;
use bus::Bus;
use crate::elfload;

pub enum AddrTransMode {
    Bare,
    Sv32,
}

pub struct MMU {
    state: AddrTransMode,
    bus: bus::Bus,
}

impl MMU {
    pub fn new(loader: elfload::ElfLoader) -> MMU {
        MMU {
            state: AddrTransMode::Bare,
            bus: Bus::new(loader),
        }
    }

    pub fn trans_addr(&self, addr: usize) -> usize {
        match self.state {
            AddrTransMode::Bare => addr,
            AddrTransMode::Sv32 => {
                addr
            },
        }
    }
}
