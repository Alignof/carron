pub struct Dram {
    dram: Vec<u8>,
}

impl Dram {
    pub fn new() -> Dram {
        const DRAM_SIZE: u32 = 65536; // 2^16
        let mut new_dram = vec![0; DRAM_SIZE as usize];

        Dram {
            dram: new_dram,
        }
    }
}
