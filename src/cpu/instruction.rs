// riscv-spec-20191213-1.pdf page=130

#[derive(Debug)]
pub struct Instruction {
    pub opc: OpecodeKind,
    pub rd: Option<usize>,
    pub rs1: Option<usize>,
    pub rs2: Option<usize>,
    pub imm: Option<i32>,
}

pub enum Extensions {
    BaseI,
    M,
    A,
    C,
    Zicsr,
    Priv,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum OpecodeKind {
    //== Base Integer Instruction ==
    OP_LUI,
    OP_AUIPC,
    OP_JAL,
    OP_JALR,
    OP_BEQ,
    OP_BNE,
    OP_BLT,
    OP_BGE,
    OP_BLTU,
    OP_BGEU,
    OP_LB,
    OP_LH,
    OP_LW,
    OP_LBU,
    OP_LHU,
    OP_SB,
    OP_SH,
    OP_SW,
    OP_ADDI,
    OP_SLTI,
    OP_SLTIU,
    OP_XORI,
    OP_ORI,
    OP_ANDI,
    OP_SLLI,
    OP_SRLI,
    OP_SRAI,
    OP_ADD,
    OP_SUB,
    OP_SLL,
    OP_SLT,
    OP_SLTU,
    OP_XOR,
    OP_SRL,
    OP_SRA,
    OP_OR,
    OP_AND,
    OP_FENCE,
    OP_ECALL,
    OP_EBREAK,
    //-- rv64 --
    OP_LWU,
    OP_LD,
    OP_SD,
    OP_ADDIW,
    OP_SLLIW,
    OP_SRLIW,
    OP_SRAIW,
    OP_ADDW,
    OP_SUBW,
    OP_SLLW,
    OP_SRLW,
    OP_SRAW,

    //== Zicsr Extension ==
    OP_CSRRW,
    OP_CSRRS,
    OP_CSRRC,
    OP_CSRRWI,
    OP_CSRRSI,
    OP_CSRRCI,

    //== privileged Instruction ==
    OP_SRET,
    OP_MRET,
    OP_WFI,
    OP_SFENCE_VMA,

    //== M Extension ==
    OP_MUL,
    OP_MULH,
    OP_MULHSU,
    OP_MULHU,
    OP_DIV,
    OP_DIVU,
    OP_REM,
    OP_REMU,
    //-- rv64 --
    OP_MULW,
    OP_DIVW,
    OP_DIVUW,
    OP_REMW,
    OP_REMUW,

    //== A Extension ==
    OP_LR_W,
    OP_SC_W,
    OP_AMOSWAP_W,
    OP_AMOADD_W,
    OP_AMOXOR_W,
    OP_AMOAND_W,
    OP_AMOOR_W,
    OP_AMOMIN_W,
    OP_AMOMAX_W,
    OP_AMOMINU_W,
    OP_AMOMAXU_W,
    //-- rv64 --
    OP_LR_D,
    OP_SC_D,
    OP_AMOSWAP_D,
    OP_AMOADD_D,
    OP_AMOXOR_D,
    OP_AMOAND_D,
    OP_AMOOR_D,
    OP_AMOMIN_D,
    OP_AMOMAX_D,
    OP_AMOMINU_D,
    OP_AMOMAXU_D,

    //== C Extension ==
    OP_C_ADDI4SPN,
    OP_C_LW,
    OP_C_SW,
    OP_C_NOP,
    OP_C_ADDI,
    OP_C_JAL,
    OP_C_LI,
    OP_C_ADDI16SP,
    OP_C_LUI,
    OP_C_SRLI,
    OP_C_SRAI,
    OP_C_ANDI,
    OP_C_SUB,
    OP_C_XOR,
    OP_C_OR,
    OP_C_AND,
    OP_C_J,
    OP_C_BEQZ,
    OP_C_BNEZ,
    OP_C_SLLI,
    OP_C_LWSP,
    OP_C_JR,
    OP_C_MV,
    OP_C_EBREAK,
    OP_C_JALR,
    OP_C_ADD,
    OP_C_SWSP,
}

impl Instruction {
    pub fn print_myself(&self) {
        print!("{:<12}{:>4}", self.opc_to_string(), self.reg_to_string());
        if let Some(v) = self.rs1 {
            print!("{:>10},", v)
        } else {
            print!("          ,")
        }
        if let Some(v) = self.rs2 {
            print!("{:>10},", v)
        } else {
            print!("          ,")
        }
        if let Some(v) = self.imm {
            print!("{:>10},", v)
        } else {
            print!("          ,")
        }
    }

