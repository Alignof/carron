pub enum AddrTransMode {
    Bare,
    Sv32,
}

pub struct MMU {
    state: AddrTransMode,
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            state: AddrTransMode::Bare,
        }
    }
}
