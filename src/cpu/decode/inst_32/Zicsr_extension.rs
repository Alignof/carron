use crate::cpu::decode::DecodeUtil;
use crate::cpu::instruction::OpecodeKind;

pub fn parse_opecode(inst: u32) -> Result<OpecodeKind, &'static str> {
    let opmap: u8  = inst.slice(6, 0) as u8;
    let funct3: u8 = inst.slice(14, 12) as u8;

    match opmap {
        0b1110011 => match funct3 {
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

    match opkind {
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
        OpecodeKind::OP_CSRRW	=> Some(rs1),
        OpecodeKind::OP_CSRRS	=> Some(rs1),
        OpecodeKind::OP_CSRRC	=> Some(rs1),
        OpecodeKind::OP_CSRRWI	=> Some(rs1),
        OpecodeKind::OP_CSRRSI	=> Some(rs1),
        OpecodeKind::OP_CSRRCI	=> Some(rs1),
        _ => None,
    }
}

pub fn parse_rs2(inst: u32, opkind: &OpecodeKind) -> Option<usize> {
    let csr: usize = inst.slice(31, 20) as usize;

    match opkind {
        OpecodeKind::OP_CSRRW	=> Some(csr),
        OpecodeKind::OP_CSRRS	=> Some(csr),
        OpecodeKind::OP_CSRRC	=> Some(csr),
        OpecodeKind::OP_CSRRWI	=> Some(csr),
        OpecodeKind::OP_CSRRSI	=> Some(csr),
        OpecodeKind::OP_CSRRCI	=> Some(csr),
        _ => None,
    }
}

pub fn parse_imm(inst: u32, _opkind: &OpecodeKind) -> Option<i32> {
    None
}

