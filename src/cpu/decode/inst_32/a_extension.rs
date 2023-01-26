use crate::cpu::decode::{only_rv64, DecodeUtil};
use crate::cpu::instruction::OpecodeKind;
use crate::cpu::{Isa, TrapCause};

pub fn parse_opecode(inst: u32, isa: Isa) -> Result<OpecodeKind, (Option<u64>, TrapCause, String)> {
    let opmap: u8 = inst.slice(6, 0) as u8;
    let funct3: u8 = inst.slice(14, 12) as u8;
    let funct7: u8 = inst.slice(31, 27) as u8;
    let illegal_inst_exception = || {
        Err((
            Some(u64::from(inst)),
            TrapCause::IllegalInst,
            format!("opecode decoding failed, {inst:b}"),
        ))
    };

    match opmap {
        0b0101111 => match funct3 {
            0b010 => match funct7 {
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
                _ => illegal_inst_exception(),
            },
            0b011 => match funct7 {
                0b00010 => only_rv64(OpecodeKind::OP_LR_D, isa),
                0b00011 => only_rv64(OpecodeKind::OP_SC_D, isa),
                0b00001 => only_rv64(OpecodeKind::OP_AMOSWAP_D, isa),
                0b00000 => only_rv64(OpecodeKind::OP_AMOADD_D, isa),
                0b00100 => only_rv64(OpecodeKind::OP_AMOXOR_D, isa),
                0b01100 => only_rv64(OpecodeKind::OP_AMOAND_D, isa),
                0b01000 => only_rv64(OpecodeKind::OP_AMOOR_D, isa),
                0b10000 => only_rv64(OpecodeKind::OP_AMOMIN_D, isa),
                0b10100 => only_rv64(OpecodeKind::OP_AMOMAX_D, isa),
                0b11000 => only_rv64(OpecodeKind::OP_AMOMINU_D, isa),
                0b11100 => only_rv64(OpecodeKind::OP_AMOMAXU_D, isa),
                _ => illegal_inst_exception(),
            },
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
        OpecodeKind::OP_LR_D => Ok(Some(rd)),
        OpecodeKind::OP_SC_D => Ok(Some(rd)),
        OpecodeKind::OP_AMOSWAP_D => Ok(Some(rd)),
        OpecodeKind::OP_AMOADD_D => Ok(Some(rd)),
        OpecodeKind::OP_AMOXOR_D => Ok(Some(rd)),
        OpecodeKind::OP_AMOAND_D => Ok(Some(rd)),
        OpecodeKind::OP_AMOOR_D => Ok(Some(rd)),
        OpecodeKind::OP_AMOMIN_D => Ok(Some(rd)),
        OpecodeKind::OP_AMOMAX_D => Ok(Some(rd)),
        OpecodeKind::OP_AMOMINU_D => Ok(Some(rd)),
        OpecodeKind::OP_AMOMAXU_D => Ok(Some(rd)),
        _ => panic!("rd decoding failed in A extension"),
    }
}

pub fn parse_rs1(
    inst: u32,
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
        OpecodeKind::OP_LR_D => Ok(Some(rs1)),
        OpecodeKind::OP_SC_D => Ok(Some(rs1)),
        OpecodeKind::OP_AMOSWAP_D => Ok(Some(rs1)),
        OpecodeKind::OP_AMOADD_D => Ok(Some(rs1)),
        OpecodeKind::OP_AMOXOR_D => Ok(Some(rs1)),
        OpecodeKind::OP_AMOAND_D => Ok(Some(rs1)),
        OpecodeKind::OP_AMOOR_D => Ok(Some(rs1)),
        OpecodeKind::OP_AMOMIN_D => Ok(Some(rs1)),
        OpecodeKind::OP_AMOMAX_D => Ok(Some(rs1)),
        OpecodeKind::OP_AMOMINU_D => Ok(Some(rs1)),
        OpecodeKind::OP_AMOMAXU_D => Ok(Some(rs1)),
        _ => panic!("rs1 decoding failed in A extension"),
    }
}

pub fn parse_rs2(
    inst: u32,
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
        OpecodeKind::OP_SC_D => Ok(Some(rs2)),
        OpecodeKind::OP_AMOSWAP_D => Ok(Some(rs2)),
        OpecodeKind::OP_AMOADD_D => Ok(Some(rs2)),
        OpecodeKind::OP_AMOXOR_D => Ok(Some(rs2)),
        OpecodeKind::OP_AMOAND_D => Ok(Some(rs2)),
        OpecodeKind::OP_AMOOR_D => Ok(Some(rs2)),
        OpecodeKind::OP_AMOMIN_D => Ok(Some(rs2)),
        OpecodeKind::OP_AMOMAX_D => Ok(Some(rs2)),
        OpecodeKind::OP_AMOMINU_D => Ok(Some(rs2)),
        OpecodeKind::OP_AMOMAXU_D => Ok(Some(rs2)),
        _ => Ok(None),
    }
}

#[allow(non_snake_case)]
pub fn parse_imm(
    inst: u32,
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
        OpecodeKind::OP_LR_D => Ok(Some(aq_and_rl())),
        OpecodeKind::OP_SC_D => Ok(Some(aq_and_rl())),
        OpecodeKind::OP_AMOSWAP_D => Ok(Some(aq_and_rl())),
        OpecodeKind::OP_AMOADD_D => Ok(Some(aq_and_rl())),
        OpecodeKind::OP_AMOXOR_D => Ok(Some(aq_and_rl())),
        OpecodeKind::OP_AMOAND_D => Ok(Some(aq_and_rl())),
        OpecodeKind::OP_AMOOR_D => Ok(Some(aq_and_rl())),
        OpecodeKind::OP_AMOMIN_D => Ok(Some(aq_and_rl())),
        OpecodeKind::OP_AMOMAX_D => Ok(Some(aq_and_rl())),
        OpecodeKind::OP_AMOMINU_D => Ok(Some(aq_and_rl())),
        OpecodeKind::OP_AMOMAXU_D => Ok(Some(aq_and_rl())),
        _ => panic!("imm decoding failed in A extension"),
    }
}
