use std::fmt;
use std::io::Error;

#[derive(Debug)]
pub enum AssemblerError {
    IOError(Error),
    ParserError(String),
    InvalidInstruction(String),
    InvalidOperand(String)
}

impl fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IOError(e) => write!(f, "IO Error: {}", e),
            Self::ParserError(e) => write!(f, "Parser Error: {}", e),
            Self::InvalidInstruction(e) => write!(f, "Invalid Instruction: {}", e),
            Self::InvalidOperand(e) => write!(f, "Invalid Operand: {}", e)
        }
    }
}