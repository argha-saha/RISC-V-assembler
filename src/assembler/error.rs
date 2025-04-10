#[derive(Debug)]
pub enum AssemblerError {
    IOError(std::io::Error),
    ParserError(String),
    InvalidInstruction(String),
    InvalidOperand(String)
}