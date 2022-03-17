use crate::cpu::decode::DecodeUtil;
use crate::cpu::instruction::OpecodeKind;

pub fn parse_opecode(inst: u32) -> Result<OpecodeKind, &'static str> {
    let opmap: u8  = inst.slice(6, 0) as u8;
    let funct3: u8 = inst.slice(14, 12) as u8;
    let funct7: u8 = inst.slice(31, 25) as u8;

    match opmap {
        0b1110011 => match funct3 {
            0b000 => match funct7 {
                0b0001000 => Ok(OpecodeKind::OP_SRET),
                0b0011000 => Ok(OpecodeKind::OP_MRET),
                0b0001001 => Ok(OpecodeKind::OP_SFENCE_VMA),
                _ => Err("opecode decoding failed"),
            },
            _     => Err("opecode decoding failed"),
        },
        _         => Err("opecode decoding failed"),
    }
}

pub fn parse_rd(_inst: u32, _opkind: &OpecodeKind) -> Option<usize> {
    None
}

pub fn parse_rs1(inst: u32, opkind: &OpecodeKind) -> Option<usize> {
    let rs1: usize = inst.slice(19, 15) as usize;

    match opkind {
        OpecodeKind::OP_SFENCE_VMA	=> Some(rs1),
        _ => None,
    }
}

pub fn parse_rs2(inst: u32, opkind: &OpecodeKind) -> Option<usize> {
    let rs2: usize = inst.slice(24, 20) as usize;

    match opkind {
        OpecodeKind::OP_SFENCE_VMA	=> Some(rs2),
        _ => None,
    }
}

pub fn parse_imm(_inst: u32, _opkind: &OpecodeKind) -> Option<i32> {
    None
}

