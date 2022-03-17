#[allow(non_snake_case)]
mod C_extension;

use super::{Decode, DecodeUtil};
use crate::cpu::instruction::{Extensions, OpecodeKind, Instruction};

impl Decode for u16 {
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
            is_compressed: true,
        }
    }

    fn parse_opecode(self) -> Result<OpecodeKind, &'static str> {
        match self.extension() {
            Extensions::C => C_extension::parse_opecode(self),
            _ => panic!("It isn't compressed instruction"),
        }
    }

    fn parse_rd(self, opkind: &OpecodeKind) -> Option<usize> {
        match self.extension() {
            Extensions::C => C_extension::parse_rd(self, opkind),
            _ => panic!("It isn't compressed instruction"),
        }
    }

    fn parse_rs1(self, opkind: &OpecodeKind) -> Option<usize> {
        match self.extension() {
            Extensions::C => C_extension::parse_rs1(self, opkind),
            _ => panic!("It isn't compressed instruction"),
        }
    }

    fn parse_rs2(self, opkind: &OpecodeKind) -> Option<usize> {
        match self.extension() {
            Extensions::C => C_extension::parse_rs2(self, opkind),
            _ => panic!("It isn't compressed instruction"),
        }
    }

    fn parse_imm(self, opkind: &OpecodeKind) -> Option<i32> {
        match self.extension() {
            Extensions::C => C_extension::parse_imm(self, opkind),
            _ => panic!("It isn't compressed instruction"),
        }
    }
}

impl DecodeUtil for u16 {
    fn slice(self, end: u32, start: u32) -> u16 {
        (self >> start) & (2_u16.pow(end - start + 1) - 1)
    }

    fn set(self, mask: &[u32]) -> u32 {
        let mut inst: u32 = 0;
        for (i, m) in mask.iter().rev().enumerate() {
            inst |= ((self as u32 >> i) & 0x1) << m;
        }

        inst
    }

    fn extension(self) -> Extensions {
        Extensions::C
    }
}
