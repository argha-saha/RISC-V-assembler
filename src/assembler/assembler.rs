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
            InstructionType::R => Ok(Some(self.parse_r_type(&instr, operands)?)),
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

const ABI_NAMES: phf::Map<&'static str, u32> = phf::phf_map! {
    "zero" => 0,
    "ra" => 1,
    "sp" => 2,
    "gp" => 3,
    "tp" => 4,
    "t0" => 5,
    "t1" => 6,
    "t2" => 7,
    "s0" => 8,
    "s1" => 9,
    "a0" => 10,
    "a1" => 11,
    "a2" => 12,
    "a3" => 13,
    "a4" => 14,
    "a5" => 15,
    "a6" => 16,
    "a7" => 17,
    "s2" => 18,
    "s3" => 19,
    "s4" => 20,
    "s5" => 21,
    "s6" => 22,
    "s7" => 23,
    "s8" => 24,
    "s9" => 25,
    "s10" => 26,
    "s11" => 27,
    "t3" => 28,
    "t4" => 29,
    "t5" => 30,
    "t6" => 31,
};

// Parse registers x0 to x31
// ABI names should work as well (zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7)
pub fn parse_register(register: &str) -> Result<u32, AssemblerError> {
    let mut reg_name = register.to_ascii_lowercase();

    // Check if the register is an ABI name
    if let Some(&num) = ABI_NAMES.get(reg_name.as_str()) {
        return Ok(num);
    }

    // Check if the register is x0 to x31 or $0 to $31
    if let Some(stripped) = reg_name.strip_prefix('x') {
        reg_name = stripped.to_string();
    } else if let Some(stripped) = reg_name.strip_prefix('$') {
        reg_name = stripped.to_string();
    }

    if let Ok(num) = reg_name.parse::<u32>() {
        if num < 32 {
            Ok(num)
        } else {
            Err(AssemblerError::InvalidOperand(format!("Invalid register: {}", register)))
        }
    } else {
        Err(AssemblerError::InvalidOperand(format!("Invalid register: {}", register)))
    }
}

pub fn parse_immediate(imm: &str) -> Result<i32, AssemblerError> {
    let imm_str = imm.replace('_', "");

    // Determine the radix
    let (num_str, radix) = if let Some(hex) = imm.strip_prefix("0x") {
        (hex, 16)
    } else if let Some(bin) = imm.strip_prefix("0b") {
        (bin, 2)
    } else if let Some(oct) = imm.strip_prefix("0o") {
        (oct, 8)
    } else {
        (imm_str.as_str(), 10)
    };

    // Check for negative numbers
    let negative = num_str.starts_with('-');
    let abs_num_str = if negative {
        // Remove the negative sign for parsing
        &num_str[1..]
    } else {
        num_str
    };

    let abs_value = i32::from_str_radix(abs_num_str, radix)
        .map_err(|e| AssemblerError::InvalidOperand(
            format!("Invalid immediate {}: {}", imm, e)
        ))?;

    let value = if negative {
        -(abs_value as i32)
    } else {
        abs_value as i32
    };

    Ok(value)
}

pub fn parse_offset(offset: &str) -> Result<(i32, u32), AssemblerError> {
    let mut parts = offset.split('(');

    let imm_str = parts.next()
        .ok_or_else(|| AssemblerError::InvalidOperand(format!("Invalid immediate: {}", offset)))?;
    let mut rs1_str = parts.next()
        .ok_or_else(|| AssemblerError::InvalidOperand(format!("Invalid register: {}", offset)))?;

    if parts.next().is_some() {
        return Err(AssemblerError::ParseError(format!(
            "Invalid offset format: {}",
            offset
        )));
    }

    rs1_str = rs1_str.strip_suffix(')').ok_or_else(|| {
        AssemblerError::ParseError(format!("Invalid offset format: {}", offset))
    })?;

    let imm = parse_immediate(imm_str)?;
    let rs1 = parse_register(rs1_str)?;

    // x0 is hardwired to 0 so it's immutable
    if rs1 == 0 {
        return Err(AssemblerError::InvalidOperand(format!(
            "Invalid register: {}",
            rs1
        )));
    }

    Ok((imm, rs1))
}

// R-type Instruction Format
// funct7 | rs2 | rs1 | funct3 | rd | opcode
pub fn encode_r_type(opcode: u32, rd: u32, funct3: u32, rs1: u32, rs2: u32, funct7: u32) -> u32 {
    (funct7 << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode
}

// I-type Instruction Format
// imm[11:0] | rs1 | funct3 | rd | opcode
pub fn encode_i_type(opcode: u32, rd: u32, funct3: u32, rs1: u32, imm: i32) -> u32 {
    let imm = (imm as u32) & 0xFFF;
    (imm << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode
}

// S-type Instruction Format
// imm[11:5] | rs2 | rs1 | funct3 | imm[4:0] | opcode
pub fn encode_s_type(opcode: u32, funct3: u32, rs1: u32, rs2: u32, imm: i32) -> u32 {
    let imm = (imm as u32) & 0xFFF;

    // Lower and upper segments of the immediate
    let imm_lower = imm & 0x1F;
    let imm_upper = (imm >> 5) & 0x7F;

    (imm_upper << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (imm_lower << 7) | opcode
}

// B-type Instruction Format
// imm[12|10:5] | rs2 | rs1 | funct3 | imm[4:1|11] | opcode
pub fn encode_b_type(opcode: u32, funct3: u32, rs1: u32, rs2: u32, imm: i32) -> u32 {
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
pub fn encode_u_type(opcode: u32, rd: u32, imm: i32) -> u32 {
    let imm = (imm as u32) & 0xFFFFF000;
    imm | (rd << 7) | opcode
}

// J-type Instruction Format
// imm[20|10:1|11|19:12] | rd | opcode
pub fn encode_j_type(opcode: u32, rd: u32, imm: i32) -> u32 {
    let imm = (imm as u32) & 0x1FFFFF;

    let imm_19_12 = (imm >> 12) & 0xFF;
    let imm_11 = (imm >> 11) & 0x1;
    let imm_10_1 = (imm >> 1) & 0x3FF;
    let imm_20 = (imm >> 20) & 0x1;

    (imm_20 << 31)
        | (imm_10_1 << 21)
        | (imm_11 << 20)
        | (imm_19_12 << 12)
        | (rd << 7)
        | opcode
}