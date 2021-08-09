use super::Decode;
use crate::cpu::instruction::{OpecodeKind, Instruction};

fn quadrant0(opmap: &u8) -> Result<OpecodeKind, &'static str> {
    match opmap {
        0b000 => Ok(OpecodeKind::OP_C_ADDI4SPN),
        0b001 => Ok(OpecodeKind::OP_C_FLD),
        0b010 => Ok(OpecodeKind::OP_C_LW),
        0b011 => Ok(OpecodeKind::OP_C_FLW),
        0b100 => Ok(OpecodeKind::OP_C_FSD),
        0b110 => Ok(OpecodeKind::OP_C_SW),
        0b111 => Ok(OpecodeKind::OP_C_FSW),
        _     => Err("opecode decoding failed"),
    }
}

fn quadrant1(inst: &u16, opmap: &u8) -> Result<OpecodeKind, &'static str> {
    let sr_flag: u8 = ((inst >> 9) & 0x3) as u8;
    let lo_flag: u8 = ((inst >> 4) & 0x3) as u8;
    let mi_flag: u8 = ((inst >> 7) & 0x1F) as u8;

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
    let lo_flag: u8 = ((inst >> 2) & 0x1F) as u8;
    let mi_flag: u8 = ((inst >> 7) & 0x1F) as u8;
    let hi_flag: u8 = ((inst >> 12) & 0x1) as u8;

    match opmap {
        0b000 => Ok(OpecodeKind::OP_C_SLLI),
        0b001 => Ok(OpecodeKind::OP_C_FLDSP),
        0b010 => Ok(OpecodeKind::OP_C_LWSP),
        0b011 => Ok(OpecodeKind::OP_C_FLWSP),
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
        0b101 => Ok(OpecodeKind::OP_C_FSDSP),
        0b110 => Ok(OpecodeKind::OP_C_SWSP),
        0b111 => Ok(OpecodeKind::OP_C_FSWSP),
        _     => Err("opecode decoding failed"),
    }
}

