use super::Device;

pub struct Mrom {
    pub mrom: Vec<u8>,
    pub base_addr: u32,
}

impl Mrom {
    #[allow(arithmetic_overflow)]
    pub fn new(entry_point: u32) -> Mrom {
        let reset_vector: Vec<u32> = vec![
            0x00000297, // auipc   t0, 0x0
            0x02028593, // addi    a1, t0, 32
            0xf1402573, // csrr    a0, mhartid
            0x0182a283, // lw      t0, 24(t0)
            0x00028067, // jr      t0    |
            0x0,        //               |
            entry_point,// <-------------+
            entry_point >> 32,
        ];

        // Vec<u32> -> Vec<u8>
        let mrom: Vec<u8> = reset_vector 
            .iter()
            .flat_map(|val| val.to_le_bytes().to_vec())
            .collect();

        Mrom {
            mrom,
            base_addr: 0x1000,
        }
    }
}

impl Device for Mrom {
    // set 1 byte
    fn store_byte(&self, addr: u32, data: u8) {}

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


    // get 1 byte
    fn load_byte(&self, addr: u32) -> u8 {
        if addr < self.base_addr {
            panic!("invalid address for mrom: {}", addr);
        }

        self.mrom[addr - self.base_addr]
    }

    // load
    fn load8(&self, addr: u32) -> i32 {
        self.load_byte(addr) as i8 as i32
    }

    fn load16(&self, addr: u32) -> i32 {
        ((self.load_byte(addr + 1) as u16) << 8 |
         (self.load_byte(addr + 0) as u16)) as i16 as i32
    }

    fn load32(&self, addr: u32) -> i32 {
        ((self.load_byte(addr + 3) as u32) << 24 |
         (self.load_byte(addr + 2) as u32) << 16 |
         (self.load_byte(addr + 1) as u32) <<  8 |
         (self.load_byte(addr + 0) as u32)) as i32
    }

    fn load_u8(&self, addr: u32) -> i32 {
        self.load_byte(addr) as i32
    }

    fn load_u16(&self, addr: u32) -> i32 {
        ((self.load_byte(addr + 1) as u32) << 8 |
         (self.load_byte(addr + 0) as u32)) as i32
    }
}
