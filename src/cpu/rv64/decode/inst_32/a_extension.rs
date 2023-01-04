use crate::cpu::instruction::OpecodeKind;
use crate::cpu::rv64::decode::DecodeUtil;
use crate::cpu::TrapCause;

pub fn parse_opecode(inst: u64) -> Result<OpecodeKind, &'static str> {
    let opmap: u8 = inst.slice(6, 0) as u8;
    let funct7: u8 = inst.slice(31, 27) as u8;

    match opmap {
        0b0101111 => match funct7 {
            0b00010 => Ok(OpecodeKind::OP_LR_W),
            0b00011 => Ok(OpecodeKind::OP_SC_W),
            0b00001 => Ok(OpecodeKind::OP_AMOSWAP_W),
            0b00000 => Ok(OpecodeKind::OP_AMOADD_W),
            0b00100 => Ok(OpecodeKind::OP_AMOXOR_W),
            0b01100 => Ok(OpecodeKind::OP_AMOAND_W),
            0b01000 => Ok(OpecodeKind::OP_AMOOR_W),
            0b10000 => Ok(OpecodeKind::OP_AMOMIN_W),
            0b10100 => Ok(OpecodeKind::OP_AMOMAX_W),
            0b11000 => Ok(OpecodeKind::OP_AMOMINU_W),
            0b11100 => Ok(OpecodeKind::OP_AMOMAXU_W),
            _ => Err("opecode decoding failed"),
        },
        _ => Err("opecode decoding failed"),
    }
}

pub fn parse_rd(
    inst: u64,
    opkind: &OpecodeKind,
) -> Result<Option<usize>, (Option<u64>, TrapCause, String)> {
    let rd: usize = inst.slice(11, 7) as usize;

    match opkind {
        OpecodeKind::OP_LR_W => Ok(Some(rd)),
        OpecodeKind::OP_SC_W => Ok(Some(rd)),
        OpecodeKind::OP_AMOSWAP_W => Ok(Some(rd)),
        OpecodeKind::OP_AMOADD_W => Ok(Some(rd)),
        OpecodeKind::OP_AMOXOR_W => Ok(Some(rd)),
        OpecodeKind::OP_AMOAND_W => Ok(Some(rd)),
        OpecodeKind::OP_AMOOR_W => Ok(Some(rd)),
        OpecodeKind::OP_AMOMIN_W => Ok(Some(rd)),
        OpecodeKind::OP_AMOMAX_W => Ok(Some(rd)),
        OpecodeKind::OP_AMOMINU_W => Ok(Some(rd)),
        OpecodeKind::OP_AMOMAXU_W => Ok(Some(rd)),
        _ => panic!("rd decoding failed in A extension"),
    }
}

pub fn parse_rs1(
    inst: u64,
    opkind: &OpecodeKind,
) -> Result<Option<usize>, (Option<u64>, TrapCause, String)> {
    let rs1: usize = inst.slice(19, 15) as usize;

    match opkind {
        OpecodeKind::OP_LR_W => Ok(Some(rs1)),
        OpecodeKind::OP_SC_W => Ok(Some(rs1)),
        OpecodeKind::OP_AMOSWAP_W => Ok(Some(rs1)),
        OpecodeKind::OP_AMOADD_W => Ok(Some(rs1)),
        OpecodeKind::OP_AMOXOR_W => Ok(Some(rs1)),
        OpecodeKind::OP_AMOAND_W => Ok(Some(rs1)),
        OpecodeKind::OP_AMOOR_W => Ok(Some(rs1)),
        OpecodeKind::OP_AMOMIN_W => Ok(Some(rs1)),
        OpecodeKind::OP_AMOMAX_W => Ok(Some(rs1)),
        OpecodeKind::OP_AMOMINU_W => Ok(Some(rs1)),
        OpecodeKind::OP_AMOMAXU_W => Ok(Some(rs1)),
        _ => panic!("rs1 decoding failed in A extension"),
    }
}

pub fn parse_rs2(
    inst: u64,
    opkind: &OpecodeKind,
) -> Result<Option<usize>, (Option<u64>, TrapCause, String)> {
    let rs2: usize = inst.slice(24, 20) as usize;

    match opkind {
        OpecodeKind::OP_SC_W => Ok(Some(rs2)),
        OpecodeKind::OP_AMOSWAP_W => Ok(Some(rs2)),
        OpecodeKind::OP_AMOADD_W => Ok(Some(rs2)),
        OpecodeKind::OP_AMOXOR_W => Ok(Some(rs2)),
        OpecodeKind::OP_AMOAND_W => Ok(Some(rs2)),
        OpecodeKind::OP_AMOOR_W => Ok(Some(rs2)),
        OpecodeKind::OP_AMOMIN_W => Ok(Some(rs2)),
        OpecodeKind::OP_AMOMAX_W => Ok(Some(rs2)),
        OpecodeKind::OP_AMOMINU_W => Ok(Some(rs2)),
        OpecodeKind::OP_AMOMAXU_W => Ok(Some(rs2)),
        _ => Ok(None),
    }
}

#[allow(non_snake_case)]
pub fn parse_imm(
    inst: u64,
    opkind: &OpecodeKind,
) -> Result<Option<i32>, (Option<u64>, TrapCause, String)> {
    let aq_and_rl = || inst.slice(26, 25) as i32;

    match opkind {
        OpecodeKind::OP_LR_W => Ok(Some(aq_and_rl())),
        OpecodeKind::OP_SC_W => Ok(Some(aq_and_rl())),
        OpecodeKind::OP_AMOSWAP_W => Ok(Some(aq_and_rl())),
        OpecodeKind::OP_AMOADD_W => Ok(Some(aq_and_rl())),
        OpecodeKind::OP_AMOXOR_W => Ok(Some(aq_and_rl())),
        OpecodeKind::OP_AMOAND_W => Ok(Some(aq_and_rl())),
        OpecodeKind::OP_AMOOR_W => Ok(Some(aq_and_rl())),
        OpecodeKind::OP_AMOMIN_W => Ok(Some(aq_and_rl())),
        OpecodeKind::OP_AMOMAX_W => Ok(Some(aq_and_rl())),
        OpecodeKind::OP_AMOMINU_W => Ok(Some(aq_and_rl())),
        OpecodeKind::OP_AMOMAXU_W => Ok(Some(aq_and_rl())),
        _ => panic!("imm decoding failed in A extension"),
    }
}
