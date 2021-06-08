mod mmap_parse_16;
mod mmap_parse_32;

use super::instruction::{OpecodeKind, Instruction};

pub trait Decode {
    fn decode(&self) -> Instruction;
    fn parse_opecode(&self) -> Result<OpecodeKind, &'static str>;
    fn parse_rd(&self,  opkind: &OpecodeKind) -> Option<u8>;
    fn parse_rs1(&self, opkind: &OpecodeKind) -> Option<u8>;
    fn parse_rs2(&self, opkind: &OpecodeKind) -> Option<u8>;
    fn parse_imm(&self, opkind: &OpecodeKind) -> Option<u32>;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_opecode_test() {
        use OpecodeKind::*;
        let test_32 = |inst_32: u32, _e_op: OpecodeKind, _e_rd| {
            let op_32 = inst_32.parse_opecode().unwrap();
            assert!(matches!(&op_32, _e_op));
            assert_eq!(inst_32.parse_rd(&op_32).unwrap(), _e_rd);
        };

        test_32(0b00000000000000000000000010110111, OP_LUI, 1);
        test_32(0b00000000000000000000000000000011, OP_LB, 0);
        test_32(0b00000000000000000001000000000011, OP_LH, 0);
        test_32(0b00000000000000000000000000010011, OP_ADDI, 0);
        test_32(0b00000000000000000100000000110011, OP_XOR, 0);
        test_32(0b00000000000000000111000000110011, OP_AND, 0);
    }

    #[test]
    fn parsing_compressed_opecode_test() {
        use OpecodeKind::*;
        let test_16 = |inst_16: u16, _e_op: OpecodeKind, _e_rd: Option<u8>| {
            let op_16 = inst_16.parse_opecode().unwrap();
            assert!(matches!(&op_16, _e_op));
            assert!(matches!(inst_16.parse_rd(&op_16), _e_rd));
        };

        test_16(0b0000000000000001, OP_C_NOP, None);
        test_16(0b0000000010000001, OP_C_ADDI, Some(0));
        test_16(0b0110000100000001, OP_C_ADDI16SP, None);
        test_16(0b0110001110000001, OP_C_LUI, None);
        test_16(0b1000001011000001, OP_C_SRAI, Some(0));
        test_16(0b1000010011000001, OP_C_ANDI, None);
    }
}