    pub fn reg_to_string(&self) -> &'static str {
        if let Some(rd_val) = self.rd {
            reg2str(rd_val)
        } else {
            "  "
        }
    }

    pub fn opc_to_extension(&self) -> Extensions {
        match self.opc {
            OpecodeKind::OP_LUI => Extensions::BaseI,
            OpecodeKind::OP_AUIPC => Extensions::BaseI,
            OpecodeKind::OP_JAL => Extensions::BaseI,
            OpecodeKind::OP_JALR => Extensions::BaseI,
            OpecodeKind::OP_BEQ => Extensions::BaseI,
            OpecodeKind::OP_BNE => Extensions::BaseI,
            OpecodeKind::OP_BLT => Extensions::BaseI,
            OpecodeKind::OP_BGE => Extensions::BaseI,
            OpecodeKind::OP_BLTU => Extensions::BaseI,
            OpecodeKind::OP_BGEU => Extensions::BaseI,
            OpecodeKind::OP_LB => Extensions::BaseI,
            OpecodeKind::OP_LH => Extensions::BaseI,
            OpecodeKind::OP_LW => Extensions::BaseI,
            OpecodeKind::OP_LBU => Extensions::BaseI,
            OpecodeKind::OP_LHU => Extensions::BaseI,
            OpecodeKind::OP_SB => Extensions::BaseI,
            OpecodeKind::OP_SH => Extensions::BaseI,
            OpecodeKind::OP_SW => Extensions::BaseI,
            OpecodeKind::OP_ADDI => Extensions::BaseI,
            OpecodeKind::OP_SLTI => Extensions::BaseI,
            OpecodeKind::OP_SLTIU => Extensions::BaseI,
            OpecodeKind::OP_XORI => Extensions::BaseI,
            OpecodeKind::OP_ORI => Extensions::BaseI,
            OpecodeKind::OP_ANDI => Extensions::BaseI,
            OpecodeKind::OP_SLLI => Extensions::BaseI,
            OpecodeKind::OP_SRLI => Extensions::BaseI,
            OpecodeKind::OP_SRAI => Extensions::BaseI,
            OpecodeKind::OP_ADD => Extensions::BaseI,
            OpecodeKind::OP_SUB => Extensions::BaseI,
            OpecodeKind::OP_SLL => Extensions::BaseI,
            OpecodeKind::OP_SLT => Extensions::BaseI,
            OpecodeKind::OP_SLTU => Extensions::BaseI,
            OpecodeKind::OP_XOR => Extensions::BaseI,
            OpecodeKind::OP_SRL => Extensions::BaseI,
            OpecodeKind::OP_SRA => Extensions::BaseI,
            OpecodeKind::OP_OR => Extensions::BaseI,
            OpecodeKind::OP_AND => Extensions::BaseI,
            OpecodeKind::OP_FENCE => Extensions::BaseI,
            OpecodeKind::OP_ECALL => Extensions::BaseI,
            OpecodeKind::OP_EBREAK => Extensions::BaseI,
            OpecodeKind::OP_LWU => Extensions::BaseI,
            OpecodeKind::OP_LD => Extensions::BaseI,
            OpecodeKind::OP_SD => Extensions::BaseI,
            OpecodeKind::OP_ADDIW => Extensions::BaseI,
            OpecodeKind::OP_SLLIW => Extensions::BaseI,
            OpecodeKind::OP_SRLIW => Extensions::BaseI,
            OpecodeKind::OP_SRAIW => Extensions::BaseI,
            OpecodeKind::OP_ADDW => Extensions::BaseI,
            OpecodeKind::OP_SUBW => Extensions::BaseI,
            OpecodeKind::OP_SLLW => Extensions::BaseI,
            OpecodeKind::OP_SRLW => Extensions::BaseI,
            OpecodeKind::OP_SRAW => Extensions::BaseI,
            OpecodeKind::OP_CSRRW => Extensions::Zicsr,
            OpecodeKind::OP_CSRRS => Extensions::Zicsr,
            OpecodeKind::OP_CSRRC => Extensions::Zicsr,
            OpecodeKind::OP_CSRRWI => Extensions::Zicsr,
            OpecodeKind::OP_CSRRSI => Extensions::Zicsr,
            OpecodeKind::OP_CSRRCI => Extensions::Zicsr,
            OpecodeKind::OP_SRET => Extensions::Priv,
            OpecodeKind::OP_MRET => Extensions::Priv,
            OpecodeKind::OP_WFI => Extensions::Priv,
            OpecodeKind::OP_SFENCE_VMA => Extensions::Priv,
            OpecodeKind::OP_MUL => Extensions::M,
            OpecodeKind::OP_MULH => Extensions::M,
            OpecodeKind::OP_MULHSU => Extensions::M,
            OpecodeKind::OP_MULHU => Extensions::M,
            OpecodeKind::OP_DIV => Extensions::M,
            OpecodeKind::OP_DIVU => Extensions::M,
            OpecodeKind::OP_REM => Extensions::M,
            OpecodeKind::OP_REMU => Extensions::M,
            OpecodeKind::OP_MULW => Extensions::M,
            OpecodeKind::OP_DIVW => Extensions::M,
            OpecodeKind::OP_DIVUW => Extensions::M,
            OpecodeKind::OP_REMW => Extensions::M,
            OpecodeKind::OP_REMUW => Extensions::M,
            OpecodeKind::OP_LR_W => Extensions::A,
            OpecodeKind::OP_SC_W => Extensions::A,
            OpecodeKind::OP_AMOSWAP_W => Extensions::A,
            OpecodeKind::OP_AMOADD_W => Extensions::A,
            OpecodeKind::OP_AMOXOR_W => Extensions::A,
            OpecodeKind::OP_AMOAND_W => Extensions::A,
            OpecodeKind::OP_AMOOR_W => Extensions::A,
            OpecodeKind::OP_AMOMIN_W => Extensions::A,
            OpecodeKind::OP_AMOMAX_W => Extensions::A,
            OpecodeKind::OP_AMOMINU_W => Extensions::A,
            OpecodeKind::OP_AMOMAXU_W => Extensions::A,
            OpecodeKind::OP_LR_D => Extensions::A,
            OpecodeKind::OP_SC_D => Extensions::A,
            OpecodeKind::OP_AMOSWAP_D => Extensions::A,
            OpecodeKind::OP_AMOADD_D => Extensions::A,
            OpecodeKind::OP_AMOXOR_D => Extensions::A,
            OpecodeKind::OP_AMOAND_D => Extensions::A,
            OpecodeKind::OP_AMOOR_D => Extensions::A,
            OpecodeKind::OP_AMOMIN_D => Extensions::A,
            OpecodeKind::OP_AMOMAX_D => Extensions::A,
            OpecodeKind::OP_AMOMINU_D => Extensions::A,
            OpecodeKind::OP_AMOMAXU_D => Extensions::A,
            OpecodeKind::OP_C_ADDI4SPN => Extensions::C,
            OpecodeKind::OP_C_LW => Extensions::C,
            OpecodeKind::OP_C_SW => Extensions::C,
            OpecodeKind::OP_C_NOP => Extensions::C,
            OpecodeKind::OP_C_ADDI => Extensions::C,
            OpecodeKind::OP_C_JAL => Extensions::C,
            OpecodeKind::OP_C_LI => Extensions::C,
            OpecodeKind::OP_C_ADDI16SP => Extensions::C,
            OpecodeKind::OP_C_LUI => Extensions::C,
            OpecodeKind::OP_C_SRLI => Extensions::C,
            OpecodeKind::OP_C_SRAI => Extensions::C,
            OpecodeKind::OP_C_ANDI => Extensions::C,
            OpecodeKind::OP_C_SUB => Extensions::C,
            OpecodeKind::OP_C_XOR => Extensions::C,
            OpecodeKind::OP_C_OR => Extensions::C,
            OpecodeKind::OP_C_AND => Extensions::C,
            OpecodeKind::OP_C_J => Extensions::C,
            OpecodeKind::OP_C_BEQZ => Extensions::C,
            OpecodeKind::OP_C_BNEZ => Extensions::C,
            OpecodeKind::OP_C_SLLI => Extensions::C,
            OpecodeKind::OP_C_LWSP => Extensions::C,
            OpecodeKind::OP_C_JR => Extensions::C,
            OpecodeKind::OP_C_MV => Extensions::C,
            OpecodeKind::OP_C_EBREAK => Extensions::C,
            OpecodeKind::OP_C_JALR => Extensions::C,
            OpecodeKind::OP_C_ADD => Extensions::C,
            OpecodeKind::OP_C_SWSP => Extensions::C,
        }
    }

