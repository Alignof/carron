#[allow(non_snake_case)]
mod c_extension;

use super::{Decode, DecodeUtil};
use crate::cpu::TrapCause;
use crate::cpu::instruction::{Extensions, OpecodeKind, Instruction};

impl Decode for u16 {
    fn decode(&self) -> Result<Instruction, (Option<i32>, TrapCause, String)> {
        let new_opc: OpecodeKind = match self.parse_opecode() {
            Ok(opc)  => opc,
            Err(msg) => return Err((
                None,
                TrapCause::IllegalInst,
                format!("{}, {:b}", msg, self)
            )),
        };
        let new_rd:  Option<usize>  = self.parse_rd(&new_opc)?;
        let new_rs1: Option<usize>  = self.parse_rs1(&new_opc)?;
        let new_rs2: Option<usize>  = self.parse_rs2(&new_opc)?;
        let new_imm: Option<i32> = self.parse_imm(&new_opc)?;

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
            Extensions::C => c_extension::parse_opecode(self),
            _ => panic!("It isn't compressed instruction"),
        }
    }

    fn parse_rd(self, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<i32>, TrapCause, String)> {
        match self.extension() {
            Extensions::C => c_extension::parse_rd(self, opkind),
            _ => panic!("It isn't compressed instruction"),
        }
    }

    fn parse_rs1(self, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<i32>, TrapCause, String)> {
        match self.extension() {
            Extensions::C => c_extension::parse_rs1(self, opkind),
            _ => panic!("It isn't compressed instruction"),
        }
    }

    fn parse_rs2(self, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<i32>, TrapCause, String)> {
        match self.extension() {
            Extensions::C => c_extension::parse_rs2(self, opkind),
            _ => panic!("It isn't compressed instruction"),
        }
    }

    fn parse_imm(self, opkind: &OpecodeKind) -> Result<Option<i32>, (Option<i32>, TrapCause, String)> {
        match self.extension() {
            Extensions::C => c_extension::parse_imm(self, opkind),
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

#[cfg(test)]
#[allow(unused_variables)]
mod decode_16 {
    use super::*;

    #[test]
    fn parsing_compressed_opecode_test() {
        use OpecodeKind::*;
        let test_16 = |inst_16: u16, _op: OpecodeKind, _rd: Option<u8>| {
            let op_16 = inst_16.parse_opecode().unwrap();
            assert!(matches!(&op_16, _op));
            assert!(matches!(inst_16.parse_rd(&op_16), _rd));
        };

        test_16(0b0000000000000001, OP_C_NOP, None);
        test_16(0b0000000010000001, OP_C_ADDI, Some(0));
        test_16(0b0110000100000001, OP_C_ADDI16SP, None);
        test_16(0b0110001110000001, OP_C_LUI, None);
        test_16(0b1000001011000001, OP_C_SRAI, Some(0));
        test_16(0b1000010011000001, OP_C_ANDI, None);
    }
}
