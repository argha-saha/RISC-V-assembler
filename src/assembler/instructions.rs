use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum InstructionType {
    R,
    I,
    S,
    B,
    U,
    J
}

#[derive(Debug, Copy, Clone)]
pub struct InstructionFormat {
    pub instr_type: InstructionType,
    pub opcode: u32,
    pub funct3: Option<u32>,
    pub funct7: Option<u32>
}

pub struct InstructionSet {
    instructions: HashMap<&'static str, InstructionFormat>
}

impl InstructionSet {
    pub fn new() -> Self {
        let mut instructions = HashMap::new();

        instructions.insert("add", InstructionFormat {
            instr_type: InstructionType::R,
            opcode: 0b0110011,
            funct3: Some(0b000),
            funct7: Some(0b0000000)
        });

        Self { instructions }
    }

    pub fn get_instruction(&self, instr: &str) -> Option<&InstructionFormat> {
        self.instructions.get(instr)
    }
}