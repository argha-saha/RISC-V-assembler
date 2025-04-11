use crate::assembler::error::AssemblerError;
use crate::assembler::instructions::{InstructionFormat, InstructionSet, InstructionType};

struct Parser {
    instructions: InstructionSet
}

impl Parser {
    pub fn new() -> Self {
        Self {
            instructions: InstructionSet::new()
        }
    }

    pub fn parse_line(&self, line: &str) -> Result<Option<u32>, AssemblerError> {
        let line = line
            .split('#')
            .next()
            .unwrap()
            .trim();

        if line.is_empty() {
            return Ok(None);
        }

        // "add x4, x5, x6" -> vec!["add", "x4", "x5", "x6"]
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        // Holds the instruction and operands
        let mnemonic = parts[0];
        let operands = &parts[1..];

        if operands.is_empty() {
            return Err(AssemblerError::ParseError("Missing operands".into()));
        }

        let instr = self.instructions.get_instruction(mnemonic)
            .ok_or_else(|| AssemblerError::InvalidInstruction(mnemonic.to_string()))?.clone();

        match instr.fmt {
            InstructionType::R => self.parse_r_type(&instr, operands),
            _ => Err(AssemblerError::ParseError("Invalid/unsupported instruction FMT".into())),
        }
    }

    // Parse R-type instructions
    fn parse_r_type(&self, fmt: &InstructionFormat, operands: &[&str]) -> Result<u32, AssemblerError> {
        if operands.len() != 3 {
            return Err(AssemblerError::ParseError(format!(
                "Expected 3 operands but received {}",
                operands.len()
            )));
        }

        let rd = parse_register(operands[0])?;
        let rs1 = parse_register(operands[1])?;
        let rs2 = parse_register(operands[2])?;

        Ok(encode_r_type(
            fmt.opcode,
            rd,
            fmt.funct3.unwrap_or(0),
            rs1,
            rs2,
            fmt.funct7.unwrap_or(0)
        ))
    }

    // TODO: Parse I-type instructions
}

fn parse_register(register: &str) -> Result<u32, AssemblerError> {
    
}

// R-type Instruction Format
// funct7 | rs2 | rs1 | funct3 | rd | opcode
fn encode_r_type(opcode: u32, rd: u32, funct3: u32, rs1: u32, rs2: u32, funct7: u32) -> u32 {
    (funct7 << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode
}

// I-type Instruction Format
// imm[11:0] | rs1 | funct3 | rd | opcode
fn encode_i_type(opcode: u32, rd: u32, funct3: u32, rs1: u32, imm: i32) -> u32 {
    let imm = (imm as u32) & 0xFFF;
    (imm << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode
}

// S-type Instruction Format
// imm[11:5] | rs2 | rs1 | funct3 | imm[4:0] | opcode
fn encode_s_type(opcode: u32, funct3: u32, rs1: u32, rs2: u32, imm: i32) -> u32 {
    let imm = (imm as u32) & 0xFFF;

    // Lower and upper segments of the immediate
    let imm_lower = imm & 0x1F;
    let imm_upper = (imm >> 5) & 0x7F;

    (imm_upper << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (imm_lower << 7) | opcode
}

// B-type Instruction Format
// imm[12|10:5] | rs2 | rs1 | funct3 | imm[4:1|11] | opcode
fn encode_b_type(opcode: u32, funct3: u32, rs1: u32, rs2: u32, imm: i32) -> u32 {
    let imm = (imm as u32) & 0x1FFF;

    let imm_11 = (imm >> 11) & 0x1;
    let imm_4_1 = (imm >> 1) & 0xF;
    let imm_10_5 = (imm >> 5) & 0x3F;
    let imm_12 = (imm >> 12) & 0x1;

    (imm_12 << 31)
        | (imm_10_5 << 25)
        | (rs2 << 20)
        | (rs1 << 15)
        | (funct3 << 12)
        | (imm_4_1 << 8)
        | (imm_11 << 7)
        | opcode
}

// U-type Instruction Format
// imm[31:12] | rd | opcode
fn encode_u_type() {

}

// J-type Instruction Format
// imm[20|10:1|11|19:12] | rd | opcode
fn encode_j_type() {

}