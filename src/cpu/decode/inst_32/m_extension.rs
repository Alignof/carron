use crate::cpu::decode::DecodeUtil;
use crate::cpu::instruction::OpecodeKind;
use crate::cpu::TrapCause;

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

pub fn parse_rd(inst: u32, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<u32>, TrapCause, String)> {
    let rd: usize = inst.slice(11, 7) as usize;

    match opkind {
        OpecodeKind::OP_MUL		=> Ok(Some(rd)),
        OpecodeKind::OP_MULH	=> Ok(Some(rd)),
        OpecodeKind::OP_MULHSU	=> Ok(Some(rd)),
        OpecodeKind::OP_MULHU	=> Ok(Some(rd)),
        OpecodeKind::OP_DIV		=> Ok(Some(rd)),
        OpecodeKind::OP_DIVU	=> Ok(Some(rd)),
        OpecodeKind::OP_REM		=> Ok(Some(rd)),
        OpecodeKind::OP_REMU	=> Ok(Some(rd)),
        _ => Ok(None),
    }
}

pub fn parse_rs1(inst: u32, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<u32>, TrapCause, String)> {
    let rs1: usize = inst.slice(19, 15) as usize;

    match opkind {
        OpecodeKind::OP_MUL		=> Ok(Some(rs1)),
        OpecodeKind::OP_MULH	=> Ok(Some(rs1)),
        OpecodeKind::OP_MULHSU	=> Ok(Some(rs1)),
        OpecodeKind::OP_MULHU	=> Ok(Some(rs1)),
        OpecodeKind::OP_DIV		=> Ok(Some(rs1)),
        OpecodeKind::OP_DIVU	=> Ok(Some(rs1)),
        OpecodeKind::OP_REM		=> Ok(Some(rs1)),
        OpecodeKind::OP_REMU	=> Ok(Some(rs1)),
        _ => Ok(None),
    }
}

pub fn parse_rs2(inst: u32, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<u32>, TrapCause, String)> {
    let rs2: usize = inst.slice(24, 20) as usize;

    match opkind {
        OpecodeKind::OP_MUL		=> Ok(Some(rs2)),
        OpecodeKind::OP_MULH	=> Ok(Some(rs2)),
        OpecodeKind::OP_MULHSU	=> Ok(Some(rs2)),
        OpecodeKind::OP_MULHU	=> Ok(Some(rs2)),
        OpecodeKind::OP_DIV		=> Ok(Some(rs2)),
        OpecodeKind::OP_DIVU	=> Ok(Some(rs2)),
        OpecodeKind::OP_REM		=> Ok(Some(rs2)),
        OpecodeKind::OP_REMU	=> Ok(Some(rs2)),
        _ => Ok(None),
    }
}

#[allow(non_snake_case)]
pub fn parse_imm(_inst: u32, _opkind: &OpecodeKind) -> Result<Option<i32>, (Option<u32>, TrapCause, String)> {
    Ok(None)
}

