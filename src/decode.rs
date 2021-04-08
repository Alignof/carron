mod mmap_parse;
use mmap_parse::*;
use crate::elfload::{get_u32};

// riscv-spec-20191213-1.pdf page=130
pub enum OpecodeKind{
	OP_LUI,
	OP_AUIPC,
	OP_JAL,
	OP_JALR,
	OP_BEQ,
	OP_BNE,
	OP_BLT,
	OP_BGE,
	OP_BLTU,
	OP_BGEU,
	OP_LB,
	OP_LH,
	OP_LW,
	OP_LBU,
	OP_LHU,
	OP_SB,
	OP_SH,
	OP_SW,
	OP_ADDI,
	OP_SLTI,
	OP_SLTIU,
	OP_XORI,
	OP_ORI,
	OP_ANDI,
	OP_SLLI,
	OP_SRLI,
	OP_ADD,
	OP_SUB,
	OP_SLL,
	OP_SLT,
	OP_SLTU,
	OP_XOR,
	OP_SRL,
	OP_SRA,
	OP_OR,
	OP_AND,
	OP_FENCE,
	OP_ECALL,
	OP_EBREAK,
}

pub struct Instruction {
	opc: OpecodeKind,
    rd: u8,
    rs1: u8,
    rs2: u8,
    imm: u32,
}

pub trait Decode {
	fn decode(&self, mmap: &[u8], index: usize) -> Instruction {
        let inst: u32 = get_u32(mmap, index);
        let new_opc: OpecodeKind = match parse_opecode(&inst){
            Ok(opc) => opc,
            Err(msg) => panic!("{}", msg),
        };
        let new_rd: u8 = parse_rd(&inst);
        let new_rs1: u8 = parse_rs1(&inst);
        let new_rs2: u8 = parse_rs2(&inst);
        let new_imm: u32 = parse_imm(&inst);

        Instruction {
            opc: new_opc,
            rd:  new_rd,
            rs1: new_rs1,
            rs2: new_rs2,
            imm: new_imm,
        }
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn opecode_parsing_test() {
        let mut test_inst: u32 = 0b00000000000000000000000000110111;
        assert!(matches!(parse_opecode(&test_inst).unwrap(), OpecodeKind::OP_LUI));

        test_inst = 0b00000000000000000000000000000011;
        assert!(matches!(parse_opecode(&test_inst).unwrap(), OpecodeKind::OP_LB));
        test_inst = 0b00000000000000000001000000000011;
        assert!(matches!(parse_opecode(&test_inst).unwrap(), OpecodeKind::OP_LH));
        test_inst = 0b00000000000000000000000000010011;
        assert!(matches!(parse_opecode(&test_inst).unwrap(), OpecodeKind::OP_ADDI));
        test_inst = 0b00000000000000000100000000110011;
        assert!(matches!(parse_opecode(&test_inst).unwrap(), OpecodeKind::OP_XOR));
        test_inst = 0b00000000000000000111000000110011;
        assert!(matches!(parse_opecode(&test_inst).unwrap(), OpecodeKind::OP_AND));
	}
}
