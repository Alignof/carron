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
                .flat_map(|val| val.to_le_bytes().to_vec())
                .collect(),
            // https://github.com/qemu/qemu/blob/b37778b840f6dc6d1bbaf0e8e0641b3d48ad77c5/hw/riscv/virt.c#L47
            base_addr: 0x1000,
        }
    }
}

impl Device for Mrom {
    // address to raw index
    fn addr2index(&self, addr: u32) -> usize {
        if addr < self.base_addr {
            panic!("invalid address for mrom: {}", addr);
        }

        (addr - self.base_addr) as usize
    }

    // get 1 byte
    fn raw_byte(&self, addr: u32) -> u8 {
        let addr = self.addr2index(addr);
        self.mrom[addr]
    }

    // store
    fn store8(&mut self, _addr: u32, _data: i32) {
        panic!("mrom is read only.");
    }

    fn store16(&mut self, _addr: u32, _data: i32) {
        panic!("mrom is read only.");
    }

    fn store32(&mut self, _addr: u32, _data: i32) {
        panic!("mrom is read only.");
    }


    // load
    fn load8(&self, addr: u32) -> i32 {
        let addr = self.addr2index(addr);
        self.mrom[addr] as i8 as i32
    }

    fn load16(&self, addr: u32) -> i32 {
        let addr = self.addr2index(addr);
        ((self.mrom[addr + 1] as u16) << 8 |
         (self.mrom[addr + 0] as u16)) as i16 as i32
    }

    fn load32(&self, addr: u32) -> i32 {
        let addr = self.addr2index(addr);
        ((self.mrom[addr + 3] as u32) << 24 |
         (self.mrom[addr + 2] as u32) << 16 |
         (self.mrom[addr + 1] as u32) <<  8 |
         (self.mrom[addr + 0] as u32)) as i32
    }

    fn load_u8(&self, addr: u32) -> i32 {
        let addr = self.addr2index(addr);
        self.mrom[addr] as i32
    }

    fn load_u16(&self, addr: u32) -> i32 {
        let addr = self.addr2index(addr);
        ((self.mrom[addr + 1] as u32) << 8 |
         (self.mrom[addr + 0] as u32)) as i32
    }
}
