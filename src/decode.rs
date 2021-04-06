use crate::elfload::{get_u32};

// riscv-spec-20191213-1.pdf page=130
enum OpecodeKind{
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
	OP_CSRRW,
	OP_CSRRS,
	OP_CSRRC,
	OP_CSRRWI,
	OP_CSRRSI,
	OP_CSRRCI,
	OP_LWU,
	OP_LD,
	OP_SD,
	OP_SRAI,
	OP_ADDIW,
	OP_SLLIW,
	OP_SRLIW,
	OP_SRAIW,
	OP_ADDW,
	OP_SUBW,
	OP_SLLW,
	OP_SRLW,
	OP_SRAW,
};

fn parse_opecode(mmap: &[u8], index: usize) -> OpecodeKind {
    inst: u32 = get_u32(mmap, index);
    opmap: u8 = inst & 0x3F;
    funct3: u8 = inst & 0x300;

    match opmap {
        0b0110111 => OP_LUI,
        0b0010111 => OP_AUIPC,
        0b1101111 => OP_JAL,
        0b1100011 => match funct3 {
            0b000 => OP_BEQ,
            0b001 => OP_BNE,
            0b100 => OP_BLT,
            0b101 => OP_BGE,
            0b110 => OP_BLTU,
            0b111 => OP_BGEU,
        },
        0b0000011 => match funct3 {
            0b000 => OP_LB,
            0b001 => OP_LH,
            0b010 => OP_LW,
            0b100 => OP_LBU,
            0b101 => OP_LHU,
        },
        0b0100011 => match funct3 {
            0b000 => OP_SB,
            0b001 => OP_SH,
            0b010 => OP_SW,
        },
        0b0010011 => match funct3 {
            0b000 => OP_ADDI,
            0b001 => OP_SLLI,
            0b010 => OP_SLTI,
            0b011 => OP_SLTIU,
            0b100 => OP_XORI,
            0b101 => OP_SRLI,//OP_SRAI,
            0b110 => OP_ORI,
            0b111 => OP_ANDI,
        },
        0b0110011 => match funct3 {
            0b000 => OP_ADD,//OP_SUB,
            0b001 => OP_SLL,
            0b010 => OP_SLT,
            0b011 => OP_SLTU,
            0b100 => OP_XOR,
            0b101 => OP_SRL,//OP_SRA,
            0b110 => OP_OR,
            0b111 => OP_ANI,
        },
        0b0001111 => OP_FENCE,
        0b1110011 => OP_FCALL,//OP_EBREAK,
    }
}


struct Instruction {
	opc: OpecodeKind,
}

pub trait Decode {
	fn decode(&self, mmap: &[u8], index: usize) -> Instruction {
        let new_op: OpecodeKind = parse_opecode();

        Instruction {
            new_op,
        }
    }
}
