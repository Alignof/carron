mod dts;
mod dtb;

use super::mrom::Mrom;

impl Mrom {
    pub fn load_dtb(&mut self, dram_addr: u32) {
        let dts: String = dts::make_dts(dram_addr).replace("  ", "");
        let dtb: dtb::dtb_data = dtb::make_dtb(dts);
        let mut dtb_mmap: Vec<u8> = dtb::make_dtb_mmap(dtb);

        self.mrom.append(
            &mut dtb_mmap
        );

        dbg!("dts comilation is done.");
    }
}

