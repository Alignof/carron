use super::Decode;
use crate::cpu::instruction::{OpecodeKind, Instruction};

#[allow(non_snake_case)]
impl Decode for u32 {
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
            is_compressed: false,
        }
    }

    fn parse_opecode(&self) -> Result<OpecodeKind, &'static str> {
        let inst: &u32 = self;
        let opmap: u8  = (inst & 0x7F) as u8;
        let funct3: u8 = ((inst >> 12) & 0x7) as u8;

        match opmap {
            0b0110111 => Ok(OpecodeKind::OP_LUI),
            0b0010111 => Ok(OpecodeKind::OP_AUIPC),
            0b1101111 => Ok(OpecodeKind::OP_JAL),
            0b1100111 => Ok(OpecodeKind::OP_JALR),
            0b1100011 => match funct3 {
                0b000 => Ok(OpecodeKind::OP_BEQ),
                0b001 => Ok(OpecodeKind::OP_BNE),
                0b100 => Ok(OpecodeKind::OP_BLT),
                0b101 => Ok(OpecodeKind::OP_BGE),
                0b110 => Ok(OpecodeKind::OP_BLTU),
                0b111 => Ok(OpecodeKind::OP_BGEU),
                _     => Err("opecode decoding failed"),
            },
            0b0000011 => match funct3 {
                0b000 => Ok(OpecodeKind::OP_LB),
                0b001 => Ok(OpecodeKind::OP_LH),
                0b010 => Ok(OpecodeKind::OP_LW),
                0b100 => Ok(OpecodeKind::OP_LBU),
                0b101 => Ok(OpecodeKind::OP_LHU),
                _     => Err("opecode decoding failed"),
            },
            0b0100011 => match funct3 {
                0b000 => Ok(OpecodeKind::OP_SB),
                0b001 => Ok(OpecodeKind::OP_SH),
                0b010 => Ok(OpecodeKind::OP_SW),
                _     => Err("opecode decoding failed"),
            },
            0b0010011 => match funct3 {
                0b000 => Ok(OpecodeKind::OP_ADDI),
                0b001 => Ok(OpecodeKind::OP_SLLI),
                0b010 => Ok(OpecodeKind::OP_SLTI),
                0b011 => Ok(OpecodeKind::OP_SLTIU),
                0b100 => Ok(OpecodeKind::OP_XORI),
                0b101 => Ok(OpecodeKind::OP_SRLI),//OP_SRAI,
                0b110 => Ok(OpecodeKind::OP_ORI),
                0b111 => Ok(OpecodeKind::OP_ANDI),
                _     => Err("opecode decoding failed"),
            }, 0b0110011 => match funct3 {
                0b000 => Ok(OpecodeKind::OP_ADD),//OP_SUB,
                0b001 => Ok(OpecodeKind::OP_SLL),
                0b010 => Ok(OpecodeKind::OP_SLT),
                0b011 => Ok(OpecodeKind::OP_SLTU),
                0b100 => Ok(OpecodeKind::OP_XOR),
                0b101 => Ok(OpecodeKind::OP_SRL),//OP_SRA,
                0b110 => Ok(OpecodeKind::OP_OR),
                0b111 => Ok(OpecodeKind::OP_AND),
                _     => Err("opecode decoding failed"),
            },
            0b0001111 => Ok(OpecodeKind::OP_FENCE),
            0b1110011 => Ok(OpecodeKind::OP_ECALL),//OP_EBREAK,
            _         => Err("opecode decoding failed"),
        }
    }

    fn parse_rd(&self, _opkind: &OpecodeKind) -> Option<usize> {
        let inst:&u32 = self;
        let opmap: usize  = (inst & 0x7F) as usize;
        let rd: usize = ((inst >> 7) & 0x1F) as usize;

        // B(EQ|NE|LT|GE|LTU|GEU), S(B|H|W), ECALL, EBREAK
        if  opmap == 0b01100011 || opmap == 0b00100011 || 
            opmap == 0b01110011 { 
                return None;
        }

        return Some(rd);
    }

    fn parse_rs1(&self, _opkind: &OpecodeKind) -> Option<usize> {
        let inst:&u32 = self;
        let opmap: usize  = (inst & 0x7F) as usize;
        let rs1: usize = ((inst >> 15) & 0x1F) as usize;

        // LUI, AUIPC, JAL, FENCE, ECALL, EBREAK
        if  opmap == 0b01010111 || opmap == 0b00010111 || 
            opmap == 0b01101111 || opmap == 0b01110011 { 
                return None;
        }

        return Some(rs1);
    }

    fn parse_rs2(&self, _opkind: &OpecodeKind) -> Option<usize> {
        let inst:&u32 = self;
        let opmap: usize  = (inst & 0x7F) as usize;
        let rs2: usize = ((inst >> 20) & 0x1F) as usize;

        // LUI, AUIPC, JAL, JALR L(B|H|W|BU|HU),
        // ADDI, SLTI, SLTIU, XORI, ORI, ANDI, SLLI,
        // FENCE, ECALL, EBREAK
        if  opmap == 0b01010111 || opmap == 0b00010111 || opmap == 0b01101111 ||
            opmap == 0b01100111 || opmap == 0b00000011 || opmap == 0b00010011 || 
            opmap == 0b00001111 || opmap == 0b01110011 { 
                return None;
        }

        return Some(rs2);
    }

    fn parse_imm(&self, opkind: &OpecodeKind) -> Option<i32> {
        let inst: &u32 = self;
        let U_type: i32 = (((inst >> 12) & 0xFFFF) << 12) as i32;
        let I_type: i32 = ((inst >> 20) & 0xFFF) as i32;
        let S_type: i32 = ((((inst >> 25) & 0x1F) << 5) | ((inst >> 7) & 0x1F)) as i32;
        let B_type: i32 = ((((inst >> 27) & 0x1) << 11) | (((inst >> 7) & 0x1) << 10) |
                          (((inst >> 25) & 0x1F) << 4) | ((inst >> 8) & 0xF)) as i32;
        let JAL_imm: i32 = (((((inst >> 12) & 0xFF) << 12) | (((inst >> 20) & 0x1) << 13) |
                          ((inst >> 21) & 0x3FF << 14) | ((inst >> 31) & 0x1 << 32)) << 1) as i32;

        match opkind {
            OpecodeKind::OP_LUI		=> Some(U_type),
            OpecodeKind::OP_AUIPC	=> Some(U_type),
            OpecodeKind::OP_JAL		=> Some(JAL_imm),
            OpecodeKind::OP_JALR	=> Some(I_type),
            OpecodeKind::OP_BEQ		=> Some(B_type),
            OpecodeKind::OP_BNE		=> Some(B_type),
            OpecodeKind::OP_BLT		=> Some(B_type),
            OpecodeKind::OP_BGE		=> Some(B_type),
            OpecodeKind::OP_BLTU	=> Some(B_type),
            OpecodeKind::OP_BGEU	=> Some(B_type),
            OpecodeKind::OP_LB		=> Some(I_type),
            OpecodeKind::OP_LH		=> Some(I_type),
            OpecodeKind::OP_LW		=> Some(I_type),
            OpecodeKind::OP_LBU		=> Some(I_type),
            OpecodeKind::OP_LHU		=> Some(I_type),
            OpecodeKind::OP_SB		=> Some(S_type),
            OpecodeKind::OP_SH		=> Some(S_type),
            OpecodeKind::OP_SW		=> Some(S_type),
            OpecodeKind::OP_ADDI	=> Some(I_type),
            OpecodeKind::OP_SLTI	=> Some(I_type),
            OpecodeKind::OP_SLTIU	=> Some(I_type),
            OpecodeKind::OP_XORI	=> Some(I_type),
            OpecodeKind::OP_ORI		=> Some(I_type),
            OpecodeKind::OP_ANDI	=> Some(I_type),
            OpecodeKind::OP_SLLI	=> Some(I_type),
            OpecodeKind::OP_SRLI	=> Some(I_type),
            _ => None,
        }
    }
}