impl Decode for u16 {
    fn decode(&self) -> Instruction {
        let new_opc: OpecodeKind = match self.parse_opecode(){
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
        let opmap: u8 = ((inst >> 13) & 0x7) as u8;
        let quadrant: u8  = (inst & 0x3) as u8;

        match quadrant {
            0b00 => quadrant0(&opmap),
            0b01 => quadrant1(inst, &opmap),
            0b10 => quadrant2(inst, &opmap),
            _    => Err("opecode decoding failed"),
        }
    }

    fn parse_rd(&self, opkind: &OpecodeKind) -> Option<usize> {
        let inst: &u16 = self;
        let q0_rd: usize  = ((inst >> 2) & 0x7) as usize;
        let q1_rd: usize  = ((inst >> 7) & 0x7) as usize;
        let q2_rd: usize  = ((inst >> 7) & 0x1F) as usize;

        match opkind {
            // Quadrant 0
            OpecodeKind::OP_C_ADDI4SPN  => Some(q0_rd),
            OpecodeKind::OP_C_FLD       => Some(q0_rd),
            OpecodeKind::OP_C_LW        => Some(q0_rd),
            OpecodeKind::OP_C_FLW       => Some(q0_rd),
            // Quadrant 1
            OpecodeKind::OP_C_SRLI        => Some(q1_rd),
            OpecodeKind::OP_C_SRAI        => Some(q1_rd),
            OpecodeKind::OP_C_ANDI        => Some(q1_rd),
            OpecodeKind::OP_C_SUB        => Some(q1_rd),
            OpecodeKind::OP_C_XOR        => Some(q1_rd),
            OpecodeKind::OP_C_OR        => Some(q1_rd),
            OpecodeKind::OP_C_AND        => Some(q1_rd),
            // Quadrant 2
            OpecodeKind::OP_C_SLLI        => Some(q2_rd),
            OpecodeKind::OP_C_FLDSP        => Some(q2_rd),
            OpecodeKind::OP_C_LWSP        => Some(q2_rd),
            OpecodeKind::OP_C_FLWSP        => Some(q2_rd),
            OpecodeKind::OP_C_JR        => Some(q2_rd),
            OpecodeKind::OP_C_MV        => Some(q2_rd),
            OpecodeKind::OP_C_EBREAK       => Some(q2_rd),
            OpecodeKind::OP_C_JALR        => Some(q2_rd),
            OpecodeKind::OP_C_ADD        => Some(q2_rd),
            _ => None,
        }
    }

    fn parse_rs1(&self, opkind: &OpecodeKind) -> Option<usize> {
        let inst: &u16 = self;
        let q0_rs1: usize = ((inst >> 7) & 0x3) as usize;
        let q1_rs1: usize = ((inst >> 7) & 0x3) as usize;
        let q2_rs1: usize = ((inst >> 7) & 0x3) as usize;
        let addi_rs1: usize = ((inst >> 7) & 0x1F) as usize;

        match opkind {
            // Quadrant 0
            OpecodeKind::OP_C_FLD       => Some(q0_rs1),
            OpecodeKind::OP_C_LW        => Some(q0_rs1),
            OpecodeKind::OP_C_FLW       => Some(q0_rs1),
            OpecodeKind::OP_C_FSD       => Some(q0_rs1),
            OpecodeKind::OP_C_SW        => Some(q0_rs1),
            OpecodeKind::OP_C_FSW       => Some(q0_rs1),
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
        let q0_rs2: usize = ((inst >> 2) & 0x7) as usize;
        let q1_rs2: usize = ((inst >> 2) & 0x7) as usize;
        let q2_rs2: usize = ((inst >> 2) & 0x1F) as usize;

        match opkind {
            // Quadrant 0
            OpecodeKind::OP_C_FSD   => Some(q0_rs2),
            OpecodeKind::OP_C_SW    => Some(q0_rs2),
            OpecodeKind::OP_C_FSW   => Some(q0_rs2),
            // Quadrant 1
            OpecodeKind::OP_C_SUB   => Some(q1_rs2),
            OpecodeKind::OP_C_XOR   => Some(q1_rs2),
            OpecodeKind::OP_C_OR    => Some(q1_rs2),
            OpecodeKind::OP_C_AND   => Some(q1_rs2),
            // Quadrant 2
            OpecodeKind::OP_C_MV    => Some(q2_rs2),
            OpecodeKind::OP_C_ADD   => Some(q2_rs2),
            OpecodeKind::OP_C_FSDSP => Some(q2_rs2),
            OpecodeKind::OP_C_SWSP  => Some(q2_rs2),
            OpecodeKind::OP_C_FSWSP => Some(q2_rs2),
            _ => None,
        }
    }

    fn parse_imm(&self, opkind: &OpecodeKind) -> Option<i32> {
        let q0_imm = ((self >> 5) & 0x3 + (((self >> 10) & 0x7) << 0x5)) as i32;
        let q1_imm = ((self >> 2) & 0x1F + (((self >> 12) & 0x1) << 0x2)) as i32;
        let q2_imm = ((self >> 2) & 0x1F + (((self >> 12) & 0x1) << 0x2)) as i32;
        match opkind {
            // Quadrant0
            OpecodeKind::OP_C_ADDI4SPN  => Some(((self >> 5) & 0xFF) as i32),
            OpecodeKind::OP_C_FLD       => Some(q0_imm),
            OpecodeKind::OP_C_LW        => Some(q0_imm),
            OpecodeKind::OP_C_FLW       => Some(q0_imm),
            OpecodeKind::OP_C_FSD       => Some(q0_imm),
            OpecodeKind::OP_C_SW        => Some(q0_imm),
            OpecodeKind::OP_C_FSW       => Some(q0_imm),
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
            OpecodeKind::OP_C_BEQZ      => Some(((self >> 2) & 0x1F + (((self >> 10) & 0x7) << 0x2)) as i32),
            OpecodeKind::OP_C_BNEZ      => Some(((self >> 2) & 0x1F + (((self >> 10) & 0x7) << 0x2)) as i32),
            // Quadrant2
            OpecodeKind::OP_C_SLLI      => Some(q2_imm),
            OpecodeKind::OP_C_FLDSP     => Some(q2_imm),
            OpecodeKind::OP_C_LWSP      => Some(q2_imm),
            OpecodeKind::OP_C_FLWSP     => Some(q2_imm),
            OpecodeKind::OP_C_JR        => Some(q2_imm),
            OpecodeKind::OP_C_FSDSP     => Some(((self >> 7) & 0x3F) as i32),
            OpecodeKind::OP_C_SWSP      => Some(((self >> 7) & 0x3F) as i32),
            OpecodeKind::OP_C_FSWSP     => Some(((self >> 7) & 0x3F) as i32),
            _ => None,
        }
    }
}
