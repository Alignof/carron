mod mmap_parse_16;
mod mmap_parse_32;

use super::instruction::{OpecodeKind, Instruction};

pub trait Decode {
    fn decode(&self) -> Instruction;
    fn parse_opecode(&self) -> Result<OpecodeKind, &'static str>;
    fn parse_rd(&self,  opkind: &OpecodeKind) -> Option<usize>;
    fn parse_rs1(&self, opkind: &OpecodeKind) -> Option<usize>;
    fn parse_rs2(&self, opkind: &OpecodeKind) -> Option<usize>;
    fn parse_imm(&self, opkind: &OpecodeKind) -> Option<i32>;
}


#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use super::*;

    #[test]
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

        test_32(0b00000000000000000000000010110111,
                OP_LUI, Some(1), None, None, Some(0));
        test_32(0b00000000000000000000001010010111,
                OP_AUIPC, Some(5), None, None, Some(0));
        test_32(0b00000000000000000000000001110011,
                OP_ECALL, None, None, None, None);
        test_32(0b00000000000001010100110001100011,
                OP_BLT, None, Some(10), Some(0), Some(24));
    }

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
