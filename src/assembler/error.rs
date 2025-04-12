use std::fmt;

#[derive(Debug, PartialEq)]
pub enum AssemblerError {
    ParseError(String),
    InvalidInstruction(String),
    InvalidOperand(String)
}

impl fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ParseError(e) => write!(f, "Parser Error: {}", e),
            Self::InvalidInstruction(e) => write!(f, "Invalid Instruction: {}", e),
            Self::InvalidOperand(e) => write!(f, "Invalid Operand: {}", e)
        }
    }
}