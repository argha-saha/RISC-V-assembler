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

        // ADD
        instructions.insert("add", InstructionFormat {
            fmt: InstructionType::R,
            opcode: 0b0110011,
            funct3: Some(0b000),
            funct7: Some(0b0000000)
        });

        // SUB
        instructions.insert("sub", InstructionFormat {
            fmt: InstructionType::R,
            opcode: 0b0110011,
            funct3: Some(0b000),
            funct7: Some(0b0100000)
        });

        // XOR
        instructions.insert("xor", InstructionFormat {
            fmt: InstructionType::R,
            opcode: 0b0110011,
            funct3: Some(0b100),
            funct7: Some(0b0000000)
        });

        // OR
        instructions.insert("or", InstructionFormat {
            fmt: InstructionType::R,
            opcode: 0b0110011,
            funct3: Some(0b110),
            funct7: Some(0b0000000)
        });

        // AND
        instructions.insert("and", InstructionFormat {
            fmt: InstructionType::R,
            opcode: 0b0110011,
            funct3: Some(0b111),
            funct7: Some(0b0000000)
        });

        // Shift Left Logical
        instructions.insert("sll", InstructionFormat {
            fmt: InstructionType::R,
            opcode: 0b0110011,
            funct3: Some(0b001),
            funct7: Some(0b0000000)
        });

        // Shift Right Logical
        instructions.insert("srl", InstructionFormat {
            fmt: InstructionType::R,
            opcode: 0b0110011,
            funct3: Some(0b101),
            funct7: Some(0b0000000)
        });

        // Shift Right Arithmetic
        instructions.insert("sra", InstructionFormat {
            fmt: InstructionType::R,
            opcode: 0b0110011,
            funct3: Some(0b101),
            funct7: Some(0b0100000)
        });

        // Set Less Than
        instructions.insert("slt", InstructionFormat {
            fmt: InstructionType::R,
            opcode: 0b0110011,
            funct3: Some(0b010),
            funct7: Some(0b0000000)
        });

        // Set Less Than (Unsigned)
        instructions.insert("sltu", InstructionFormat {
            fmt: InstructionType::R,
            opcode: 0b0110011,
            funct3: Some(0b011),
            funct7: Some(0b0000000)
        });

        // I-type Instructions

        // ADD Immediate
        instructions.insert("addi", InstructionFormat {
            fmt: InstructionType::I,
            opcode: 0b0010011,
            funct3: Some(0b000),
            funct7: None
        });

        // XOR Immediate
        instructions.insert("xori", InstructionFormat {
            fmt: InstructionType::I,
            opcode: 0b0010011,
            funct3: Some(0b100),
            funct7: None
        });

        // OR Immediate
        instructions.insert("ori", InstructionFormat {
            fmt: InstructionType::I,
            opcode: 0b0010011,
            funct3: Some(0b110),
            funct7: None
        });

        // AND Immediate
        instructions.insert("andi", InstructionFormat {
            fmt: InstructionType::I,
            opcode: 0b0010011,
            funct3: Some(0b111),
            funct7: None
        });

        // Shift Left Logical Immediate
        instructions.insert("slli", InstructionFormat {
            fmt: InstructionType::I,
            opcode: 0b0010011,
            funct3: Some(0b001),
            funct7: Some(0b0000000)
        });

        // Shift Right Logical Immediate
        instructions.insert("srli", InstructionFormat {
            fmt: InstructionType::I,
            opcode: 0b0010011,
            funct3: Some(0b101),
            funct7: Some(0b0000000)
        });

        // Shift Right Arithmetic Immediate
        instructions.insert("srai", InstructionFormat {
            fmt: InstructionType::I,
            opcode: 0b0010011,
            funct3: Some(0b101),
            funct7: Some(0b0100000)
        });

        // Set Less Than Immediate
        instructions.insert("slti", InstructionFormat {
            fmt: InstructionType::I,
            opcode: 0b0010011,
            funct3: Some(0b010),
            funct7: None
        });

        // Set Less Than Immediate (Unsigned)
        instructions.insert("sltiu", InstructionFormat {
            fmt: InstructionType::I,
            opcode: 0b0010011,
            funct3: Some(0b011),
            funct7: None
        });

        // I-type Instructions (Load-related)

        instructions.insert("lb", InstructionFormat {
            fmt: InstructionType::I,
            opcode: 0b0000011,
            funct3: Some(0b000),
            funct7: None
        });

        instructions.insert("lh", InstructionFormat {
            fmt: InstructionType::I,
            opcode: 0b0000011,
            funct3: Some(0b001),
            funct7: None
        });

        instructions.insert("lw", InstructionFormat {
            fmt: InstructionType::I,
            opcode: 0b0000011,
            funct3: Some(0b010),
            funct7: None
        });

        instructions.insert("lbu", InstructionFormat {
            fmt: InstructionType::I,
            opcode: 0b0000011,
            funct3: Some(0b100),
            funct7: None
        });

        instructions.insert("lhu", InstructionFormat {
            fmt: InstructionType::I,
            opcode: 0b0000011,
            funct3: Some(0b101),
            funct7: None
        });

        Self { instructions }
    }

    pub fn get_instruction(&self, instr: &str) -> Option<&InstructionFormat> {
        self.instructions.get(instr)
    }
}