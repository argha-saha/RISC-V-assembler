use std::{fmt, io};

#[derive(Debug, PartialEq)]
pub enum AssemblerError {
    IOError(String),
    ParseError(String),
    InvalidInstruction(String),
    InvalidOperand(String),
    UndefinedLabel(String)
}

impl fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IOError(e) => write!(f, "IO Error: {}", e),
            Self::ParseError(e) => write!(f, "Parser Error: {}", e),
            Self::InvalidInstruction(e) => write!(f, "Invalid Instruction: {}", e),
            Self::InvalidOperand(e) => write!(f, "Invalid Operand: {}", e),
            Self::UndefinedLabel(e) => write!(f, "Invalid Label: {}", e)
        }
    }
}

impl std::error::Error for AssemblerError {}

impl From<io::Error> for AssemblerError {
    fn from(e: io::Error) -> Self {
        Self::IOError(e.to_string())
    }
}