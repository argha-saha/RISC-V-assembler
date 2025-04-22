//! Parses RISC-V instructions into 32-bit machine code

use std::collections::HashMap;
use crate::assembler::error::AssemblerError;
use crate::assembler::instructions::{InstructionFormat, InstructionSet, InstructionType};
use crate::assembler::encoder::*;
use crate::assembler::pseudo_instructions::PseudoInstructions;

static NO_OPERAND_INSTRUCTIONS: phf::Set<&'static str> = phf::phf_set! {
    "nop",
    "ret",
    "ecall",
    "ebreak"
};

pub struct Parser {
    instructions: InstructionSet
}

impl Parser {
    pub fn new() -> Self {
        Self {
            instructions: InstructionSet::new()
        }
    }

    pub fn parse_line(
        &self,
        mut line: &str,
        current_address: u32,
        symbols: &HashMap<String, u32>
    ) -> Result<Vec<u32>, AssemblerError> {
        // Skip comments or extract the code before a comment
        line = line.split('#').next().unwrap().trim();

        if line.is_empty() {
            return Ok(vec![]);
        }

        // Label detection
        if let Some(colon_index) = line.find(':') {
            let after_label = &line[colon_index + 1..].trim();

            if after_label.is_empty() {
                // No instruction after label
                return Ok(vec![]);
            }

            line = after_label;
        }

        // "add x4, x5, x6" -> vec!["add", "x4", "x5", "x6"]
        let parts: Vec<&str> = line.split_whitespace()
            .map(|s| s.trim_end_matches(','))
            .collect();
        
        // Holds the instruction and operands
        let mnemonic = parts[0];
        let operands = &parts[1..];

        // Throw an error if operands are empty unless the instruction is nop or ret
        if operands.is_empty() && !NO_OPERAND_INSTRUCTIONS.contains(mnemonic) {
            return Err(AssemblerError::ParseError("Missing operands".into()));
        }

        // Check for a pseudo-instruction first
        if PseudoInstructions::is_pseudo_instruction(mnemonic, operands.len()) {
            let translated = PseudoInstructions::expand(mnemonic, operands)?;
            
            // Handle multiple expanded instructions
            let mut result = Vec::with_capacity(translated.len());
            
            // Loop through each item in translated vec
            for (idx, instr) in translated.iter().enumerate() {
                let instr_operands: Vec<&str> = instr
                    .operands
                    .iter()
                    .map(|s| s.as_str()).collect();

                if let Some(instr_format) = self.instructions.get_instruction(instr.mnemonic) {
                    // Calculate the address offset for each instruction in the expansion
                    let instr_address = current_address + (idx as u32 * 4);
                    
                    let parsed = match instr_format.fmt {
                        InstructionType::R => self.parse_r_type(instr_format, &instr_operands)?,
                        InstructionType::I => self.parse_i_type(instr_format, &instr_operands)?,
                        InstructionType::S => self.parse_s_type(instr_format, &instr_operands)?,
                        InstructionType::B => self.parse_b_type(instr_format, &instr_operands, instr_address, symbols)?,
                        InstructionType::U => self.parse_u_type(instr_format, &instr_operands, symbols)?,
                        InstructionType::J => self.parse_j_type(instr_format, &instr_operands, instr_address, symbols)?,
                    };
                    
                    result.push(parsed);
                } else {
                    return Err(AssemblerError::InvalidInstruction(instr.mnemonic.to_string()));
                }
            }
            
            return Ok(result);
        }

        // Handle base instructions
        let instr = self.instructions.get_instruction(mnemonic)
            .ok_or_else(|| AssemblerError::InvalidInstruction(mnemonic.to_string()))?.clone();

        let parsed = match instr.fmt {
            InstructionType::R => self.parse_r_type(&instr, operands)?,
            InstructionType::I => self.parse_i_type(&instr, operands)?,
            InstructionType::S => self.parse_s_type(&instr, operands)?,
            InstructionType::B => self.parse_b_type(&instr, operands, current_address, symbols)?,
            InstructionType::U => self.parse_u_type(&instr, operands, symbols)?,
            InstructionType::J => self.parse_j_type(&instr, operands, current_address, symbols)?,
        };
        
        Ok(vec![parsed])
    }

