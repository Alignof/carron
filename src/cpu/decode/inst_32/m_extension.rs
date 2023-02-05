use crate::cpu::decode::{only_rv64, DecodeUtil};
use crate::cpu::instruction::OpecodeKind;
use crate::cpu::{Isa, TrapCause};

pub fn parse_opecode(inst: u32, isa: Isa) -> Result<OpecodeKind, (Option<u64>, TrapCause, String)> {
    let opmap: u8 = inst.slice(6, 0) as u8;
    let funct3: u8 = inst.slice(14, 12) as u8;
    let illegal_inst_exception = || {
        Err((
            Some(u64::from(inst)),
            TrapCause::IllegalInst,
            format!("opecode decoding failed in m extension, {inst:b}"),
        ))
    };

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
            _ => illegal_inst_exception(),
        },
        0b0111011 => match funct3 {
            0b000 => only_rv64(OpecodeKind::OP_MULW, isa),
            0b100 => only_rv64(OpecodeKind::OP_DIVW, isa),
            0b101 => only_rv64(OpecodeKind::OP_DIVUW, isa),
            0b110 => only_rv64(OpecodeKind::OP_REMW, isa),
            0b111 => only_rv64(OpecodeKind::OP_REMUW, isa),
            _ => illegal_inst_exception(),
        },
        _ => illegal_inst_exception(),
    }
}

pub fn parse_rd(
    inst: u32,
    opkind: &OpecodeKind,
) -> Result<Option<usize>, (Option<u64>, TrapCause, String)> {
    let rd: usize = inst.slice(11, 7) as usize;

    match opkind {
        OpecodeKind::OP_MUL => Ok(Some(rd)),
        OpecodeKind::OP_MULH => Ok(Some(rd)),
        OpecodeKind::OP_MULHSU => Ok(Some(rd)),
        OpecodeKind::OP_MULHU => Ok(Some(rd)),
        OpecodeKind::OP_DIV => Ok(Some(rd)),
        OpecodeKind::OP_DIVU => Ok(Some(rd)),
        OpecodeKind::OP_REM => Ok(Some(rd)),
        OpecodeKind::OP_REMU => Ok(Some(rd)),
        OpecodeKind::OP_MULW => Ok(Some(rd)),
        OpecodeKind::OP_DIVW => Ok(Some(rd)),
        OpecodeKind::OP_DIVUW => Ok(Some(rd)),
        OpecodeKind::OP_REMW => Ok(Some(rd)),
        OpecodeKind::OP_REMUW => Ok(Some(rd)),
        _ => Ok(None),
    }
}

pub fn parse_rs1(
    inst: u32,
    opkind: &OpecodeKind,
) -> Result<Option<usize>, (Option<u64>, TrapCause, String)> {
    let rs1: usize = inst.slice(19, 15) as usize;

    match opkind {
        OpecodeKind::OP_MUL => Ok(Some(rs1)),
        OpecodeKind::OP_MULH => Ok(Some(rs1)),
        OpecodeKind::OP_MULHSU => Ok(Some(rs1)),
        OpecodeKind::OP_MULHU => Ok(Some(rs1)),
        OpecodeKind::OP_DIV => Ok(Some(rs1)),
        OpecodeKind::OP_DIVU => Ok(Some(rs1)),
        OpecodeKind::OP_REM => Ok(Some(rs1)),
        OpecodeKind::OP_REMU => Ok(Some(rs1)),
        OpecodeKind::OP_MULW => Ok(Some(rs1)),
        OpecodeKind::OP_DIVW => Ok(Some(rs1)),
        OpecodeKind::OP_DIVUW => Ok(Some(rs1)),
        OpecodeKind::OP_REMW => Ok(Some(rs1)),
        OpecodeKind::OP_REMUW => Ok(Some(rs1)),
        _ => Ok(None),
    }
}

pub fn parse_rs2(
    inst: u32,
    opkind: &OpecodeKind,
) -> Result<Option<usize>, (Option<u64>, TrapCause, String)> {
    let rs2: usize = inst.slice(24, 20) as usize;

    match opkind {
        OpecodeKind::OP_MUL => Ok(Some(rs2)),
        OpecodeKind::OP_MULH => Ok(Some(rs2)),
        OpecodeKind::OP_MULHSU => Ok(Some(rs2)),
        OpecodeKind::OP_MULHU => Ok(Some(rs2)),
        OpecodeKind::OP_DIV => Ok(Some(rs2)),
        OpecodeKind::OP_DIVU => Ok(Some(rs2)),
        OpecodeKind::OP_REM => Ok(Some(rs2)),
        OpecodeKind::OP_REMU => Ok(Some(rs2)),
        OpecodeKind::OP_MULW => Ok(Some(rs2)),
        OpecodeKind::OP_DIVW => Ok(Some(rs2)),
        OpecodeKind::OP_DIVUW => Ok(Some(rs2)),
        OpecodeKind::OP_REMW => Ok(Some(rs2)),
        OpecodeKind::OP_REMUW => Ok(Some(rs2)),
        _ => Ok(None),
    }
}

#[allow(non_snake_case)]
pub fn parse_imm(
    _inst: u32,
    _opkind: &OpecodeKind,
) -> Result<Option<i32>, (Option<u64>, TrapCause, String)> {
    Ok(None)
}
