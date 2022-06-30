mod base_i;
mod m_extension;
mod a_extension;
mod zicsr_extension;
mod priv_extension;

use super::{Decode, DecodeUtil};
use crate::cpu::TrapCause;
use crate::cpu::instruction::{Extensions, OpecodeKind, Instruction};

#[allow(non_snake_case)]
impl Decode for u32 {
    fn decode(&self) -> Result<Instruction, (Option<i32>, TrapCause, String)> {
        let new_opc: OpecodeKind = match self.parse_opecode() {
            Ok(opc)  => opc,
            Err(msg) => return Err((
                None,
                TrapCause::IllegalInst,
                format!("{}, {:b}", msg, self)
            )),
        };
        let new_rd:  Option<usize>  = self.parse_rd(&new_opc);
        let new_rs1: Option<usize>  = self.parse_rs1(&new_opc);
        let new_rs2: Option<usize>  = self.parse_rs2(&new_opc);
        let new_imm: Option<i32> = self.parse_imm(&new_opc);

        Ok(Instruction {
            opc: new_opc,
            rd:  new_rd,
            rs1: new_rs1,
            rs2: new_rs2,
            imm: new_imm,
        })
    }

    fn parse_opecode(self) -> Result<OpecodeKind, &'static str> {
        match self.extension() {
            Extensions::BaseI => base_i::parse_opecode(self),
            Extensions::M => m_extension::parse_opecode(self),
            Extensions::A => a_extension::parse_opecode(self),
            Extensions::Zicsr => zicsr_extension::parse_opecode(self),
            Extensions::Priv => priv_extension::parse_opecode(self),
            _ => panic!("This instruction does not matched any extensions."),
        }
    }

    fn parse_rd(self, opkind: &OpecodeKind) -> Option<usize> {
        match self.extension() {
            Extensions::BaseI => base_i::parse_rd(self, opkind),
            Extensions::M => m_extension::parse_rd(self, opkind),
            Extensions::A => a_extension::parse_rd(self, opkind),
            Extensions::Zicsr => zicsr_extension::parse_rd(self, opkind),
            Extensions::Priv => priv_extension::parse_rd(self, opkind),
            _ => panic!("This instruction does not matched any extensions."),
        }
    }

    fn parse_rs1(self, opkind: &OpecodeKind) -> Option<usize> {
        match self.extension() {
            Extensions::BaseI => base_i::parse_rs1(self, opkind),
            Extensions::M => m_extension::parse_rs1(self, opkind),
            Extensions::A => a_extension::parse_rs1(self, opkind),
            Extensions::Zicsr => zicsr_extension::parse_rs1(self, opkind),
            Extensions::Priv => priv_extension::parse_rs1(self, opkind),
            _ => panic!("This instruction does not matched any extensions."),
        }
    }

    fn parse_rs2(self, opkind: &OpecodeKind) -> Option<usize> {
        match self.extension() {
            Extensions::BaseI => base_i::parse_rs2(self, opkind),
            Extensions::M => m_extension::parse_rs2(self, opkind),
            Extensions::A => a_extension::parse_rs2(self, opkind),
            Extensions::Zicsr => zicsr_extension::parse_rs2(self, opkind),
            Extensions::Priv => priv_extension::parse_rs2(self, opkind),
            _ => panic!("This instruction does not matched any extensions."),
        }
    }

    fn parse_imm(self, opkind: &OpecodeKind) -> Option<i32> {
        match self.extension() {
            Extensions::BaseI => base_i::parse_imm(self, opkind),
            Extensions::M => m_extension::parse_imm(self, opkind),
            Extensions::A => a_extension::parse_imm(self, opkind),
            Extensions::Zicsr => zicsr_extension::parse_imm(self, opkind),
            Extensions::Priv => priv_extension::parse_imm(self, opkind),
            _ => panic!("This instruction does not matched any extensions."),
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
            0b0101111 => Extensions::A,
            0b0110011 => match funct7 {
                0b0000001 => Extensions::M,
                _ => Extensions::BaseI,
            },
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

#[cfg(test)]
#[allow(unused_variables)]
mod decode_32 {
    use super::*;

    #[test]
    #[allow(overflowing_literals)]
    fn parsing_opecode_test() {
        use OpecodeKind::*;
        let test_32 = |inst_32: u32, op: OpecodeKind, rd: Option<usize>,
                       rs1: Option<usize>, rs2: Option<usize>, imm: Option<i32>| {
            let op_32 = inst_32.parse_opecode().unwrap();
            assert!(matches!(&op_32, op));
            assert_eq!(inst_32.parse_rd(&op_32), rd);
            assert_eq!(inst_32.parse_rs1(&op_32), rs1);
            assert_eq!(inst_32.parse_rs2(&op_32), rs2);
            assert_eq!(inst_32.parse_imm(&op_32), imm);
        };

        test_32(0b10000000000000000000000010110111,
                OP_LUI, Some(1), None, None, Some(0x80000000));
        test_32(0b00000000000000000000001010010111,
                OP_AUIPC, Some(5), None, None, Some(0));
        test_32(0b11111111100111111111000001101111,
                OP_JAL, Some(0), None, None, Some(-8));
        test_32(0b11111110001000001000111010100011,
                OP_SB, None, Some(1), Some(2), Some(-3));
        test_32(0b11101110110000101000001010010011,
                OP_ADDI, Some(5), Some(5), None, Some(-276));
        test_32(0b00000000000000000000000001110011,
                OP_ECALL, None, None, None, None);
        test_32(0b00000000000001010100110001100011,
                OP_BLT, None, Some(10), Some(0), Some(24));
        test_32(0x00100513, OP_ADDI, Some(10), Some(0), None, Some(1))
    }
}
