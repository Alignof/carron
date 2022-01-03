use super::{Decode, DecodeUtil};
use crate::cpu::instruction::{OpecodeKind, Instruction};

fn quadrant0(opmap: &u8) -> Result<OpecodeKind, &'static str> {
    match opmap {
        0b000 => Ok(OpecodeKind::OP_C_ADDI4SPN),
        0b010 => Ok(OpecodeKind::OP_C_LW),
        0b110 => Ok(OpecodeKind::OP_C_SW),
        _     => Err("opecode decoding failed"),
    }
}

fn quadrant1(inst: &u16, opmap: &u8) -> Result<OpecodeKind, &'static str> {
    let sr_flag: u8 = inst.cut(10, 11) as u8;
    let lo_flag: u8 = inst.cut(5, 6) as u8;
    let mi_flag: u8 = inst.cut(7, 11) as u8;

    match opmap {
        0b000 => match mi_flag {
            0b00000 => Ok(OpecodeKind::OP_C_NOP),
            _       => Ok(OpecodeKind::OP_C_ADDI),
        },
        0b001 => Ok(OpecodeKind::OP_C_JAL),
        0b010 => Ok(OpecodeKind::OP_C_LI),
        0b011 => match mi_flag {
            0b00010 => Ok(OpecodeKind::OP_C_ADDI16SP),
            _       => Ok(OpecodeKind::OP_C_LUI),
        },
        0b100 => match sr_flag {
            0b00 => Ok(OpecodeKind::OP_C_SRLI),
            0b01 => Ok(OpecodeKind::OP_C_SRAI),
            0b10 => Ok(OpecodeKind::OP_C_ANDI),
            0b11 => match lo_flag {
                0b00 => Ok(OpecodeKind::OP_C_SUB),
                0b01 => Ok(OpecodeKind::OP_C_XOR),
                0b10 => Ok(OpecodeKind::OP_C_OR),
                0b11 => Ok(OpecodeKind::OP_C_AND),
                _    => Err("opecode decoding failed"),
            },
            _    => Err("opecode decoding failed"),
        },
        0b101 => Ok(OpecodeKind::OP_C_J),
        0b110 => Ok(OpecodeKind::OP_C_BEQZ),
        0b111 => Ok(OpecodeKind::OP_C_BNEZ),
        _     => Err("opecode decoding failed"),
    }
}


fn quadrant2(inst: &u16, opmap: &u8) -> Result<OpecodeKind, &'static str> { 
    let lo_flag: u8 = inst.cut(2, 6) as u8;
    let mi_flag: u8 = inst.cut(7, 11) as u8;
    let hi_flag: u8 = inst.cut(12, 12) as u8;

    match opmap {
        0b000 => Ok(OpecodeKind::OP_C_SLLI),
        0b010 => Ok(OpecodeKind::OP_C_LWSP),
        0b100 => match hi_flag {
            0b0 => match lo_flag {
                0b0 => Ok(OpecodeKind::OP_C_JR),
                _   => Ok(OpecodeKind::OP_C_MV),
            }, 
            0b1 => match mi_flag {
                0b0 => Ok(OpecodeKind::OP_C_EBREAK),
                _   => match lo_flag {
                    0b0 => Ok(OpecodeKind::OP_C_JALR),
                    _   => Ok(OpecodeKind::OP_C_ADD),
                },
            },
        _   => Err("opecode decoding failed"),
        },
        0b110 => Ok(OpecodeKind::OP_C_SWSP),
        _     => Err("opecode decoding failed"),
    }
}

impl Decode for u16 {
    fn decode(&self) -> Instruction {
        let new_opc: OpecodeKind = match self.parse_opecode() {
            Ok(opc)  => opc,
            Err(msg) => panic!("{}, {:b}", msg, self),
        };
        let new_rd:  Option<usize>  = self.parse_rd(&new_opc);
        let new_rs1: Option<usize>  = self.parse_rs1(&new_opc);
        let new_rs2: Option<usize>  = self.parse_rs2(&new_opc);
        let new_imm: Option<i32> = self.parse_imm(&new_opc);

        Instruction {
            opc: new_opc,
            rd:  new_rd,
            rs1: new_rs1,
            rs2: new_rs2,
            imm: new_imm,
            is_compressed: true,
        }
    }