    pub fn opc_to_string(&self) -> &'static str {
        match self.opc {
            OpecodeKind::OP_LUI => "lui",
            OpecodeKind::OP_AUIPC => "auipc",
            OpecodeKind::OP_JAL => "jal",
            OpecodeKind::OP_JALR => "jalr",
            OpecodeKind::OP_BEQ => "beq",
            OpecodeKind::OP_BNE => "bne",
            OpecodeKind::OP_BLT => "blt",
            OpecodeKind::OP_BGE => "bge",
            OpecodeKind::OP_BLTU => "bltu",
            OpecodeKind::OP_BGEU => "bgeu",
            OpecodeKind::OP_LB => "lb",
            OpecodeKind::OP_LH => "lh",
            OpecodeKind::OP_LW => "lw",
            OpecodeKind::OP_LBU => "lbu",
            OpecodeKind::OP_LHU => "lhu",
            OpecodeKind::OP_SB => "sb",
            OpecodeKind::OP_SH => "sh",
            OpecodeKind::OP_SW => "sw",
            OpecodeKind::OP_ADDI => "addi",
            OpecodeKind::OP_SLTI => "slti",
            OpecodeKind::OP_SLTIU => "sltiu",
            OpecodeKind::OP_XORI => "xori",
            OpecodeKind::OP_ORI => "ori",
            OpecodeKind::OP_ANDI => "andi",
            OpecodeKind::OP_SLLI => "slli",
            OpecodeKind::OP_SRLI => "srli",
            OpecodeKind::OP_SRAI => "srai",
            OpecodeKind::OP_ADD => "add",
            OpecodeKind::OP_SUB => "sub",
            OpecodeKind::OP_SLL => "sll",
            OpecodeKind::OP_SLT => "slt",
            OpecodeKind::OP_SLTU => "sltu",
            OpecodeKind::OP_XOR => "xor",
            OpecodeKind::OP_SRL => "srl",
            OpecodeKind::OP_SRA => "sra",
            OpecodeKind::OP_OR => "or",
            OpecodeKind::OP_AND => "and",
            OpecodeKind::OP_FENCE => "fence",
            OpecodeKind::OP_ECALL => "ecall",
            OpecodeKind::OP_EBREAK => "ebreak",
            OpecodeKind::OP_LWU => "lwu",
            OpecodeKind::OP_LD => "ld",
            OpecodeKind::OP_SD => "sd",
            OpecodeKind::OP_ADDIW => "addiw",
            OpecodeKind::OP_SLLIW => "slliw",
            OpecodeKind::OP_SRLIW => "srliw",
            OpecodeKind::OP_SRAIW => "sraiw",
            OpecodeKind::OP_ADDW => "addw",
            OpecodeKind::OP_SUBW => "subw",
            OpecodeKind::OP_SLLW => "sllw",
            OpecodeKind::OP_SRLW => "srlw",
            OpecodeKind::OP_SRAW => "sraw",
            OpecodeKind::OP_CSRRW => "csrrw",
            OpecodeKind::OP_CSRRS => "csrrs",
            OpecodeKind::OP_CSRRC => "csrrc",
            OpecodeKind::OP_CSRRWI => "csrrwi",
            OpecodeKind::OP_CSRRSI => "csrrsi",
            OpecodeKind::OP_CSRRCI => "csrrci",
            OpecodeKind::OP_SRET => "sret",
            OpecodeKind::OP_MRET => "mret",
            OpecodeKind::OP_WFI => "wfi",
            OpecodeKind::OP_SFENCE_VMA => "sfence.vma",
            OpecodeKind::OP_MUL => "mul",
            OpecodeKind::OP_MULH => "mulh",
            OpecodeKind::OP_MULHSU => "mulhsu,",
            OpecodeKind::OP_MULHU => "mulhu",
            OpecodeKind::OP_DIV => "div",
            OpecodeKind::OP_DIVU => "divu",
            OpecodeKind::OP_REM => "rem",
            OpecodeKind::OP_REMU => "remu",
            OpecodeKind::OP_MULW => "mulw",
            OpecodeKind::OP_DIVW => "divw",
            OpecodeKind::OP_DIVUW => "divuw",
            OpecodeKind::OP_REMW => "remw",
            OpecodeKind::OP_REMUW => "remuw",
            OpecodeKind::OP_LR_W => "lr.w",
            OpecodeKind::OP_SC_W => "sc.w",
            OpecodeKind::OP_AMOSWAP_W => "amoswap.w",
            OpecodeKind::OP_AMOADD_W => "amoadd.w",
            OpecodeKind::OP_AMOXOR_W => "amoxor.w",
            OpecodeKind::OP_AMOAND_W => "amoand.w",
            OpecodeKind::OP_AMOOR_W => "amoor.w",
            OpecodeKind::OP_AMOMIN_W => "amomin.w",
            OpecodeKind::OP_AMOMAX_W => "amomax.w",
            OpecodeKind::OP_AMOMINU_W => "amominu.w",
            OpecodeKind::OP_AMOMAXU_W => "amomaxu.w",
            OpecodeKind::OP_LR_D => "lr.d",
            OpecodeKind::OP_SC_D => "sc.d",
            OpecodeKind::OP_AMOSWAP_D => "amoswap.d",
            OpecodeKind::OP_AMOADD_D => "amoadd.d",
            OpecodeKind::OP_AMOXOR_D => "amoxor.d",
            OpecodeKind::OP_AMOAND_D => "amoand.d",
            OpecodeKind::OP_AMOOR_D => "amoor.d",
            OpecodeKind::OP_AMOMIN_D => "amomin.d",
            OpecodeKind::OP_AMOMAX_D => "amomax.d",
            OpecodeKind::OP_AMOMINU_D => "amominu.d",
            OpecodeKind::OP_AMOMAXU_D => "amomaxu.d",
            OpecodeKind::OP_C_ADDI4SPN => "C.addi4spn",
            OpecodeKind::OP_C_LW => "C.lw",
            OpecodeKind::OP_C_SW => "C.sw",
            OpecodeKind::OP_C_NOP => "C.nop",
            OpecodeKind::OP_C_ADDI => "C.addi",
            OpecodeKind::OP_C_JAL => "C.jal",
            OpecodeKind::OP_C_LI => "C.li",
            OpecodeKind::OP_C_ADDI16SP => "C.addi16sp",
            OpecodeKind::OP_C_LUI => "C.lui",
            OpecodeKind::OP_C_SRLI => "C.srli",
            OpecodeKind::OP_C_SRAI => "C.srai",
            OpecodeKind::OP_C_ANDI => "C.andi",
            OpecodeKind::OP_C_SUB => "C.sub",
            OpecodeKind::OP_C_XOR => "C.xor",
            OpecodeKind::OP_C_OR => "C.or",
            OpecodeKind::OP_C_AND => "C.and",
            OpecodeKind::OP_C_J => "C.j",
            OpecodeKind::OP_C_BEQZ => "C.beqz",
            OpecodeKind::OP_C_BNEZ => "C.bnez",
            OpecodeKind::OP_C_SLLI => "C.slli",
            OpecodeKind::OP_C_LWSP => "C.lwsp",
            OpecodeKind::OP_C_JR => "C.jr",
            OpecodeKind::OP_C_MV => "C.mv",
            OpecodeKind::OP_C_EBREAK => "C.ebreak",
            OpecodeKind::OP_C_JALR => "C.jalr",
            OpecodeKind::OP_C_ADD => "C.add",
            OpecodeKind::OP_C_SWSP => "C.swsp",
        }
    }
}

pub fn reg2str(rd_value: usize) -> &'static str {
    match rd_value {
        0 => "zero",
        1 => "ra",
        2 => "sp",
        3 => "gp",
        4 => "tp",
        5 => "t0",
        6 => "t1",
        7 => "t2",
        8 => "s0", // fp
        9 => "s1",
        10 => "a0",
        11 => "a1",
        12 => "a2",
        13 => "a3",
        14 => "a4",
        15 => "a5",
        16 => "a6",
        17 => "a7",
        18 => "s2",
        19 => "s3",
        20 => "s4",
        21 => "s5",
        22 => "s6",
        23 => "s7",
        24 => "s8",
        25 => "s9",
        26 => "s10",
        27 => "s11",
        28 => "t3",
        29 => "t4",
        30 => "t5",
        31 => "t6",
        _ => panic!("unknown register"),
    }
}
