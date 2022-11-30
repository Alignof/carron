use crate::cpu::decode::DecodeUtil;
use crate::cpu::instruction::OpecodeKind;
use crate::cpu::TrapCause;

pub fn parse_opecode(inst: u32) -> Result<OpecodeKind, &'static str> {
    let opmap: u8 = inst.slice(6, 0) as u8;
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
            _ => Err("opecode decoding failed"),
        },
        0b0000011 => match funct3 {
            0b000 => Ok(OpecodeKind::OP_LB),
            0b001 => Ok(OpecodeKind::OP_LH),
            0b010 => Ok(OpecodeKind::OP_LW),
            0b100 => Ok(OpecodeKind::OP_LBU),
            0b101 => Ok(OpecodeKind::OP_LHU),
            _ => Err("opecode decoding failed"),
        },
        0b0100011 => match funct3 {
            0b000 => Ok(OpecodeKind::OP_SB),
            0b001 => Ok(OpecodeKind::OP_SH),
            0b010 => Ok(OpecodeKind::OP_SW),
            _ => Err("opecode decoding failed"),
        },
        0b0010011 => match funct3 {
            0b000 => Ok(OpecodeKind::OP_ADDI),
            0b001 => Ok(OpecodeKind::OP_SLLI),
            0b010 => Ok(OpecodeKind::OP_SLTI),
            0b011 => Ok(OpecodeKind::OP_SLTIU),
            0b100 => Ok(OpecodeKind::OP_XORI),
            0b101 => {
                if (inst >> 30) & 0x1 == 0x0 {
                    Ok(OpecodeKind::OP_SRLI)
                } else {
                    Ok(OpecodeKind::OP_SRAI)
                }
            }
            0b110 => Ok(OpecodeKind::OP_ORI),
            0b111 => Ok(OpecodeKind::OP_ANDI),
            _ => Err("opecode decoding failed"),
        },
        0b0110011 => match funct3 {
            0b000 => {
                if (inst >> 30) & 0x1 == 0x0 {
                    Ok(OpecodeKind::OP_ADD)
                } else {
                    Ok(OpecodeKind::OP_SUB)
                }
            }
            0b001 => Ok(OpecodeKind::OP_SLL),
            0b010 => Ok(OpecodeKind::OP_SLT),
            0b011 => Ok(OpecodeKind::OP_SLTU),
            0b100 => Ok(OpecodeKind::OP_XOR),
            0b101 => {
                if (inst >> 30) & 0x1 == 0x0 {
                    Ok(OpecodeKind::OP_SRL)
                } else {
                    Ok(OpecodeKind::OP_SRA)
                }
            }
            0b110 => Ok(OpecodeKind::OP_OR),
            0b111 => Ok(OpecodeKind::OP_AND),
            _ => Err("opecode decoding failed"),
        },
        0b0001111 => Ok(OpecodeKind::OP_FENCE),
        0b1110011 => match funct3 {
            0b000 => match funct7 {
                0b0000000 => match funct5 {
                    0b00000 => Ok(OpecodeKind::OP_ECALL),
                    0b00001 => Ok(OpecodeKind::OP_EBREAK),
                    _ => Err("opecode decoding failed"),
                },
                _ => Err("opecode decoding failed"),
            },
            _ => Err("opecode decoding failed"),
        },
        _ => Err("opecode decoding failed"),
    }
}

pub fn parse_rd(
    inst: u32,
    opkind: &OpecodeKind,
) -> Result<Option<usize>, (Option<u32>, TrapCause, String)> {
    let rd: usize = inst.slice(11, 7) as usize;

    // B(EQ|NE|LT|GE|LTU|GEU), S(B|H|W), ECALL, EBREAK
    match opkind {
        OpecodeKind::OP_LUI => Ok(Some(rd)),
        OpecodeKind::OP_AUIPC => Ok(Some(rd)),
        OpecodeKind::OP_JAL => Ok(Some(rd)),
        OpecodeKind::OP_JALR => Ok(Some(rd)),
        OpecodeKind::OP_LB => Ok(Some(rd)),
        OpecodeKind::OP_LH => Ok(Some(rd)),
        OpecodeKind::OP_LW => Ok(Some(rd)),
        OpecodeKind::OP_LBU => Ok(Some(rd)),
        OpecodeKind::OP_LHU => Ok(Some(rd)),
        OpecodeKind::OP_ADDI => Ok(Some(rd)),
        OpecodeKind::OP_SLTI => Ok(Some(rd)),
        OpecodeKind::OP_SLTIU => Ok(Some(rd)),
        OpecodeKind::OP_XORI => Ok(Some(rd)),
        OpecodeKind::OP_ORI => Ok(Some(rd)),
        OpecodeKind::OP_ANDI => Ok(Some(rd)),
        OpecodeKind::OP_SLLI => Ok(Some(rd)),
        OpecodeKind::OP_SRLI => Ok(Some(rd)),
        OpecodeKind::OP_SRAI => Ok(Some(rd)),
        OpecodeKind::OP_ADD => Ok(Some(rd)),
        OpecodeKind::OP_SUB => Ok(Some(rd)),
        OpecodeKind::OP_SLL => Ok(Some(rd)),
        OpecodeKind::OP_SLT => Ok(Some(rd)),
        OpecodeKind::OP_SLTU => Ok(Some(rd)),
        OpecodeKind::OP_XOR => Ok(Some(rd)),
        OpecodeKind::OP_SRL => Ok(Some(rd)),
        OpecodeKind::OP_SRA => Ok(Some(rd)),
        OpecodeKind::OP_OR => Ok(Some(rd)),
        OpecodeKind::OP_AND => Ok(Some(rd)),
        _ => Ok(None),
    }
}

