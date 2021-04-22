use super::{OpecodeKind};

fn quadrant0(inst: &u32, opmap: &u8) -> OpecodeKind {
    match opmap {
        0b000 => Ok(OpecodeKind::OP_C_ADDI4SPN),
        0b001 => Ok(OpecodeKind::OP_C_FLD),
        0b010 => Ok(OpecodeKind::OP_C_LW),
        0b011 => Ok(OpecodeKind::OP_C_FLW),
        0b100 => Ok(OpecodeKind::OP_C_FSD),
        0b110 => Ok(OpecodeKind::OP_C_SW),
        0b111 => Ok(OpecodeKind::OP_C_FSW),
    }
}

fn quadrant1(inst: &u32, opmap: &u8) -> OpecodeKind {
    let sr_flag: u8 = ((inst >> 9) & 0x3) as u8;
    let lo_flag: u8 = ((inst >> 4) & 0x3) as u8;

    match opmap {
        0b000 => Ok(OpecodeKind::OP_C_ADDI),
        0b001 => Ok(OpecodeKind::OP_C_JAL),
        0b010 => Ok(OpecodeKind::OP_C_LI),
        0b011 => Ok(OpecodeKind::OP_C_ADDI16SP),
        0b100 => match sr_flag {
            0b00 => Ok(OpecodeKind::OP_C_SRLI),
            0b01 => Ok(OpecodeKind::OP_C_SRAI),
            0b10 => Ok(OpecodeKind::OP_C_ANDI),
            0b11 => match lo_flag {
                0b00 => Ok(OpecodeKind::OP_C_SUB),
                0b01 => Ok(OpecodeKind::OP_C_XOR),
                0b10 => Ok(OpecodeKind::OP_C_OR),
                0b11 => Ok(OpecodeKind::OP_C_AND),
            },
        },
        0b110 => Ok(OpecodeKind::OP_C_BEQZ),
        0b111 => Ok(OpecodeKind::OP_C_BNEZ),
    }
}


fn quadrant2(inst: &u32, opmap: &u8) -> OpecodeKind {
    let lo_flag: u8 = ((inst >> 2) & 0x1F) as u8;
    let mi_flag: u8 = ((inst >> 7) & 0x1F) as u8;
    let hi_flag: u8 = ((inst >> 12) & 0x1) as u8;

    match opmap {
        0b000 => Ok(OpecodeKind::OP_C_SLLI),
        0b001 => Ok(OpecodeKind::OP_C_FLDSP),
        0b010 => Ok(OpecodeKind::OP_C_LWSP),
        0b011 => Ok(OpecodeKind::OP_C_FLWSP),
        0b100 => match hi_flag {
            0b0 => match lo_flag {
                0b0 => Ok(OpecodeKind::OP_C_JR),
                _   => Ok(OpecodeKind::OP_C_MV),
            }, 
            0b1 => match mi_flag {
                0b0 => Ok(OpecodeKind::OP_C_EBREAK),
                _   => match lo_flag {
                    0b0 => Ok(OpecodeKind::OP_C_JALR),
                    _   => Ok(OpecodeKind::OP_C_ADD),
                },
            },
        },
        0b101 => Ok(OpecodeKind::OP_C_FSDSP),
        0b110 => Ok(OpecodeKind::OP_C_SWSP),
        0b111 => Ok(OpecodeKind::OP_C_FSWSP),
    }
}

pub fn parse_opecode(inst:&u32) -> Result<OpecodeKind, &'static str> {
    let opmap: u8 = ((inst >> 12) & 0x7) as u8;
    let quadrant: u8  = (inst & 0x3) as u8;

    match quadrant {
        0b00 => quadrant0(inst, &opmap),
        0b01 => quadrant1(inst, &opmap),
        0b10 => quadrant2(inst, &opmap),
        _    => Err("opecode decoding failed"),
    }
}

