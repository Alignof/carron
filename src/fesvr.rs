use crate::Emulator;

impl Emulator {
    pub fn check_tohost(&self, tohost_addr: u32) -> bool {
        self.cpu.bus.load32(tohost_addr).expect("load tohost addr failed") != 0
    }
}
