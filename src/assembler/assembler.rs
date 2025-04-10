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
            fmt.funct3.unwrap(),
            rs1,
            rs2,
            fmt.funct7.unwrap()
        ))
    }

    // TODO: Parse I-type instructions
}

fn parse_register(p0: &str) -> _ {
    todo!()
}

fn encode_r_type(p0: u32, p1: _, p2: u32, p3: _, p4: _, p5: u32) -> u32 {
    todo!()
}