use crate::cpu::decode::DecodeUtil;
use crate::cpu::instruction::OpecodeKind;

pub fn parse_opecode(inst: u32) -> Result<OpecodeKind, &'static str> {
    let opmap: u8  = inst.slice(6, 0) as u8;
    let funct3: u8 = inst.slice(14, 12) as u8;
    let funct5: u8 = inst.slice(24, 20) as u8;
    let funct7: u8 = inst.slice(31, 25) as u8;

    match opmap {
        0b0110111 => Ok(OpecodeKind::OP_LUI),
        0b0010111 => Ok(OpecodeKind::OP_AUIPC),
        0b1101111 => Ok(OpecodeKind::OP_JAL),
        0b1100111 => Ok(OpecodeKind::OP_JALR),
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
            0b101 => if (inst >> 30) & 0x1 == 0x0 {
                Ok(OpecodeKind::OP_SRLI)
            } else {
                Ok(OpecodeKind::OP_SRAI)
            },
            0b110 => Ok(OpecodeKind::OP_ORI),
            0b111 => Ok(OpecodeKind::OP_ANDI),
            _     => Err("opecode decoding failed"),
        }, 0b0110011 => match funct3 {
            0b000 => if (inst >> 30) & 0x1 == 0x0 {
                Ok(OpecodeKind::OP_ADD)
            } else {
                Ok(OpecodeKind::OP_SUB)
            },
            0b001 => Ok(OpecodeKind::OP_SLL),
            0b010 => Ok(OpecodeKind::OP_SLT),
            0b011 => Ok(OpecodeKind::OP_SLTU),
            0b100 => Ok(OpecodeKind::OP_XOR),
            0b101 => if (inst >> 30) & 0x1 == 0x0 {
                Ok(OpecodeKind::OP_SRL)
            } else {
                Ok(OpecodeKind::OP_SRA)
            },
            0b110 => Ok(OpecodeKind::OP_OR),
            0b111 => Ok(OpecodeKind::OP_AND),
            _     => Err("opecode decoding failed"),
        },
        0b0001111 => Ok(OpecodeKind::OP_FENCE),
        0b1110011 => match funct3 {
            0b000 => match funct7 {
                0b0000000 => match funct5 {
                    0b00000 => Ok(OpecodeKind::OP_ECALL),
                    0b00001 => Ok(OpecodeKind::OP_EBREAK),
                    _ => Err("opecode decoding failed"),
                }
                0b0001000 => Ok(OpecodeKind::OP_SRET),
                0b0011000 => Ok(OpecodeKind::OP_MRET),
                0b0001001 => Ok(OpecodeKind::OP_SFENCE_VMA),
                _ => Err("opecode decoding failed"),
            },
            0b001 => Ok(OpecodeKind::OP_CSRRW),
            0b010 => Ok(OpecodeKind::OP_CSRRS),
            0b011 => Ok(OpecodeKind::OP_CSRRC),
            0b101 => Ok(OpecodeKind::OP_CSRRWI),
            0b110 => Ok(OpecodeKind::OP_CSRRSI),
            0b111 => Ok(OpecodeKind::OP_CSRRCI),
            _     => Err("opecode decoding failed"),
        },
        _         => Err("opecode decoding failed"),
    }
}

pub fn parse_rd(inst: u32, opkind: &OpecodeKind) -> Option<usize> {
    let rd: usize = inst.slice(11, 7) as usize;

    // B(EQ|NE|LT|GE|LTU|GEU), S(B|H|W), ECALL, EBREAK
    match opkind {
        OpecodeKind::OP_LUI		=> Some(rd),
        OpecodeKind::OP_AUIPC	=> Some(rd),
        OpecodeKind::OP_JAL		=> Some(rd),
        OpecodeKind::OP_JALR	=> Some(rd),
        OpecodeKind::OP_LB		=> Some(rd),
        OpecodeKind::OP_LH		=> Some(rd),
        OpecodeKind::OP_LW		=> Some(rd),
        OpecodeKind::OP_LBU		=> Some(rd),
        OpecodeKind::OP_LHU		=> Some(rd),
        OpecodeKind::OP_ADDI	=> Some(rd),
        OpecodeKind::OP_SLTI	=> Some(rd),
        OpecodeKind::OP_SLTIU	=> Some(rd),
        OpecodeKind::OP_XORI	=> Some(rd),
        OpecodeKind::OP_ORI		=> Some(rd),
        OpecodeKind::OP_ANDI	=> Some(rd),
        OpecodeKind::OP_SLLI	=> Some(rd),
        OpecodeKind::OP_SRLI	=> Some(rd),
        OpecodeKind::OP_SRAI	=> Some(rd),
        OpecodeKind::OP_ADD		=> Some(rd),
        OpecodeKind::OP_SUB		=> Some(rd),
        OpecodeKind::OP_SLL		=> Some(rd),
        OpecodeKind::OP_SLT		=> Some(rd),
        OpecodeKind::OP_SLTU	=> Some(rd),
        OpecodeKind::OP_XOR		=> Some(rd),
        OpecodeKind::OP_SRL		=> Some(rd),
        OpecodeKind::OP_SRA		=> Some(rd),
        OpecodeKind::OP_OR		=> Some(rd),
        OpecodeKind::OP_AND		=> Some(rd),
        OpecodeKind::OP_CSRRW	=> Some(rd),
        OpecodeKind::OP_CSRRS	=> Some(rd),
        OpecodeKind::OP_CSRRC	=> Some(rd),
        OpecodeKind::OP_CSRRWI	=> Some(rd),
        OpecodeKind::OP_CSRRSI	=> Some(rd),
        OpecodeKind::OP_CSRRCI	=> Some(rd),
        _ => None,
    }
}

