use crate::elfload;
use super::Device;

pub struct Mrom {
        mrom: Vec<u8>,
    pub base_addr: u32,
}

impl Mrom {
    fn new() -> Mrom {
        const MROM_SIZE: u32 = 1024 * 1024 * 8; // 2^23
        let entry_point = 0x80000000;
        Mrom {
            // reset vector
            mrom: vec![
                0x00000297, // auipc   t0, 0x0
                0x02028593, // addi    a1, t0, 32
                0xf1402573, // csrr    a0, mhartid
                0x0182a283, // lw      t0, 24(t0)
                0x00028067, // jr      t0    |
                0x0,        //               |
                entry_point,// <-------------+
                0x0,
            ],
            base_addr: 0x1000,
        }
    }
}

