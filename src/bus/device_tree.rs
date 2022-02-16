mod dts;
mod dtb;

use super::mrom::Mrom;

impl Mrom {
    pub fn load_dtb(&mut self, dram_addr: u32) {
        let dts: String = dts::make_dts(dram_addr).replace("  ", "");
        let dtb: Vec<u8> = dtb::make_dtb(dts);

        self.mrom.append(
            &mut dtb
        );

        dbg!("dts comilation is done.");
    }
}