pub fn parse_rs1(inst: u32, opkind: &OpecodeKind) -> Option<usize> {
    let rs1: usize = inst.slice(19, 15) as usize;

    // LUI, AUIPC, JAL, FENCE, ECALL, EBREAK
    match opkind {
        OpecodeKind::OP_JALR	=> Some(rs1),
        OpecodeKind::OP_BEQ		=> Some(rs1),
        OpecodeKind::OP_BNE		=> Some(rs1),
        OpecodeKind::OP_BLT		=> Some(rs1),
        OpecodeKind::OP_BGE		=> Some(rs1),
        OpecodeKind::OP_BLTU	=> Some(rs1),
        OpecodeKind::OP_BGEU	=> Some(rs1),
        OpecodeKind::OP_LB		=> Some(rs1),
        OpecodeKind::OP_LH		=> Some(rs1),
        OpecodeKind::OP_LW		=> Some(rs1),
        OpecodeKind::OP_LBU		=> Some(rs1),
        OpecodeKind::OP_LHU		=> Some(rs1),
        OpecodeKind::OP_SB		=> Some(rs1),
        OpecodeKind::OP_SH		=> Some(rs1),
        OpecodeKind::OP_SW		=> Some(rs1),
        OpecodeKind::OP_ADDI	=> Some(rs1),
        OpecodeKind::OP_SLTI	=> Some(rs1),
        OpecodeKind::OP_SLTIU	=> Some(rs1),
        OpecodeKind::OP_XORI	=> Some(rs1),
        OpecodeKind::OP_ORI		=> Some(rs1),
        OpecodeKind::OP_ANDI	=> Some(rs1),
        OpecodeKind::OP_SLLI	=> Some(rs1),
        OpecodeKind::OP_SRLI	=> Some(rs1),
        OpecodeKind::OP_SRAI	=> Some(rs1),
        OpecodeKind::OP_ADD		=> Some(rs1),
        OpecodeKind::OP_SUB		=> Some(rs1),
        OpecodeKind::OP_SLL		=> Some(rs1),
        OpecodeKind::OP_SLT		=> Some(rs1),
        OpecodeKind::OP_SLTU	=> Some(rs1),
        OpecodeKind::OP_XOR		=> Some(rs1),
        OpecodeKind::OP_SRL		=> Some(rs1),
        OpecodeKind::OP_SRA		=> Some(rs1),
        OpecodeKind::OP_OR		=> Some(rs1),
        OpecodeKind::OP_AND		=> Some(rs1),
        OpecodeKind::OP_CSRRW	=> Some(rs1),
        OpecodeKind::OP_CSRRS	=> Some(rs1),
        OpecodeKind::OP_CSRRC	=> Some(rs1),
        OpecodeKind::OP_CSRRWI	=> Some(rs1),
        OpecodeKind::OP_CSRRSI	=> Some(rs1),
        OpecodeKind::OP_CSRRCI	=> Some(rs1),
        OpecodeKind::OP_SFENCE_VMA	=> Some(rs1),
        _ => None,
    }
}