/*
match opkind {
    OpecodeKind::OP_LUI		=> Some(),
    OpecodeKind::OP_AUIPC	=> Some(),
    OpecodeKind::OP_JAL		=> Some(),
    OpecodeKind::OP_JALR	=> Some(),
    OpecodeKind::OP_BEQ		=> Some(),
    OpecodeKind::OP_BNE		=> Some(),
    OpecodeKind::OP_BLT		=> Some(),
    OpecodeKind::OP_BGE		=> Some(),
    OpecodeKind::OP_BLTU	=> Some(),
    OpecodeKind::OP_BGEU	=> Some(),
    OpecodeKind::OP_LB		=> Some(),
    OpecodeKind::OP_LH		=> Some(),
    OpecodeKind::OP_LW		=> Some(),
    OpecodeKind::OP_LBU		=> Some(),
    OpecodeKind::OP_LHU		=> Some(),
    OpecodeKind::OP_SB		=> Some(),
    OpecodeKind::OP_SH		=> Some(),
    OpecodeKind::OP_SW		=> Some(),
    OpecodeKind::OP_ADDI	=> Some(),
    OpecodeKind::OP_SLTI	=> Some(),
    OpecodeKind::OP_SLTIU	=> Some(),
    OpecodeKind::OP_XORI	=> Some(),
    OpecodeKind::OP_ORI		=> Some(),
    OpecodeKind::OP_ANDI	=> Some(),
    OpecodeKind::OP_SLLI	=> Some(),
    OpecodeKind::OP_SRLI	=> Some(),
    OpecodeKind::OP_ADD		=> Some(),
    OpecodeKind::OP_SUB		=> Some(),
    OpecodeKind::OP_SLL		=> Some(),
    OpecodeKind::OP_SLT		=> Some(),
    OpecodeKind::OP_SLTU	=> Some(),
    OpecodeKind::OP_XOR		=> Some(),
    OpecodeKind::OP_SRL		=> Some(),
    OpecodeKind::OP_SRA		=> Some(),
    OpecodeKind::OP_OR		=> Some(),
    OpecodeKind::OP_AND		=> Some(),
    OpecodeKind::OP_FENCE	=> Some(),
    OpecodeKind::OP_ECALL	=> Some(),
    OpecodeKind::OP_EBREAK	=> Some(),
    _ => None,
}
*/
