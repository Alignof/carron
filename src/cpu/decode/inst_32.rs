#[allow(non_snake_case)]
mod BaseI_extension;

use super::{Decode, DecodeUtil};
use crate::cpu::instruction::{Extensions, OpecodeKind, Instruction};

#[allow(non_snake_case)]
impl Decode for u32 {
    fn decode(&self) -> Instruction {
        let new_opc: OpecodeKind = match self.parse_opecode() {
            Ok(opc)  => opc,
            Err(msg) => panic!("{}, {:b}", msg, self),
        };
        let new_rd:  Option<usize>  = self.parse_rd(&new_opc);
        let new_rs1: Option<usize>  = self.parse_rs1(&new_opc);
        let new_rs2: Option<usize>  = self.parse_rs2(&new_opc);
        let new_imm: Option<i32> = self.parse_imm(&new_opc);

        Instruction {
            opc: new_opc,
            rd:  new_rd,
            rs1: new_rs1,
            rs2: new_rs2,
            imm: new_imm,
            is_compressed: false,
        }
    }

    fn parse_opecode(self) -> Result<OpecodeKind, &'static str> {
        match self.extension() {
            Extensions::BaseI => BaseI_extension::parse_opecode(self),
            _ => panic!("It isn't compressed instruction"),
        }
    }

    fn parse_rd(self, opkind: &OpecodeKind) -> Option<usize> {
        match self.extension() {
            Extensions::BaseI => BaseI_extension::parse_rd(self, opkind),
            _ => panic!("It isn't compressed instruction"),
        }
    }

    fn parse_rs1(self, opkind: &OpecodeKind) -> Option<usize> {
        match self.extension() {
            Extensions::BaseI => BaseI_extension::parse_rs1(self, opkind),
            _ => panic!("It isn't compressed instruction"),
        }
    }

    fn parse_rs2(self, opkind: &OpecodeKind) -> Option<usize> {
        match self.extension() {
            Extensions::BaseI => BaseI_extension::parse_rs2(self, opkind),
            _ => panic!("It isn't compressed instruction"),
        }
    }

    fn parse_imm(self, opkind: &OpecodeKind) -> Option<i32> {
        match self.extension() {
            Extensions::BaseI => BaseI_extension::parse_imm(self, opkind),
            _ => panic!("It isn't compressed instruction"),
        }
    }
}

impl DecodeUtil for u32 {
    fn slice(self, end: u32, start: u32) -> u32 {
        (self >> start) & (2_u32.pow(end - start + 1) - 1)
    }

    fn set(self, mask: &[u32]) -> u32 {
        let mut inst: u32 = 0;
        for (i, m) in mask.iter().rev().enumerate() {
            inst |= ((self >> i) & 0x1) << m;
        }

        inst
    }

    fn extension(self) -> Extensions {
        let opmap: u8  = self.slice(6, 0) as u8;
        let funct3: u8 = self.slice(14, 12) as u8;
        let funct7: u8 = self.slice(31, 25) as u8;
        match opmap {
            0b0101111 => Extensions::M,
            0b0111011 => Extensions::A,
            0b1110011 => match funct3 {
                0b000 => match funct7 {
                    0b0000000 => Extensions::BaseI,
                    _ => Extensions::Priv,
                },
                _     => Extensions::Zicsr,
            },
            _ => Extensions::BaseI,
        }
    }
}