pub fn parse_rd(inst: &u32, opkind: OpecodeKind) -> u8 {
    let q0_rd: u8 = ((inst >> 2) & 0x7) as u8;
    let q1_rd: u8 = ((inst >> 7) & 0x7) as u8;
    let q2_rd: u8 = ((inst >> 7) & 0x1F) as u8;

    match opkind {
        // Quadrant 0
        OpecodeKind::OP_C_ADDI4SPN  => q0_rd,
        OpecodeKind::OP_C_FLD       => q0_rd,
        OpecodeKind::OP_C_FW        => q0_rd,
        OpecodeKind::OP_C_FLW       => q0_rd,
        // Quadrant 1
        OpecodeKind::OP_C_SRLI	    => q1_rd,
        OpecodeKind::OP_C_SRAI	    => q1_rd,
        OpecodeKind::OP_C_ANDI	    => q1_rd,
        OpecodeKind::OP_C_SUB	    => q1_rd,
        OpecodeKind::OP_C_XOR	    => q1_rd,
        OpecodeKind::OP_C_OR	    => q1_rd,
        OpecodeKind::OP_C_AND	    => q1_rd,
        // Quadrant 2
        OpecodeKind::OP_C_SLLI	    => q2_rd,
        OpecodeKind::OP_C_FLDSP	    => q2_rd,
        OpecodeKind::OP_C_LWSP	    => q2_rd,
        OpecodeKind::OP_C_FLWSP	    => q2_rd,
        OpecodeKind::OP_C_JR	    => q2_rd,
        OpecodeKind::OP_C_MV	    => q2_rd,
        OpecodeKind::OP_C_EBREAK   	=> q2_rd,
        OpecodeKind::OP_C_JALR	    => q2_rd,
        OpecodeKind::OP_C_ADD	    => q2_rd,
        _ => 0,
    }
}

pub fn parse_rs1(inst: &u32, opkind:OpecodeKind) -> u8 {
    let q0_rs1: u8 = ((inst >> 7) & 0x3) as u8;
    let q1_rs1: u8 = ((inst >> 7) & 0x3) as u8;
    let q2_rs1: u8 = ((inst >> 7) & 0x3) as u8;
    let addi_rs1: u8 = ((inst >> 7) & 0x1F) as u8;

    match opkind {
        // Quadrant 0
        OpecodeKind::OP_C_FLD       => q0_rs1,
        OpecodeKind::OP_C_FW        => q0_rs1,
        OpecodeKind::OP_C_FLW       => q0_rs1,
        OpecodeKind::OP_C_FSD       => q0_rs1,
        OpecodeKind::OP_C_SW        => q0_rs1,
        OpecodeKind::OP_C_FSW       => q0_rs1,
        // Quadrant 1
        OpecodeKind::OP_C_ADDI		=> addi_rs1,
        OpecodeKind::OP_C_ADDI16SP	=> addi_rs1,
        OpecodeKind::OP_C_SRLI		=> q1_rs1,
        OpecodeKind::OP_C_SRAI		=> q1_rs1,
        OpecodeKind::OP_C_ANDI		=> q1_rs1,
        OpecodeKind::OP_C_SUB		=> q1_rs1,
        OpecodeKind::OP_C_XOR		=> q1_rs1,
        OpecodeKind::OP_C_OR		=> q1_rs1,
        OpecodeKind::OP_C_AND		=> q1_rs1,
        OpecodeKind::OP_C_BEQZ		=> q1_rs1,
        OpecodeKind::OP_C_BNEZ		=> q1_rs1,
        // Quadrant 2
        OpecodeKind::OP_C_SLLI	    => q2_rs1,
        OpecodeKind::OP_C_JR	    => q2_rs1,
        OpecodeKind::OP_C_JALR	    => q2_rs1,
        OpecodeKind::OP_C_ADD	    => q2_rs1,
        _ => 0,
    }
}

pub fn parse_rs2(inst: &u32) -> u8 {
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


pub fn parse_imm(inst: &u32) -> u32 {
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