pub fn parse_rs1(
    inst: u32,
    opkind: &OpecodeKind,
) -> Result<Option<usize>, (Option<u32>, TrapCause, String)> {
    let rs1: usize = inst.slice(19, 15) as usize;

    // LUI, AUIPC, JAL, FENCE, ECALL, EBREAK
    match opkind {
        OpecodeKind::OP_JALR => Ok(Some(rs1)),
        OpecodeKind::OP_BEQ => Ok(Some(rs1)),
        OpecodeKind::OP_BNE => Ok(Some(rs1)),
        OpecodeKind::OP_BLT => Ok(Some(rs1)),
        OpecodeKind::OP_BGE => Ok(Some(rs1)),
        OpecodeKind::OP_BLTU => Ok(Some(rs1)),
        OpecodeKind::OP_BGEU => Ok(Some(rs1)),
        OpecodeKind::OP_LB => Ok(Some(rs1)),
        OpecodeKind::OP_LH => Ok(Some(rs1)),
        OpecodeKind::OP_LW => Ok(Some(rs1)),
        OpecodeKind::OP_LBU => Ok(Some(rs1)),
        OpecodeKind::OP_LHU => Ok(Some(rs1)),
        OpecodeKind::OP_SB => Ok(Some(rs1)),
        OpecodeKind::OP_SH => Ok(Some(rs1)),
        OpecodeKind::OP_SW => Ok(Some(rs1)),
        OpecodeKind::OP_ADDI => Ok(Some(rs1)),
        OpecodeKind::OP_SLTI => Ok(Some(rs1)),
        OpecodeKind::OP_SLTIU => Ok(Some(rs1)),
        OpecodeKind::OP_XORI => Ok(Some(rs1)),
        OpecodeKind::OP_ORI => Ok(Some(rs1)),
        OpecodeKind::OP_ANDI => Ok(Some(rs1)),
        OpecodeKind::OP_SLLI => Ok(Some(rs1)),
        OpecodeKind::OP_SRLI => Ok(Some(rs1)),
        OpecodeKind::OP_SRAI => Ok(Some(rs1)),
        OpecodeKind::OP_ADD => Ok(Some(rs1)),
        OpecodeKind::OP_SUB => Ok(Some(rs1)),
        OpecodeKind::OP_SLL => Ok(Some(rs1)),
        OpecodeKind::OP_SLT => Ok(Some(rs1)),
        OpecodeKind::OP_SLTU => Ok(Some(rs1)),
        OpecodeKind::OP_XOR => Ok(Some(rs1)),
        OpecodeKind::OP_SRL => Ok(Some(rs1)),
        OpecodeKind::OP_SRA => Ok(Some(rs1)),
        OpecodeKind::OP_OR => Ok(Some(rs1)),
        OpecodeKind::OP_AND => Ok(Some(rs1)),
        _ => Ok(None),
    }
}

