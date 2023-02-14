mod dtb;
mod dts;

use super::mrom::Mrom;
use crate::Isa;

impl Mrom {
    pub fn load_dtb(&mut self, dram_addr: u64, isa: Isa) {
        let dts: String = dts::make_dts(dram_addr, isa).replace("  ", "");
        let dtb: Vec<u8> = dtb::make_dtb(dts);
        self.mrom.extend(dtb);
        self.set_size();
    }
}
