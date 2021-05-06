use super::{OpecodeKind, Instruction, Decode};

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

    match opmap {
        0b000 => Ok(OpecodeKind::OP_C_ADDI),
        0b001 => Ok(OpecodeKind::OP_C_JAL),
        0b010 => Ok(OpecodeKind::OP_C_LI),
        0b011 => Ok(OpecodeKind::OP_C_ADDI16SP),
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
        let new_rd:  u8  = self.parse_rd(&new_opc);
        let new_rs1: u8  = self.parse_rs1(&new_opc);
        let new_rs2: u8  = self.parse_rs2(&new_opc);
        let new_imm: u32 = self.parse_imm(&new_opc);

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

    fn parse_rd(&self, opkind: &OpecodeKind) -> u8 {
        let inst: &u16 = self;
        let q0_rd: u8  = ((inst >> 2) & 0x7) as u8;
        let q1_rd: u8  = ((inst >> 7) & 0x7) as u8;
        let q2_rd: u8  = ((inst >> 7) & 0x1F) as u8;

        match opkind {
            // Quadrant 0
            OpecodeKind::OP_C_ADDI4SPN  => q0_rd,
            OpecodeKind::OP_C_FLD       => q0_rd,
            OpecodeKind::OP_C_LW        => q0_rd,
            OpecodeKind::OP_C_FLW       => q0_rd,
            // Quadrant 1
            OpecodeKind::OP_C_SRLI	    => q1_rd,
            OpecodeKind::OP_C_SRAI	    => q1_rd,
            OpecodeKind::OP_C_ANDI	    => q1_rd,
            OpecodeKind::OP_C_SUB	    => q1_rd,
            OpecodeKind::OP_C_XOR	    => q1_rd,
            OpecodeKind::OP_C_OR	    => q1_rd,
            OpecodeKind::OP_C_AND	    => q1_rd,
            // Quadrant 2
            OpecodeKind::OP_C_SLLI	    => q2_rd,
            OpecodeKind::OP_C_FLDSP	    => q2_rd,
            OpecodeKind::OP_C_LWSP	    => q2_rd,
            OpecodeKind::OP_C_FLWSP	    => q2_rd,
            OpecodeKind::OP_C_JR	    => q2_rd,
            OpecodeKind::OP_C_MV	    => q2_rd,
            OpecodeKind::OP_C_EBREAK   	=> q2_rd,
            OpecodeKind::OP_C_JALR	    => q2_rd,
            OpecodeKind::OP_C_ADD	    => q2_rd,
            _ => 0,
        }
    }

    fn parse_rs1(&self, opkind: &OpecodeKind) -> u8 {
        let inst: &u16 = self;
        let q0_rs1: u8 = ((inst >> 7) & 0x3) as u8;
        let q1_rs1: u8 = ((inst >> 7) & 0x3) as u8;
        let q2_rs1: u8 = ((inst >> 7) & 0x3) as u8;
        let addi_rs1: u8 = ((inst >> 7) & 0x1F) as u8;

        match opkind {
            // Quadrant 0
            OpecodeKind::OP_C_FLD       => q0_rs1,
            OpecodeKind::OP_C_LW        => q0_rs1,
            OpecodeKind::OP_C_FLW       => q0_rs1,
            OpecodeKind::OP_C_FSD       => q0_rs1,
            OpecodeKind::OP_C_SW        => q0_rs1,
            OpecodeKind::OP_C_FSW       => q0_rs1,
            // Quadrant 1
            OpecodeKind::OP_C_ADDI		=> addi_rs1,
            OpecodeKind::OP_C_ADDI16SP	=> addi_rs1,
            OpecodeKind::OP_C_SRLI		=> q1_rs1,
            OpecodeKind::OP_C_SRAI		=> q1_rs1,
            OpecodeKind::OP_C_ANDI		=> q1_rs1,
            OpecodeKind::OP_C_SUB		=> q1_rs1,
            OpecodeKind::OP_C_XOR		=> q1_rs1,
            OpecodeKind::OP_C_OR		=> q1_rs1,
            OpecodeKind::OP_C_AND		=> q1_rs1,
            OpecodeKind::OP_C_BEQZ		=> q1_rs1,
            OpecodeKind::OP_C_BNEZ		=> q1_rs1,
            // Quadrant 2
            OpecodeKind::OP_C_SLLI	    => q2_rs1,
            OpecodeKind::OP_C_JR	    => q2_rs1,
            OpecodeKind::OP_C_JALR	    => q2_rs1,
            OpecodeKind::OP_C_ADD	    => q2_rs1,
            _ => 0,
        }
    }

    fn parse_rs2(&self, opkind: &OpecodeKind) -> u8 {
        let inst: &u16 = self;
        let q0_rs2: u8 = ((inst >> 2) & 0x7) as u8;
        let q1_rs2: u8 = ((inst >> 2) & 0x7) as u8;
        let q2_rs2: u8 = ((inst >> 2) & 0x1F) as u8;

        match opkind {
            // Quadrant 0
            OpecodeKind::OP_C_FSD   => q0_rs2,
            OpecodeKind::OP_C_SW    => q0_rs2,
            OpecodeKind::OP_C_FSW   => q0_rs2,
            // Quadrant 1
            OpecodeKind::OP_C_SUB	=> q1_rs2,
            OpecodeKind::OP_C_XOR	=> q1_rs2,
            OpecodeKind::OP_C_OR	=> q1_rs2,
            OpecodeKind::OP_C_AND	=> q1_rs2,
            // Quadrant 2
            OpecodeKind::OP_C_MV	=> q2_rs2,
            OpecodeKind::OP_C_ADD	=> q2_rs2,
            OpecodeKind::OP_C_FSDSP	=> q2_rs2,
            OpecodeKind::OP_C_SWSP	=> q2_rs2,
            OpecodeKind::OP_C_FSWSP	=> q2_rs2,
            _ => 0,
        }
    }

    fn parse_imm(&self, opkind: &OpecodeKind) -> u32 {
        (match opkind {
            // Quadrant0
            OpecodeKind::OP_C_ADDI4SPN	=> (self >> 5) & 0xFF,
            OpecodeKind::OP_C_FLD	    => (self >> 5) & 0x3 + (((self >> 10) & 0x7) << 0x5),
            OpecodeKind::OP_C_LW	    => (self >> 5) & 0x3 + (((self >> 10) & 0x7) << 0x5),
            OpecodeKind::OP_C_FLW	    => (self >> 5) & 0x3 + (((self >> 10) & 0x7) << 0x5),
            OpecodeKind::OP_C_FSD	    => (self >> 5) & 0x3 + (((self >> 10) & 0x7) << 0x5),
            OpecodeKind::OP_C_SW	    => (self >> 5) & 0x3 + (((self >> 10) & 0x7) << 0x5),
            OpecodeKind::OP_C_FSW	    => (self >> 5) & 0x3 + (((self >> 10) & 0x7) << 0x5),
            // Quadrant1
            OpecodeKind::OP_C_NOP		=> (self >> 2) & 0x1F + (((self >> 12) & 0x1) << 0x2),
            OpecodeKind::OP_C_ADDI		=> (self >> 2) & 0x1F + (((self >> 12) & 0x1) << 0x2),
            OpecodeKind::OP_C_JAL		=> (self >> 2) & 0x7FF,
            OpecodeKind::OP_C_LI		=> (self >> 2) & 0x1F + (((self >> 12) & 0x1) << 0x2),
            OpecodeKind::OP_C_ADDI16SP	=> (self >> 2) & 0x1F + (((self >> 12) & 0x1) << 0x2),
            OpecodeKind::OP_C_LUI		=> (self >> 2) & 0x1F + (((self >> 12) & 0x1) << 0x2),
            OpecodeKind::OP_C_SRLI		=> (self >> 2) & 0x1F + (((self >> 12) & 0x1) << 0x2),
            OpecodeKind::OP_C_SRAI		=> (self >> 2) & 0x1F + (((self >> 12) & 0x1) << 0x2),
            OpecodeKind::OP_C_ANDI		=> (self >> 2) & 0x1F + (((self >> 12) & 0x1) << 0x2),
            OpecodeKind::OP_C_J		    => (self >> 2) & 0x7FF,
            OpecodeKind::OP_C_BEQZ		=> (self >> 2) & 0x1F + (((self >> 10) & 0x7) << 0x2),
            OpecodeKind::OP_C_BNEZ		=> (self >> 2) & 0x1F + (((self >> 10) & 0x7) << 0x2),
            // Quadrant2
            OpecodeKind::OP_C_SLLI		=> (self >> 2) & 0x1F + (((self >> 12) & 0x1) << 0x2),
            OpecodeKind::OP_C_FLDSP		=> (self >> 2) & 0x1F + (((self >> 12) & 0x1) << 0x2),
            OpecodeKind::OP_C_LWSP		=> (self >> 2) & 0x1F + (((self >> 12) & 0x1) << 0x2),
            OpecodeKind::OP_C_FLWSP		=> (self >> 2) & 0x1F + (((self >> 12) & 0x1) << 0x2),
            OpecodeKind::OP_C_JR		=> (self >> 2) & 0x1F + (((self >> 12) & 0x1) << 0x2),
            OpecodeKind::OP_C_FSDSP		=> (self >> 7) & 0x3F,
            OpecodeKind::OP_C_SWSP		=> (self >> 7) & 0x3F,
            OpecodeKind::OP_C_FSWSP		=> (self >> 7) & 0x3F,
            _ => 0,
        }) as u32
    }
}

