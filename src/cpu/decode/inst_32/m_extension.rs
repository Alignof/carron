use crate::cpu::decode::DecodeUtil;
use crate::cpu::instruction::OpecodeKind;

pub fn parse_opecode(inst: u32) -> Result<OpecodeKind, &'static str> {
    let opmap: u8  = inst.slice(6, 0) as u8;
    let funct3: u8 = inst.slice(14, 12) as u8;

    match opmap {
        0b0110011 => match funct3 {
            0b000 => Ok(OpecodeKind::OP_MUL),
            0b001 => Ok(OpecodeKind::OP_MULH),
            0b010 => Ok(OpecodeKind::OP_MULHSU),
            0b011 => Ok(OpecodeKind::OP_MULHU),
            0b100 => Ok(OpecodeKind::OP_DIV),
            0b101 => Ok(OpecodeKind::OP_DIVU),
            0b110 => Ok(OpecodeKind::OP_REM),
            0b111 => Ok(OpecodeKind::OP_REMU),
            _ => Err("opecode decoding failed"),
        },
        _ => Err("opecode decoding failed"),
    }
}

pub fn parse_rd(inst: u32, opkind: &OpecodeKind) -> Option<usize> {
    let rd: usize = inst.slice(11, 7) as usize;

    match opkind {
        OpecodeKind::OP_MUL		=> Some(rd),
        OpecodeKind::OP_MULH	=> Some(rd),
        OpecodeKind::OP_MULHSU	=> Some(rd),
        OpecodeKind::OP_MULHU	=> Some(rd),
        OpecodeKind::OP_DIV		=> Some(rd),
        OpecodeKind::OP_DIVU	=> Some(rd),
        OpecodeKind::OP_REM		=> Some(rd),
        OpecodeKind::OP_REMU	=> Some(rd),
        _ => None,
    }
}

pub fn parse_rs1(inst: u32, opkind: &OpecodeKind) -> Option<usize> {
    let rs1: usize = inst.slice(19, 15) as usize;

    match opkind {
        OpecodeKind::OP_MUL		=> Some(rs1),
        OpecodeKind::OP_MULH	=> Some(rs1),
        OpecodeKind::OP_MULHSU	=> Some(rs1),
        OpecodeKind::OP_MULHU	=> Some(rs1),
        OpecodeKind::OP_DIV		=> Some(rs1),
        OpecodeKind::OP_DIVU	=> Some(rs1),
        OpecodeKind::OP_REM		=> Some(rs1),
        OpecodeKind::OP_REMU	=> Some(rs1),
        _ => None,
    }
}

pub fn parse_rs2(inst: u32, opkind: &OpecodeKind) -> Option<usize> {
    let rs2: usize = inst.slice(24, 20) as usize;

    match opkind {
        OpecodeKind::OP_MUL		=> Some(rs2),
        OpecodeKind::OP_MULH	=> Some(rs2),
        OpecodeKind::OP_MULHSU	=> Some(rs2),
        OpecodeKind::OP_MULHU	=> Some(rs2),
        OpecodeKind::OP_DIV		=> Some(rs2),
        OpecodeKind::OP_DIVU	=> Some(rs2),
        OpecodeKind::OP_REM		=> Some(rs2),
        OpecodeKind::OP_REMU	=> Some(rs2),
        _ => None,
    }
}

#[allow(non_snake_case)]
pub fn parse_imm(_inst: u32, _opkind: &OpecodeKind) -> Option<i32> {
    None
}