    fn parse_opecode(&self) -> Result<OpecodeKind, &'static str> {
        let inst: &u16 = self;
        let opmap: u8 = inst.cut(13, 15) as u8;
        let quadrant: u8  = inst.cut(0, 1) as u8;

        match quadrant {
            0b00 => quadrant0(&opmap),
            0b01 => quadrant1(inst, &opmap),
            0b10 => quadrant2(inst, &opmap),
            _    => Err("opecode decoding failed"),
        }
    }

    fn parse_rd(&self, opkind: &OpecodeKind) -> Option<usize> {
        let inst: &u16 = self;
        let q0_rd: usize  = inst.cut(2, 4) as usize;
        let q1_rd: usize  = inst.cut(7, 9) as usize;
        let q1_wide_rd: usize  = inst.cut(7, 11) as usize;
        let q2_rd: usize  = inst.cut(7, 11) as usize;

        match opkind {
            // Quadrant 0
            OpecodeKind::OP_C_ADDI4SPN  => Some(q0_rd),
            OpecodeKind::OP_C_LW        => Some(q0_rd),
            // Quadrant 1
            OpecodeKind::OP_C_SRLI      => Some(q1_rd),
            OpecodeKind::OP_C_SRAI      => Some(q1_rd),
            OpecodeKind::OP_C_ANDI      => Some(q1_rd),
            OpecodeKind::OP_C_SUB       => Some(q1_rd),
            OpecodeKind::OP_C_XOR       => Some(q1_rd),
            OpecodeKind::OP_C_OR        => Some(q1_rd),
            OpecodeKind::OP_C_AND       => Some(q1_rd),
            OpecodeKind::OP_C_LI        => Some(q1_wide_rd),
            OpecodeKind::OP_C_LUI       => Some(q1_wide_rd),
            OpecodeKind::OP_C_ADDI      => Some(q1_wide_rd),
            OpecodeKind::OP_C_LI        => Some(q1_wide_rd),
            OpecodeKind::OP_C_LUI       => Some(q1_wide_rd),
            // Quadrant 2
            OpecodeKind::OP_C_SLLI      => Some(q2_rd),
            OpecodeKind::OP_C_LWSP      => Some(q2_rd),
            OpecodeKind::OP_C_JR        => Some(q2_rd),
            OpecodeKind::OP_C_MV        => Some(q2_rd),
            OpecodeKind::OP_C_EBREAK    => Some(q2_rd),
            OpecodeKind::OP_C_JALR      => Some(q2_rd),
            OpecodeKind::OP_C_ADD       => Some(q2_rd),
            _ => None,
        }
    }

    fn parse_rs1(&self, opkind: &OpecodeKind) -> Option<usize> {
        let inst: &u16 = self;
        let q0_rs1: usize = inst.cut(7, 9) as usize;
        let q1_rs1: usize = inst.cut(7, 9) as usize;
        let q2_rs1: usize = inst.cut(7, 11) as usize;
        let addi_rs1: usize = inst.cut(7, 11) as usize;

        match opkind {
            // Quadrant 0
            OpecodeKind::OP_C_LW        => Some(q0_rs1),
            OpecodeKind::OP_C_SW        => Some(q0_rs1),
            // Quadrant 1
            OpecodeKind::OP_C_ADDI      => Some(addi_rs1),
            OpecodeKind::OP_C_ADDI16SP  => Some(addi_rs1),
            OpecodeKind::OP_C_SRLI      => Some(q1_rs1),
            OpecodeKind::OP_C_SRAI      => Some(q1_rs1),
            OpecodeKind::OP_C_ANDI      => Some(q1_rs1),
            OpecodeKind::OP_C_SUB       => Some(q1_rs1),
            OpecodeKind::OP_C_XOR       => Some(q1_rs1),
            OpecodeKind::OP_C_OR        => Some(q1_rs1),
            OpecodeKind::OP_C_AND       => Some(q1_rs1),
            OpecodeKind::OP_C_BEQZ      => Some(q1_rs1),
            OpecodeKind::OP_C_BNEZ      => Some(q1_rs1),
            // Quadrant 2
            OpecodeKind::OP_C_SLLI      => Some(q2_rs1),
            OpecodeKind::OP_C_JR        => Some(q2_rs1),
            OpecodeKind::OP_C_JALR      => Some(q2_rs1),
            OpecodeKind::OP_C_ADD       => Some(q2_rs1),
            _ => None,
        }
    }

    fn parse_rs2(&self, opkind: &OpecodeKind) -> Option<usize> {
        let inst: &u16 = self;
        let q0_rs2: usize = inst.cut(2, 4) as usize;
        let q1_rs2: usize = inst.cut(2, 4) as usize;
        let q2_rs2: usize = inst.cut(2, 6) as usize;

        match opkind {
            // Quadrant 0
            OpecodeKind::OP_C_SW    => Some(q0_rs2),
            // Quadrant 1
            OpecodeKind::OP_C_SUB   => Some(q1_rs2),
            OpecodeKind::OP_C_XOR   => Some(q1_rs2),
            OpecodeKind::OP_C_OR    => Some(q1_rs2),
            OpecodeKind::OP_C_AND   => Some(q1_rs2),
            // Quadrant 2
            OpecodeKind::OP_C_MV    => Some(q2_rs2),
            OpecodeKind::OP_C_ADD   => Some(q2_rs2),
            OpecodeKind::OP_C_SWSP  => Some(q2_rs2),
            _ => None,
        }
    }

    fn parse_imm(&self, opkind: &OpecodeKind) -> Option<i32> {
        let q0_uimm = ((((self >> 10) & 0x7) << 3) | (((self >> 6) & 0x1) << 2) | (((self >> 5) & 0x1) << 6)) as i32;
        let q0_nzuimm = ((((self >> 10) & 0x7) << 3) | (((self >> 6) & 0x1) << 2) | (((self >> 5) & 0x1) << 6)) as i32;
        let q1_imm = ((self >> 2) & (0x1F + (((self >> 12) & 0x1) << 0x2))) as i32;
        let q2_imm = ((self >> 2) & (0x1F + (((self >> 12) & 0x1) << 0x2))) as i32;
        match opkind {
            // Quadrant0
            OpecodeKind::OP_C_ADDI4SPN  => Some(((self >> 5) & 0xFF) as i32),
            OpecodeKind::OP_C_LW        => Some(q0_uimm),
            OpecodeKind::OP_C_SW        => Some(q0_uimm),
            // Quadrant1
            OpecodeKind::OP_C_NOP       => Some(q1_imm),
            OpecodeKind::OP_C_ADDI      => Some(q1_imm),
            OpecodeKind::OP_C_JAL       => Some(((self >> 2) & 0x7FF) as i32),
            OpecodeKind::OP_C_LI        => Some(q1_imm),
            OpecodeKind::OP_C_ADDI16SP  => Some(q1_imm),
            OpecodeKind::OP_C_LUI       => Some(q1_imm),
            OpecodeKind::OP_C_SRLI      => Some(q1_imm),
            OpecodeKind::OP_C_SRAI      => Some(q1_imm),
            OpecodeKind::OP_C_ANDI      => Some(q1_imm),
            OpecodeKind::OP_C_J         => Some(((self >> 2) & 0x7FF) as i32),
            OpecodeKind::OP_C_BEQZ      => Some(((self >> 2) & (0x1F + (((self >> 10) & 0x7) << 0x2))) as i32),
            OpecodeKind::OP_C_BNEZ      => Some(((self >> 2) & (0x1F + (((self >> 10) & 0x7) << 0x2))) as i32),
            // Quadrant2
            OpecodeKind::OP_C_SLLI      => Some(q2_imm),
            OpecodeKind::OP_C_LWSP      => Some(q2_imm),
            OpecodeKind::OP_C_JR        => Some(q2_imm),
            OpecodeKind::OP_C_SWSP      => Some(((self >> 7) & 0x3F) as i32),
            _ => None,
        }
    }
}

impl DecodeUtil for u16 {
    fn cut(&self, start: u32, end: u32) -> Self {
        (self >> start) & (2_u16.pow(end - start) - 1)
    }
}
