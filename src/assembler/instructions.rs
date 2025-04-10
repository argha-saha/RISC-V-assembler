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
    pub fmt: InstructionType,
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

        // R-type Instructions

        instructions.insert("add", InstructionFormat {
            fmt: InstructionType::R,
            opcode: 0b0110011,
            funct3: Some(0b000),
            funct7: Some(0b0000000)
        });

        instructions.insert("sub", InstructionFormat {
            fmt: InstructionType::R,
            opcode: 0b0110011,
            funct3: Some(0b000),
            funct7: Some(0b0100000)
        });

        instructions.insert("xor", InstructionFormat {
            fmt: InstructionType::R,
            opcode: 0b0110011,
            funct3: Some(0b100),
            funct7: Some(0b0000000)
        });

        instructions.insert("or", InstructionFormat {
            fmt: InstructionType::R,
            opcode: 0b0110011,
            funct3: Some(0b110),
            funct7: Some(0b0000000)
        });

        instructions.insert("and", InstructionFormat {
            fmt: InstructionType::R,
            opcode: 0b0110011,
            funct3: Some(0b111),
            funct7: Some(0b0000000)
        });

        instructions.insert("sll", InstructionFormat {
            fmt: InstructionType::R,
            opcode: 0b0110011,
            funct3: Some(0b001),
            funct7: Some(0b0000000)
        });

        instructions.insert("srl", InstructionFormat {
            fmt: InstructionType::R,
            opcode: 0b0110011,
            funct3: Some(0b101),
            funct7: Some(0b0000000)
        });

        instructions.insert("sra", InstructionFormat {
            fmt: InstructionType::R,
            opcode: 0b0110011,
            funct3: Some(0b101),
            funct7: Some(0b0100000)
        });

        instructions.insert("slt", InstructionFormat {
            fmt: InstructionType::R,
            opcode: 0b0110011,
            funct3: Some(0b010),
            funct7: Some(0b0000000)
        });

        instructions.insert("sltu", InstructionFormat {
            fmt: InstructionType::R,
            opcode: 0b0110011,
            funct3: Some(0b011),
            funct7: Some(0b0000000)
        });

        Self { instructions }
    }

    pub fn get_instruction(&self, instr: &str) -> Option<&InstructionFormat> {
        self.instructions.get(instr)
    }
}