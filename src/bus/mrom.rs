use super::Device;

pub struct Mrom {
        mrom: Vec<u8>,
    pub base_addr: u32,
}

impl Mrom {
    pub fn new(entry_point: u32) -> Mrom {
        let reset_vector: Vec<u32> = vec![
            0x00000297, // auipc   t0, 0x0
            0x02028593, // addi    a1, t0, 32
            0xf1402573, // csrr    a0, mhartid
            0x0182a283, // lw      t0, 24(t0)
            0x00028067, // jr      t0    |
            0x0,        //               |
            entry_point,// <-------------+
            0x0,
        ];

        Mrom {
            // Vec<u32> -> Vec<u8>
            mrom: reset_vector
                .iter()
                .flat_map(|val| val.to_be_bytes().to_vec())
                .collect(),
            // https://github.com/qemu/qemu/blob/b37778b840f6dc6d1bbaf0e8e0641b3d48ad77c5/hw/riscv/virt.c#L47
            base_addr: 0x1000,
        }
    }
}

