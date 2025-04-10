#[derive(Debug)]
pub enum InstructionType {
    R,
    I,
    S,
    B,
    U,
    J
}

#[derive(Debug)]
pub struct InstructionFormat {
    pub instr_type: InstructionType,
    pub opcode: u32,
    pub funct3: Option<u32>,
    pub funct7: Option<u32>
}