    // Parse R-type instructions
    pub fn parse_r_type(
        &self, 
        fmt: &InstructionFormat, 
        operands: &[&str]
    ) -> Result<u32, AssemblerError> {
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

    pub fn parse_i_type(
        &self,
        fmt: &InstructionFormat,
        operands: &[&str],
    ) -> Result<u32, AssemblerError> {
        if fmt.opcode == 0b1110011 {
            if fmt.funct7 == Some(0x0) {
                // ecall
                return Ok(encode_i_type(
                    fmt.opcode,
                    0,
                    fmt.funct3.unwrap_or(0),
                    0,
                    0x0
                ));
            } else if fmt.funct7 == Some(0x1) {
                // ebreak
                return Ok(encode_i_type(
                    fmt.opcode,
                    0,
                    fmt.funct3.unwrap_or(0),
                    0,
                    0x1
                ));
            }
        }

        let (rd, rs1, imm) = match operands.len() {
            // I-type load instructions
            2 => {
                let rd = parse_register(operands[0])?;
                let (imm, rs1) = parse_offset(operands[1])?;
                (rd, rs1, imm)
            }

            // I-type arithmetic instructions
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

    pub fn parse_s_type(
        &self,
        fmt: &InstructionFormat,
        operands: &[&str]
    ) -> Result<u32, AssemblerError> {
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

    pub fn parse_b_type(
        &self,
        fmt: &InstructionFormat,
        operands: &[&str],
        current_address: u32,
        symbols: &HashMap<String, u32>
    ) -> Result<u32, AssemblerError> {
        if operands.len() != 3 {
            return Err(AssemblerError::ParseError(format!(
                "Expected 3 operands but received {} for a b-type instruction",
                operands.len()
            )));
        }

        let rs1 = parse_register(operands[0])?;
        let rs2 = parse_register(operands[1])?;

        let offset = match parse_immediate(operands[2]) {
            Ok(immediate) => immediate,
            Err(_) => {
                let target_address = symbols.get(operands[2])
                    .ok_or_else(|| AssemblerError::UndefinedLabel(operands[2].to_string()))?;

                // PC relative addressing
                (*target_address as i32) - (current_address as i32)
            }
        };

        Ok(encode_b_type(
            fmt.opcode,
            fmt.funct3.unwrap_or(0),
            rs1,
            rs2,
            offset
        ))
    }

    pub fn parse_u_type(
        &self,
        fmt: &InstructionFormat,
        operands: &[&str],
        symbols: &HashMap<String, u32>
    ) -> Result<u32, AssemblerError> {
        if operands.len() != 2 {
            return Err(AssemblerError::ParseError(format!(
                "Expected 2 operands but received {} for a u-type instruction",
                operands.len()
            )));
        }

        let rd = parse_register(operands[0])?;

        let imm = match parse_immediate(operands[1]) {
            Ok(immediate) => immediate,
            Err(_) => {
                let label = operands[1];
                let target_address = symbols.get(label)
                    .ok_or_else(|| AssemblerError::UndefinedLabel(label.to_string()))?;

                *target_address as i32
            }
        };

        Ok(encode_u_type(
            fmt.opcode,
            rd,
            imm
        ))
    }

    pub fn parse_j_type(
        &self,
        fmt: &InstructionFormat,
        operands: &[&str],
        current_address: u32,
        symbols: &HashMap<String, u32>
    ) -> Result<u32, AssemblerError> {
        if operands.len() != 2 {
            return Err(AssemblerError::ParseError(format!(
                "Expected 2 operands but received {} a j-type instruction",
                operands.len()
            )));
        }

        let rd = parse_register(operands[0])?;

        let imm = match parse_immediate(operands[1]) {
            Ok(immediate) => immediate,
            Err(_) => {
                let target_address = symbols.get(operands[1])
                    .ok_or_else(|| AssemblerError::UndefinedLabel(operands[1].to_string()))?;

                // PC relative addressing
                (*target_address as i32) - (current_address as i32)
            }
        };

        Ok(encode_j_type(
            fmt.opcode,
            rd,
            imm
        ))
    }
}

const ABI_NAME_REGISTERS: phf::Map<&'static str, u32> = phf::phf_map! {
    "zero" => 0,  // Zero constant
    "ra" => 1,    // Return address
    "sp" => 2,    // Stack pointer
    "gp" => 3,    // Global pointer
    "tp" => 4,    // Thread pointer
    "t0" => 5,    // Temporary
    "t1" => 6,    // Temporary
    "t2" => 7,    // Temporary
    "fp" => 8,    // Frame pointer
    "s0" => 8,    // Saved register
    "s1" => 9,    // Saved register
    "a0" => 10,   // Fn args/return values
    "a1" => 11,   // Fn args
    "a2" => 12,   // Fn args
    "a3" => 13,   // Fn args
    "a4" => 14,   // Fn args
    "a5" => 15,   // Fn args
    "a6" => 16,   // Fn args
    "a7" => 17,   // Fn args
    "s2" => 18,   // Saved register
    "s3" => 19,   // Saved register
    "s4" => 20,   // Saved register
    "s5" => 21,   // Saved register
    "s6" => 22,   // Saved register
    "s7" => 23,   // Saved register
    "s8" => 24,   // Saved register
    "s9" => 25,   // Saved register
    "s10" => 26,  // Saved register
    "s11" => 27,  // Saved register
    "t3" => 28,   // Temporary
    "t4" => 29,   // Temporary
    "t5" => 30,   // Temporary
    "t6" => 31,   // Temporary
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

pub fn parse_csr(imm: &str) -> Result<i32, AssemblerError> {
    if let Ok(parsed) = parse_immediate(imm) {
        return Ok(parsed);
    }

    if let Some(&addr) = CSR_ADDRESSES.get(imm) {
        return Ok(addr as i32);
    }

    Err(AssemblerError::InvalidOperand(
        format!("Invalid CSR name or immediate: {}", imm)
    ))
}