use crate::cpu::decode::DecodeUtil;
use crate::cpu::instruction::OpecodeKind;
use crate::cpu::TrapCause;

pub fn parse_opecode(inst: u32) -> Result<OpecodeKind, &'static str> {
    let _opmap: u8  = inst.slice(6, 0) as u8;
    let _funct3: u8 = inst.slice(14, 12) as u8;
    let funct7: u8 = inst.slice(31, 25) as u8;

    match inst {
        0b000100000010000000000000011100011 => Ok(OpecodeKind::OP_SRET),
        0b001100000010000000000000011100011 => Ok(OpecodeKind::OP_MRET),
        0b000100000101000000000000011100011 => Ok(OpecodeKind::OP_WFI),
        _ => {
            match funct7 {
                0b0001001 => Ok(OpecodeKind::OP_SFENCE_VMA),
                _ => Err("opecode decoding failed"),
            }
        },
    }
}

pub fn parse_rd(_inst: u32, _opkind: &OpecodeKind) -> Result<Option<usize>, (Option<i32>, TrapCause, String)> {
    Ok(None)
}

pub fn parse_rs1(inst: u32, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<i32>, TrapCause, String)> {
    let rs1: usize = inst.slice(19, 15) as usize;

    match opkind {
        OpecodeKind::OP_SFENCE_VMA	=> Ok(Some(rs1)),
        _ => Ok(None),
    }
}

pub fn parse_rs2(inst: u32, opkind: &OpecodeKind) -> Result<Option<usize>, (Option<i32>, TrapCause, String)> {
    let rs2: usize = inst.slice(24, 20) as usize;

    match opkind {
        OpecodeKind::OP_SFENCE_VMA	=> Ok(Some(rs2)),
        _ => Ok(None),
    }
}

pub fn parse_imm(_inst: u32, _opkind: &OpecodeKind) -> Result<Option<i32>, (Option<i32>, TrapCause, String)> {
    Ok(None)
}

