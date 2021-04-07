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

fn parse_opecode(mmap: &[u8], inst:&u32) -> OpecodeKind {
    let opmap: u8  = (inst & 0x3F) as u8;
    let funct3: u8 = (inst & 0x300) as u8;

    match opmap {
        0b0110111 => OpecodeKind::OP_LUI,
        0b0010111 => OpecodeKind::OP_AUIPC,
        0b1101111 => OpecodeKind::OP_JAL,
        0b1100011 => match funct3 {
            0b000 => OpecodeKind::OP_BEQ,
            0b001 => OpecodeKind::OP_BNE,
            0b100 => OpecodeKind::OP_BLT,
            0b101 => OpecodeKind::OP_BGE,
            0b110 => OpecodeKind::OP_BLTU,
            0b111 => OpecodeKind::OP_BGEU,
        },
        0b0000011 => match funct3 {
            0b000 => OpecodeKind::OP_LB,
            0b001 => OpecodeKind::OP_LH,
            0b010 => OpecodeKind::OP_LW,
            0b100 => OpecodeKind::OP_LBU,
            0b101 => OpecodeKind::OP_LHU,
        },
        0b0100011 => match funct3 {
            0b000 => OpecodeKind::OP_SB,
            0b001 => OpecodeKind::OP_SH,
            0b010 => OpecodeKind::OP_SW,
        },
        0b0010011 => match funct3 {
            0b000 => OpecodeKind::OP_ADDI,
            0b001 => OpecodeKind::OP_SLLI,
            0b010 => OpecodeKind::OP_SLTI,
            0b011 => OpecodeKind::OP_SLTIU,
            0b100 => OpecodeKind::OP_XORI,
            0b101 => OpecodeKind::OP_SRLI,//OP_SRAI,
            0b110 => OpecodeKind::OP_ORI,
            0b111 => OpecodeKind::OP_ANDI,
        },
        0b0110011 => match funct3 {
            0b000 => OpecodeKind::OP_ADD,//OP_SUB,
            0b001 => OpecodeKind::OP_SLL,
            0b010 => OpecodeKind::OP_SLT,
            0b011 => OpecodeKind::OP_SLTU,
            0b100 => OpecodeKind::OP_XOR,
            0b101 => OpecodeKind::OP_SRL,//OP_SRA,
            0b110 => OpecodeKind::OP_OR,
            0b111 => OpecodeKind::OP_AND,
        },
        0b0001111 => OpecodeKind::OP_FENCE,
        0b1110011 => OpecodeKind::OP_ECALL,//OP_EBREAK,
    }
}


pub struct Instruction {
	opc: OpecodeKind,
}

pub trait Decode {
	fn decode(&self, mmap: &[u8], index: usize) -> Instruction {
        let inst: u32 = get_u32(mmap, index);
        let new_opc: OpecodeKind = parse_opecode(&mmap, &inst);

        Instruction {
            opc: new_opc,
        }
    }
}

#[cfg(test)]
mod tests {
	#[test]
	fn opecode_parsing_test() {
		assert_eq!(0b00000000000000000000000000010111, OP_LUI);
	}
}

