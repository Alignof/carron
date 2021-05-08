mod mmap_parse_16;
mod mmap_parse_32;

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
//== compressed Instruction == 
    OP_C_ADDI4SPN,
    OP_C_FLD,
    OP_C_LW,
    OP_C_FLW,
    OP_C_FSD,
    OP_C_SW,
    OP_C_FSW,
    OP_C_NOP,
    OP_C_ADDI,
    OP_C_JAL,
    OP_C_LI,
    OP_C_ADDI16SP,
    OP_C_LUI,
    OP_C_SRLI,
    OP_C_SRAI,
    OP_C_ANDI,
    OP_C_SUB,
    OP_C_XOR,
    OP_C_OR,
    OP_C_AND,
    OP_C_J,
    OP_C_BEQZ,
    OP_C_BNEZ,
    OP_C_SLLI,
    OP_C_FLDSP,
    OP_C_LWSP,
    OP_C_FLWSP,
    OP_C_JR,
    OP_C_MV,
    OP_C_EBREAK,
    OP_C_JALR,
    OP_C_ADD,
    OP_C_FSDSP,
    OP_C_SWSP,
    OP_C_FSWSP,
}

pub struct Instruction {
	pub opc: OpecodeKind,
    pub rd:  Option<u8>,
    pub rs1: Option<u8>,
    pub rs2: Option<u8>,
    pub imm: Option<u32>,
    pub is_compressed: bool,
}

impl Instruction {
    pub fn opc_to_string(&self) -> &'static str {
        use OpecodeKind::*;
        match self.opc {
            OP_LUI		    => "lui",
            OP_AUIPC	    => "auipc",
            OP_JAL		    => "jal",
            OP_JALR		    => "jalr",
            OP_BEQ		    => "beq",
            OP_BNE		    => "bne",
            OP_BLT		    => "blt",
            OP_BGE		    => "bge",
            OP_BLTU		    => "bltu",
            OP_BGEU		    => "bgeu",
            OP_LB		    => "lb",
            OP_LH		    => "lh",
            OP_LW		    => "lw",
            OP_LBU		    => "lbu",
            OP_LHU		    => "lhu",
            OP_SB		    => "sb",
            OP_SH		    => "sh",
            OP_SW		    => "sw",
            OP_ADDI		    => "addi",
            OP_SLTI		    => "slti",
            OP_SLTIU	    => "sltiu",
            OP_XORI		    => "xori",
            OP_ORI		    => "ori",
            OP_ANDI		    => "andi",
            OP_SLLI		    => "slli",
            OP_SRLI		    => "srli",
            OP_ADD		    => "add",
            OP_SUB		    => "sub",
            OP_SLL		    => "sll",
            OP_SLT		    => "slt",
            OP_SLTU		    => "sltu",
            OP_XOR		    => "xor",
            OP_SRL		    => "srl",
            OP_SRA		    => "sra",
            OP_OR		    => "or",
            OP_AND		    => "and",
            OP_FENCE	    => "fence",
            OP_ECALL	    => "ecall",
            OP_EBREAK	    => "ebreak",
            OP_C_ADDI4SPN	=> "C.addi4spn",
            OP_C_FLD		=> "C.fld",
            OP_C_LW		    => "C.lw",
            OP_C_FLW		=> "C.flw",
            OP_C_FSD		=> "C.fsd",
            OP_C_SW		    => "C.sw",
            OP_C_FSW		=> "C.fsw",
            OP_C_NOP		=> "C.nop",
            OP_C_ADDI		=> "C.addi",
            OP_C_JAL		=> "C.jal",
            OP_C_LI		    => "C.li",
            OP_C_ADDI16SP	=> "C.addi16sp",
            OP_C_LUI		=> "C.lui",
            OP_C_SRLI		=> "C.srli",
            OP_C_SRAI		=> "C.srai",
            OP_C_ANDI		=> "C.andi",
            OP_C_SUB		=> "C.sub",
            OP_C_XOR		=> "C.xor",
            OP_C_OR		    => "C.or",
            OP_C_AND		=> "C.and",
            OP_C_J		    => "C.j",
            OP_C_BEQZ		=> "C.beqz",
            OP_C_BNEZ		=> "C.bnez",
            OP_C_SLLI		=> "C.slli",
            OP_C_FLDSP		=> "C.fldsp",
            OP_C_LWSP		=> "C.lwsp",
            OP_C_FLWSP		=> "C.flwsp",
            OP_C_JR		    => "C.jr",
            OP_C_MV		    => "C.mv",
            OP_C_EBREAK		=> "C.ebreak",
            OP_C_JALR		=> "C.jalr",
            OP_C_ADD		=> "C.add",
            OP_C_FSDSP		=> "C.fsdsp",
            OP_C_SWSP		=> "C.swsp",
            OP_C_FSWSP		=> "C.fswsp",
        }
    }

    pub fn reg_to_string(&self) -> &'static str {
        match self.rd {
            Some(rd) => match rd {
                0  => "zero",
                1  => "ra",
                2  => "sp",
                3  => "gp",
                4  => "tp",
                5  => "t0",
                6  => "t1",
                7  => "t2",
                8  => "fp",
                9  => "s1",
                10 => "a0",
                11 => "a1",
                12 => "a2",
                13 => "a3",
                14 => "a4",
                15 => "a5",
                16 => "a6",
                17 => "a7",
                18 => "s2",
                19 => "s3",
                20 => "s4",
                21 => "s5",
                22 => "s6",
                23 => "s7",
                24 => "s8",
                25 => "s9",
                26 => "s10",
                27 => "s11",
                28 => "t3",
                29 => "t4",
                30 => "t5",
                31 => "t6",
                _  => panic!("unknown register"),
            },
            None => "  ",
        }
    }
}


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
        let mut test_32: u32 = 0b00000000000000000000000000110111;
        assert!(matches!(test_32.parse_opecode().unwrap(), OpecodeKind::OP_LUI));

        test_32 = 0b00000000000000000000000000000011;
        assert!(matches!(test_32.parse_opecode().unwrap(), OpecodeKind::OP_LB));
        test_32 = 0b00000000000000000001000000000011;
        assert!(matches!(test_32.parse_opecode().unwrap(), OpecodeKind::OP_LH));
        test_32 = 0b00000000000000000000000000010011;
        assert!(matches!(test_32.parse_opecode().unwrap(), OpecodeKind::OP_ADDI));
        test_32 = 0b00000000000000000100000000110011;
        assert!(matches!(test_32.parse_opecode().unwrap(), OpecodeKind::OP_XOR));
        test_32 = 0b00000000000000000111000000110011;
        assert!(matches!(test_32.parse_opecode().unwrap(), OpecodeKind::OP_AND));
	}

	fn parsing_compressed_opecode_test() {
        let mut test_16: u16 = 0b00000000000000000000000000110111;
        assert!(matches!(test_16.parse_opecode().unwrap(), OpecodeKind::OP_LUI));
    }

}
