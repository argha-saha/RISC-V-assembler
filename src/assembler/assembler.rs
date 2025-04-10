use crate::assembler::error::AssemblerError;
use crate::assembler::instructions::{InstructionSet, InstructionType};

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

        let parts: Vec<&str> = line.split_whitespace().collect();
        let (mnemonic, operands) = parts.split_first().unwrap();

        let instr = self.instructions.get_instruction(mnemonic)
            .ok_or_else(|| AssemblerError::InvalidInstruction(mnemonic.to_string()))?.clone();

        match instr.fmt {
            InstructionType::R => self.parse_r_type(&instr, operands),
            _ => Err(AssemblerError::ParseError("Invalid/unsupported Instruction FMT".into())),
        }
    }

    // TODO: Parse R-type instructions

    // TODO: Parse I-type instructions
}