pub fn parse_rs2(inst: u32, opkind: &OpecodeKind) -> Option<usize> {
    let rs2: usize = inst.slice(24, 20) as usize;
    let csr: usize = inst.slice(31, 20) as usize;

    // LUI, AUIPC, JAL, JALR L(B|H|W|BU|HU),
    // ADDI, SLTI, SLTIU, XORI, ORI, ANDI, SLLI,
    // FENCE, ECALL, EBREAK
    match opkind {
        OpecodeKind::OP_BEQ		=> Some(rs2),
        OpecodeKind::OP_BNE		=> Some(rs2),
        OpecodeKind::OP_BLT		=> Some(rs2),
        OpecodeKind::OP_BGE		=> Some(rs2),
        OpecodeKind::OP_BLTU	=> Some(rs2),
        OpecodeKind::OP_BGEU	=> Some(rs2),
        OpecodeKind::OP_SB		=> Some(rs2),
        OpecodeKind::OP_SH		=> Some(rs2),
        OpecodeKind::OP_SW		=> Some(rs2),
        OpecodeKind::OP_ADD		=> Some(rs2),
        OpecodeKind::OP_SUB		=> Some(rs2),
        OpecodeKind::OP_SLL		=> Some(rs2),
        OpecodeKind::OP_SLT		=> Some(rs2),
        OpecodeKind::OP_SLTU	=> Some(rs2),
        OpecodeKind::OP_XOR		=> Some(rs2),
        OpecodeKind::OP_SRL		=> Some(rs2),
        OpecodeKind::OP_SRA		=> Some(rs2),
        OpecodeKind::OP_OR		=> Some(rs2),
        OpecodeKind::OP_AND		=> Some(rs2),
        OpecodeKind::OP_SFENCE_VMA	=> Some(rs2),
        OpecodeKind::OP_CSRRW	=> Some(csr),
        OpecodeKind::OP_CSRRS	=> Some(csr),
        OpecodeKind::OP_CSRRC	=> Some(csr),
        OpecodeKind::OP_CSRRWI	=> Some(csr),
        OpecodeKind::OP_CSRRSI	=> Some(csr),
        OpecodeKind::OP_CSRRCI	=> Some(csr),
        _ => None,
    }
}

#[allow(non_snake_case)]
pub fn parse_imm(inst: u32, opkind: &OpecodeKind) -> Option<i32> {
    let U_type = | | {
        (inst.slice(31, 12) << 12) as i32
    };
    let I_type = | | {
        let imm32 = inst.slice(31, 20) as i32;
        inst.to_signed_nbit(imm32, 12)
    };
    let S_type = | | {
        let imm32 = (inst.slice(11, 7).set(&[4,3,2,1,0]) | inst.slice(31, 25).set(&[11,10,9,8,7,6,5])) as i32;
        inst.to_signed_nbit(imm32, 12)
    };
    let B_type = | | {
        let imm32 = (inst.slice(11, 7).set(&[4,3,2,1,11]) | inst.slice(31, 25).set(&[12,10,9,8,7,6,5])) as i32;
        inst.to_signed_nbit(imm32, 13)
    };
    let J_type = | | {
        let imm32 = inst.slice(31, 12)
            .set(&[20,10,9,8,7,6,5,4,3,2,1,11,19,18,17,16,15,14,13,12]) as i32;
        inst.to_signed_nbit(imm32, 20)
    };

    match opkind {
        OpecodeKind::OP_LUI		=> Some(U_type()),
        OpecodeKind::OP_AUIPC	=> Some(U_type()),
        OpecodeKind::OP_JAL		=> Some(J_type()),
        OpecodeKind::OP_JALR	=> Some(I_type()),
        OpecodeKind::OP_BEQ		=> Some(B_type()),
        OpecodeKind::OP_BNE		=> Some(B_type()),
        OpecodeKind::OP_BLT		=> Some(B_type()),
        OpecodeKind::OP_BGE		=> Some(B_type()),
        OpecodeKind::OP_BLTU	=> Some(B_type()),
        OpecodeKind::OP_BGEU	=> Some(B_type()),
        OpecodeKind::OP_LB		=> Some(I_type()),
        OpecodeKind::OP_LH		=> Some(I_type()),
        OpecodeKind::OP_LW		=> Some(I_type()),
        OpecodeKind::OP_LBU		=> Some(I_type()),
        OpecodeKind::OP_LHU		=> Some(I_type()),
        OpecodeKind::OP_SB		=> Some(S_type()),
        OpecodeKind::OP_SH		=> Some(S_type()),
        OpecodeKind::OP_SW		=> Some(S_type()),
        OpecodeKind::OP_ADDI	=> Some(I_type()),
        OpecodeKind::OP_SLTI	=> Some(I_type()),
        OpecodeKind::OP_SLTIU	=> Some(I_type()),
        OpecodeKind::OP_XORI	=> Some(I_type()),
        OpecodeKind::OP_ORI		=> Some(I_type()),
        OpecodeKind::OP_ANDI	=> Some(I_type()),
        OpecodeKind::OP_SLLI	=> Some(I_type()),
        OpecodeKind::OP_SRLI	=> Some(I_type()),
        OpecodeKind::OP_SRAI	=> Some(I_type()),
        _ => None,
    }
}

