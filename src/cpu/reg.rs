use crate::cpu::instruction::reg2str;
use crate::cpu::CrossIsaUtil;
use crate::{log, Isa};
use std::rc::Rc;

pub struct Register {
    regs: [u64; 32],
    isa: Rc<Isa>,
}

impl Register {
    pub fn new(isa: Rc<Isa>) -> Self {
        Register { regs: [0; 32], isa }
    }

    pub fn show(&self) {
        //log::diffln!(
        //    "zero: {:#018x}  ra: {:#018x}  sp: {:#018x}  gp: {:#018x}\n  \
        //       tp: {:#018x}  t0: {:#018x}  t1: {:#018x}  t2: {:#018x}\n  \
        //       s0: {:#018x}  s1: {:#018x}  a0: {:#018x}  a1: {:#018x}\n  \
        //       a2: {:#018x}  a3: {:#018x}  a4: {:#018x}  a5: {:#018x}\n  \
        //       a6: {:#018x}  a7: {:#018x}  s2: {:#018x}  s3: {:#018x}\n  \
        //       s4: {:#018x}  s5: {:#018x}  s6: {:#018x}  s7: {:#018x}\n  \
        //       s8: {:#018x}  s9: {:#018x} s10: {:#018x} s11: {:#018x}\n  \
        //       t3: {:#018x}  t4: {:#018x}  t5: {:#018x}  t6: {:#018x}",
        //    self.regs[0],
        //    self.regs[1],
        //    self.regs[2],
        //    self.regs[3],
        //    self.regs[4],
        //    self.regs[5],
        //    self.regs[6],
        //    self.regs[7],
        //    self.regs[8],
        //    self.regs[9],
        //    self.regs[10],
        //    self.regs[11],
        //    self.regs[12],
        //    self.regs[13],
        //    self.regs[14],
        //    self.regs[15],
        //    self.regs[16],
        //    self.regs[17],
        //    self.regs[18],
        //    self.regs[19],
        //    self.regs[20],
        //    self.regs[21],
        //    self.regs[22],
        //    self.regs[23],
        //    self.regs[24],
        //    self.regs[25],
        //    self.regs[26],
        //    self.regs[27],
        //    self.regs[28],
        //    self.regs[29],
        //    self.regs[30],
        //    self.regs[31],
        //);

        if &crate::log::LogLv::Debug <= crate::log::LOG_LEVEL.get().unwrap() {
            println!("=========================================== dump ============================================");
            for (num, reg) in self.regs.iter().enumerate() {
                match *self.isa {
                    Isa::Rv32 => {
                        print!("{:>4}: 0x{:08x}\t", reg2str(num), reg);
                        if (num + 1) % 4 == 0 {
                            println!();
                        }
                    }
                    Isa::Rv64 => {
                        print!("{:>4}: 0x{:016x}\t", reg2str(num), reg);
                        if (num + 1) % 3 == 0 {
                            println!();
                        }
                    }
                }
            }
            println!("\n=============================================================================================");
        }
    }

    pub fn read(&self, src: Option<usize>) -> u64 {
        let src = src.unwrap();
        if src == 0 {
            0
        } else {
            self.regs[src].fix2regsz(&self.isa)
        }
    }

    pub fn write(&mut self, dist: Option<usize>, src: u64) {
        let dist = dist.unwrap();
        if dist != 0 {
            self.regs[dist] = src.fix2regsz(&self.isa);
        }
    }
}

impl Default for Register {
    fn default() -> Self {
        Self::new(Isa::Rv64.into())
    }
}
