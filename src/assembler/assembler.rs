use crate::assembler::error::AssemblerError;
use crate::assembler::instructions::{InstructionFormat, InstructionSet, InstructionType};
use crate::assembler::encoder::*;

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
            InstructionType::I => Ok(Some(self.parse_i_type(&instr, operands)?)),
            InstructionType::S => Ok(Some(self.parse_s_type(&instr, operands)?)),
            InstructionType::B => Ok(Some(self.parse_b_type(&instr, operands)?)),
            InstructionType::U => Ok(Some(self.parse_u_type(&instr, operands)?)),
            InstructionType::J => Ok(Some(self.parse_j_type(&instr, operands)?))
        }
    }

    // Parse R-type instructions
    fn parse_r_type(&self, fmt: &InstructionFormat, operands: &[&str]) -> Result<u32, AssemblerError> {
        if operands.len() != 3 {
            return Err(AssemblerError::ParseError(format!(
                "Expected 3 operands but received {} for an r-type instruction",
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

    fn parse_i_type(&self, fmt: &InstructionFormat, operands: &[&str]) -> Result<u32, AssemblerError> {
        let (rd, rs1, imm) = match operands.len() {
            // I-type load instructions
            2 => {
                let rd = parse_register(operands[0])?;
                let (imm, rs1) = parse_offset(operands[1])?;
                (rd, rs1, imm)
            }

            3 => {
                let rd = parse_register(operands[0])?;
                let rs1 = parse_register(operands[1])?;
                let imm = parse_immediate(operands[2])?;
                (rd, rs1, imm)
            }

            _ => {
                return Err(AssemblerError::ParseError(format!(
                    "Expected 2 or 3 operands but received {} for an i-type instruction",
                    operands.len()
                )));
            }
        };

        Ok(encode_i_type(
            fmt.opcode,
            rd,
            fmt.funct3.unwrap_or(0),
            rs1,
            imm
        ))
    }

    fn parse_s_type(&self, fmt: &InstructionFormat, operands: &[&str]) -> Result<u32, AssemblerError> {
        if operands.len() != 2 {
            return Err(AssemblerError::ParseError(format!(
                "Expected 2 operands but received {} for an s-type instruction",
                operands.len()
            )));
        }

        let rs2 = parse_register(operands[0])?;
        let (imm, rs1) = parse_offset(operands[1])?;

        Ok(encode_s_type(
            fmt.opcode, 
            fmt.funct3.unwrap_or(0), 
            rs1, 
            rs2, 
            imm
        ))
    }

    fn parse_b_type(&self, fmt: &InstructionFormat, operands: &[&str]) -> Result<u32, AssemblerError> {
        if operands.len() != 3 {
            return Err(AssemblerError::ParseError(format!(
                "Expected 3 operands but received {} for a b-type instruction",
                operands.len()
            )));
        }

        let rs1 = parse_register(operands[0])?;
        let rs2 = parse_register(operands[1])?;
        let imm = parse_immediate(operands[2])?;

        Ok(encode_b_type(
            fmt.opcode,
            fmt.funct3.unwrap_or(0),
            rs1,
            rs2,
            imm
        ))
    }

    fn parse_u_type(&self, fmt: &InstructionFormat, operands: &[&str]) -> Result<u32, AssemblerError> {
        if operands.len() != 2 {
            return Err(AssemblerError::ParseError(format!(
                "Expected 2 operands but received {} for a u-type instruction",
                operands.len()
            )));
        }

        let rd = parse_register(operands[0])?;
        let imm = parse_immediate(operands[1])?;

        Ok(encode_u_type(
            fmt.opcode,
            rd,
            imm
        ))
    }

    fn parse_j_type(&self, fmt: &InstructionFormat, operands: &[&str]) -> Result<u32, AssemblerError> {
        if operands.len() != 2 {
            return Err(AssemblerError::ParseError(format!(
                "Expected 2 operands but received {} a j-type instruction",
                operands.len()
            )));
        }

        let rd = parse_register(operands[0])?;
        let imm = parse_immediate(operands[1])?;

        Ok(encode_j_type(
            fmt.opcode,
            rd,
            imm
        ))
    }
}

const ABI_NAME_REGISTERS: phf::Map<&'static str, u32> = phf::phf_map! {
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
    if let Some(&num) = ABI_NAME_REGISTERS.get(reg_name.as_str()) {
        return Ok(num);
    }

    // Check if the register is x0 to x31 or $0 to $31
    if let Some(stripped) = reg_name.strip_prefix('x') {         // RISC-V style
        reg_name = stripped.to_string();
    } else if let Some(stripped) = reg_name.strip_prefix('$') {  // MIPS style
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
    // Remove underscores (0xFFFF_FFFF -> 0xFFFFFFFF)
    let imm_str = imm.replace('_', "");

    // Check for negative
    let negative = imm_str.starts_with('-');
    let unsigned_imm_str = if negative {
        &imm_str[1..]
    } else {
        &imm_str
    };

    // Determine the radix
    let (num_str, radix) = if let Some(hex) = unsigned_imm_str.strip_prefix("0x") {
        (hex, 16)
    } else if let Some(bin) = unsigned_imm_str.strip_prefix("0b") {
        (bin, 2)
    } else if let Some(oct) = unsigned_imm_str.strip_prefix("0o") {
        (oct, 8)
    } else {
        (unsigned_imm_str, 10)
    };

    // Parse the immediate
    let parsed_imm = if radix == 10 {
        // Decimal
        let value = i32::from_str_radix(num_str, 10).map_err(|e| {
            AssemblerError::InvalidOperand(format!("Invalid immediate: {}", e))
        })?;

        // Restore the sign
        if negative {
            -value
        } else {
            value
        }
    } else {
        let value = u32::from_str_radix(num_str, radix).map_err(|e| {
            AssemblerError::InvalidOperand(format!("Invalid immediate: {}", e))
        })? as i32;

        if negative {
            -value
        } else {
            value
        }
    };

    Ok(parsed_imm)
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