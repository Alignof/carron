mod mmap_parse;
use mmap_parse::*;
use crate::elfload::{get_u32};

// riscv-spec-20191213-1.pdf page=130
#[allow(non_camel_case_types)]
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
	pub opc: OpecodeKind,
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub imm: u32,
}

impl Instruction {
    pub fn opc_to_string(&self) -> &'static str {
        use OpecodeKind::*;
        match self.opc {
            OP_LUI		=> "lui",
            OP_AUIPC	=> "auipc",
            OP_JAL		=> "jal",
            OP_JALR		=> "jalr",
            OP_BEQ		=> "beq",
            OP_BNE		=> "bne",
            OP_BLT		=> "blt",
            OP_BGE		=> "bge",
            OP_BLTU		=> "bltu",
            OP_BGEU		=> "bgeu",
            OP_LB		=> "lb",
            OP_LH		=> "lh",
            OP_LW		=> "lw",
            OP_LBU		=> "lbu",
            OP_LHU		=> "lhu",
            OP_SB		=> "sb",
            OP_SH		=> "sh",
            OP_SW		=> "sw",
            OP_ADDI		=> "addi",
            OP_SLTI		=> "slti",
            OP_SLTIU	=> "sltiu",
            OP_XORI		=> "xori",
            OP_ORI		=> "ori",
            OP_ANDI		=> "andi",
            OP_SLLI		=> "slli",
            OP_SRLI		=> "srli",
            OP_ADD		=> "add",
            OP_SUB		=> "sub",
            OP_SLL		=> "sll",
            OP_SLT		=> "slt",
            OP_SLTU		=> "sltu",
            OP_XOR		=> "xor",
            OP_SRL		=> "srl",
            OP_SRA		=> "sra",
            OP_OR		=> "or",
            OP_AND		=> "and",
            OP_FENCE	=> "fence",
            OP_ECALL	=> "ecall",
            OP_EBREAK	=> "ebreak",
        }
    }
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