pub fn parse_rs2(
    inst: u32,
    opkind: &OpecodeKind,
) -> Result<Option<usize>, (Option<u32>, TrapCause, String)> {
    let rs2: usize = inst.slice(24, 20) as usize;

    // LUI, AUIPC, JAL, JALR L(B|H|W|BU|HU),
    // ADDI, SLTI, SLTIU, XORI, ORI, ANDI, SLLI,
    // FENCE, ECALL, EBREAK
    match opkind {
        OpecodeKind::OP_BEQ => Ok(Some(rs2)),
        OpecodeKind::OP_BNE => Ok(Some(rs2)),
        OpecodeKind::OP_BLT => Ok(Some(rs2)),
        OpecodeKind::OP_BGE => Ok(Some(rs2)),
        OpecodeKind::OP_BLTU => Ok(Some(rs2)),
        OpecodeKind::OP_BGEU => Ok(Some(rs2)),
        OpecodeKind::OP_SB => Ok(Some(rs2)),
        OpecodeKind::OP_SH => Ok(Some(rs2)),
        OpecodeKind::OP_SW => Ok(Some(rs2)),
        OpecodeKind::OP_ADD => Ok(Some(rs2)),
        OpecodeKind::OP_SUB => Ok(Some(rs2)),
        OpecodeKind::OP_SLL => Ok(Some(rs2)),
        OpecodeKind::OP_SLT => Ok(Some(rs2)),
        OpecodeKind::OP_SLTU => Ok(Some(rs2)),
        OpecodeKind::OP_XOR => Ok(Some(rs2)),
        OpecodeKind::OP_SRL => Ok(Some(rs2)),
        OpecodeKind::OP_SRA => Ok(Some(rs2)),
        OpecodeKind::OP_OR => Ok(Some(rs2)),
        OpecodeKind::OP_AND => Ok(Some(rs2)),
        _ => Ok(None),
    }
}

#[allow(non_snake_case)]
pub fn parse_imm(
    inst: u32,
    opkind: &OpecodeKind,
) -> Result<Option<i32>, (Option<u32>, TrapCause, String)> {
    let U_type = || (inst.slice(31, 12) << 12) as i32;
    let I_type = || {
        let imm32 = inst.slice(31, 20) as i32;
        inst.to_signed_nbit(imm32, 12)
    };
    let S_type = || {
        let imm32 = (inst.slice(11, 7).set(&[4, 3, 2, 1, 0])
            | inst.slice(31, 25).set(&[11, 10, 9, 8, 7, 6, 5])) as i32;
        inst.to_signed_nbit(imm32, 12)
    };
    let B_type = || {
        let imm32 = (inst.slice(11, 7).set(&[4, 3, 2, 1, 11])
            | inst.slice(31, 25).set(&[12, 10, 9, 8, 7, 6, 5])) as i32;
        inst.to_signed_nbit(imm32, 13)
    };
    let J_type = || {
        let imm32 = inst.slice(31, 12).set(&[
            20, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 11, 19, 18, 17, 16, 15, 14, 13, 12,
        ]) as i32;
        inst.to_signed_nbit(imm32, 20)
    };

    match opkind {
        OpecodeKind::OP_LUI => Ok(Some(U_type())),
        OpecodeKind::OP_AUIPC => Ok(Some(U_type())),
        OpecodeKind::OP_JAL => Ok(Some(J_type())),
        OpecodeKind::OP_JALR => Ok(Some(I_type())),
        OpecodeKind::OP_BEQ => Ok(Some(B_type())),
        OpecodeKind::OP_BNE => Ok(Some(B_type())),
        OpecodeKind::OP_BLT => Ok(Some(B_type())),
        OpecodeKind::OP_BGE => Ok(Some(B_type())),
        OpecodeKind::OP_BLTU => Ok(Some(B_type())),
        OpecodeKind::OP_BGEU => Ok(Some(B_type())),
        OpecodeKind::OP_LB => Ok(Some(I_type())),
        OpecodeKind::OP_LH => Ok(Some(I_type())),
        OpecodeKind::OP_LW => Ok(Some(I_type())),
        OpecodeKind::OP_LBU => Ok(Some(I_type())),
        OpecodeKind::OP_LHU => Ok(Some(I_type())),
        OpecodeKind::OP_SB => Ok(Some(S_type())),
        OpecodeKind::OP_SH => Ok(Some(S_type())),
        OpecodeKind::OP_SW => Ok(Some(S_type())),
        OpecodeKind::OP_ADDI => Ok(Some(I_type())),
        OpecodeKind::OP_SLTI => Ok(Some(I_type())),
        OpecodeKind::OP_SLTIU => Ok(Some(I_type())),
        OpecodeKind::OP_XORI => Ok(Some(I_type())),
        OpecodeKind::OP_ORI => Ok(Some(I_type())),
        OpecodeKind::OP_ANDI => Ok(Some(I_type())),
        OpecodeKind::OP_SLLI | OpecodeKind::OP_SRLI => {
            if inst.slice(31, 25) == 0 {
                Ok(Some(I_type()))
            } else {
                Err((
                    None,
                    TrapCause::IllegalInst,
                    "invalid shamt[5] in srli or slli".to_string(),
                ))
            }
        }
        OpecodeKind::OP_SRAI => {
            if inst.slice(31, 25) == 0b0100000 {
                Ok(Some(I_type()))
            } else {
                Err((
                    None,
                    TrapCause::IllegalInst,
                    "invalid shamt[5] in srai".to_string(),
                ))
            }
        }
        _ => Ok(None),
    }
}
