mod dts;
mod dtb;

use super::mrom::Mrom;

impl Mrom {
    pub fn load_dtb(&self, dram_addr: u32) {
        let dts: String = dts::make_dts(dram_addr).replace("  ", "");
        let dtb: dtb::dtb_data = dtb::make_dtb(dts);
        dbg!("dts comilation is done.");
    }
}

