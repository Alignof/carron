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

fn parse_opecode(inst:&u32) -> Result<OpecodeKind, &'static str> {
    let opmap: u8  = (inst & 0x3F) as u8;
    let funct3: u8 = ((inst >> 12) & 0x7) as u8;

    match opmap {
        0b0110111 => Ok(OpecodeKind::OP_LUI),
        0b0010111 => Ok(OpecodeKind::OP_AUIPC),
        0b1101111 => Ok(OpecodeKind::OP_JAL),
        0b1100011 => match funct3 {
            0b000 => Ok(OpecodeKind::OP_BEQ),
            0b001 => Ok(OpecodeKind::OP_BNE),
            0b100 => Ok(OpecodeKind::OP_BLT),
            0b101 => Ok(OpecodeKind::OP_BGE),
            0b110 => Ok(OpecodeKind::OP_BLTU),
            0b111 => Ok(OpecodeKind::OP_BGEU),
            _     => Err("opecode decoding failed"),
        },
        0b0000011 => match funct3 {
            0b000 => Ok(OpecodeKind::OP_LB),
            0b001 => Ok(OpecodeKind::OP_LH),
            0b010 => Ok(OpecodeKind::OP_LW),
            0b100 => Ok(OpecodeKind::OP_LBU),
            0b101 => Ok(OpecodeKind::OP_LHU),
            _     => Err("opecode decoding failed"),
        },
        0b0100011 => match funct3 {
            0b000 => Ok(OpecodeKind::OP_SB),
            0b001 => Ok(OpecodeKind::OP_SH),
            0b010 => Ok(OpecodeKind::OP_SW),
            _     => Err("opecode decoding failed"),
        },
        0b0010011 => match funct3 {
            0b000 => Ok(OpecodeKind::OP_ADDI),
            0b001 => Ok(OpecodeKind::OP_SLLI),
            0b010 => Ok(OpecodeKind::OP_SLTI),
            0b011 => Ok(OpecodeKind::OP_SLTIU),
            0b100 => Ok(OpecodeKind::OP_XORI),
            0b101 => Ok(OpecodeKind::OP_SRLI),//OP_SRAI,
            0b110 => Ok(OpecodeKind::OP_ORI),
            0b111 => Ok(OpecodeKind::OP_ANDI),
            _     => Err("opecode decoding failed"),
        },
        0b0110011 => match funct3 {
            0b000 => Ok(OpecodeKind::OP_ADD),//OP_SUB,
            0b001 => Ok(OpecodeKind::OP_SLL),
            0b010 => Ok(OpecodeKind::OP_SLT),
            0b011 => Ok(OpecodeKind::OP_SLTU),
            0b100 => Ok(OpecodeKind::OP_XOR),
            0b101 => Ok(OpecodeKind::OP_SRL),//OP_SRA,
            0b110 => Ok(OpecodeKind::OP_OR),
            0b111 => Ok(OpecodeKind::OP_AND),
            _     => Err("opecode decoding failed"),
        },
        0b0001111 => Ok(OpecodeKind::OP_FENCE),
        0b1110011 => Ok(OpecodeKind::OP_ECALL),//OP_EBREAK,
        _         => Err("opecode decoding failed"),
    }
}

fn parse_rd(inst: &u32) -> u8 {
    let opmap: u8  = (inst & 0x3F) as u8;
    let rd: u8 = ((inst >> 7) & 0x1F) as u8;

    // B(EQ|NE|LT|GE|LTU|GEU), S(B|H|W), ECALL, EBREAK
    if  opmap == 0b01100011 || opmap == 0b00100011 || 
        opmap == 0b01110011 { 
            return 0;
    }

    return rd;
}

fn parse_rs1(inst: &u32) -> u8 {
    let opmap: u8  = (inst & 0x3F) as u8;
    let rs1: u8 = ((inst >> 15) & 0x1F) as u8;

    // LUI, AUIPC, JAL, FENCE, ECALL, EBREAK
    if  opmap == 0b01010111 || opmap == 0b00010111 || 
        opmap == 0b01101111 || opmap == 0b01110011 { 
            return 0;
    }

    return rs1;
}

fn parse_rs2(inst: &u32) -> u8 {
    let opmap: u8  = (inst & 0x3F) as u8;
    let rs2: u8 = ((inst >> 20) & 0x1F) as u8;

    // LUI, AUIPC, JAL, JALR L(B|H|W|BU|HU),
    // ADDI, SLTI, SLTIU, XORI, ORI, ANDI, SLLI,
    // FENCE, ECALL, EBREAK
    if  opmap == 0b01010111 || opmap == 0b00010111 || opmap == 0b01101111 ||
        opmap == 0b01100111 || opmap == 0b00000011 || opmap == 0b00010011 || 
        opmap == 0b00001111 || opmap == 0b01110011 { 
            return 0;
    }

    return rs2;
}


fn parse_imm(inst: &u32) -> u32 {
    let opmap: u8  = (inst & 0x3F) as u8;

    // LUI, AUIPC
    if opmap == 0b00110111 || opmap == 0b00010111 {
        return ((inst >> 12) & 0xFFFFF) as u32;
    }

    // JAL
    if opmap == 0b01101111 {
        return ((inst >> 12) & 0xFFFFF) as u32;
    }

    // JALR, L(B|H|W), ADDI, SLTI, SLTIU, XORI, ORI, ANDI
    if opmap == 0b01100111 || opmap == 0b00000011 || opmap == 0b00010011 {
        return ((inst >> 20) & 0xFFF) as u32;
    }

    // S(B|H|W)
    if opmap == 0b00100011 {
        return (((inst >> 25) & 0x1F) << 5 + (inst >> 7) & 0x1F) as u32;
    }

    // B(EQ|NE|LT|GE|LTU|GEU)
    if opmap == 0b01100011 {
        return (((inst >> 27) & 0x1) << 11 + ((inst >> 7) & 0x1) << 10 +
                ((inst >> 25) & 0x1F) << 4 + (inst >> 8) & 0xF) as u32;
    }

    return 0;
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
