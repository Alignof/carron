mod inst_16;
mod inst_32;

use super::instruction::{Extensions, OpecodeKind, Instruction};

pub trait Decode {
    fn decode(&self) -> Instruction;
    fn parse_opecode(self) -> Result<OpecodeKind, &'static str>;
    fn parse_rd(self,  opkind: &OpecodeKind) -> Option<usize>;
    fn parse_rs1(self, opkind: &OpecodeKind) -> Option<usize>;
    fn parse_rs2(self, opkind: &OpecodeKind) -> Option<usize>;
    fn parse_imm(self, opkind: &OpecodeKind) -> Option<i32>;
}

pub trait DecodeUtil {
    fn slice(self, end: u32, start: u32) -> Self;
    fn set(self, mask: &[u32]) -> u32;
    fn extension(self) -> Extensions;
    fn to_signed_nbit(&self, imm32: i32, bit_size: u32) -> i32 {
        let imm32 = imm32 & (2_i32.pow(bit_size) - 1);
        if imm32 >> (bit_size - 1) & 0x1 == 1 {
            imm32 - 2_i32.pow(bit_size)
        } else {
            imm32
        }
    }
}

