//! Defines the RISC-V instruction set

use phf::phf_map;

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

static INSTRUCTIONS: phf::Map<&'static str, InstructionFormat> = phf_map! {
    /* R-type Instructions */

    // ADD
    "add" => InstructionFormat {
        fmt: InstructionType::R,
        opcode: 0b0110011,
        funct3: Some(0b000),
        funct7: Some(0b0000000)
    },

    // ADD Word (RV64I)
    "addw" => InstructionFormat {
        fmt: InstructionType::R,
        opcode: 0b0111011,
        funct3: Some(0b000),
        funct7: Some(0b0000000)
    },

    // SUB
    "sub" => InstructionFormat {
        fmt: InstructionType::R,
        opcode: 0b0110011,
        funct3: Some(0b000),
        funct7: Some(0b0100000)
    },

    // SUB Word (RV64I)
    "subw" => InstructionFormat {
        fmt: InstructionType::R,
        opcode: 0b0111011,
        funct3: Some(0b000),
        funct7: Some(0b0100000)
    },

    // XOR
    "xor" => InstructionFormat {
        fmt: InstructionType::R,
        opcode: 0b0110011,
        funct3: Some(0b100),
        funct7: Some(0b0000000)
    },

    // OR
    "or" => InstructionFormat {
        fmt: InstructionType::R,
        opcode: 0b0110011,
        funct3: Some(0b110),
        funct7: Some(0b0000000)
    },

    // AND
    "and" => InstructionFormat {
        fmt: InstructionType::R,
        opcode: 0b0110011,
        funct3: Some(0b111),
        funct7: Some(0b0000000)
    },

    // Shift Left Logical
    "sll" => InstructionFormat {
        fmt: InstructionType::R,
        opcode: 0b0110011,
        funct3: Some(0b001),
        funct7: Some(0b0000000)
    },

    // Shift Left Logical Word (RV64I)
    "sllw" => InstructionFormat {
        fmt: InstructionType::R,
        opcode: 0b0111011,
        funct3: Some(0b001),
        funct7: Some(0b0000000)
    },

    // Shift Right Logical
    "srl" => InstructionFormat {
        fmt: InstructionType::R,
        opcode: 0b0110011,
        funct3: Some(0b101),
        funct7: Some(0b0000000)
    },

    // Shift Right Logical Word (RV64I)
    "srlw" => InstructionFormat {
        fmt: InstructionType::R,
        opcode: 0b0111011,
        funct3: Some(0b101),
        funct7: Some(0b0000000)
    },

    // Shift Right Arithmetic
    "sra" => InstructionFormat {
        fmt: InstructionType::R,
        opcode: 0b0110011,
        funct3: Some(0b101),
        funct7: Some(0b0100000)
    },

    // Shift Right Arithmetic Word (RV64I)
    "sraw" => InstructionFormat {
        fmt: InstructionType::R,
        opcode: 0b0111011,
        funct3: Some(0b101),
        funct7: Some(0b0100000)
    },

    // Set Less Than
    "slt" => InstructionFormat {
        fmt: InstructionType::R,
        opcode: 0b0110011,
        funct3: Some(0b010),
        funct7: Some(0b0000000)
    },

    // Set Less Than Unsigned
    "sltu" => InstructionFormat {
        fmt: InstructionType::R,
        opcode: 0b0110011,
        funct3: Some(0b011),
        funct7: Some(0b0000000)
    },

    /* I-type Instructions */

    // ADD Immediate
    "addi" => InstructionFormat {
        fmt: InstructionType::I,
        opcode: 0b0010011,
        funct3: Some(0b000),
        funct7: None
    },

    // ADD Immediate Word (RV64I)
    "addiw" => InstructionFormat {
        fmt: InstructionType::I,
        opcode: 0b0011011,
        funct3: Some(0b000),
        funct7: None
    },

    // XOR Immediate
    "xori" => InstructionFormat {
        fmt: InstructionType::I,
        opcode: 0b0010011,
        funct3: Some(0b100),
        funct7: None
    },

    // OR Immediate
    "ori" => InstructionFormat {
        fmt: InstructionType::I,
        opcode: 0b0010011,
        funct3: Some(0b110),
        funct7: None
    },

    // AND Immediate
    "andi" => InstructionFormat {
        fmt: InstructionType::I,
        opcode: 0b0010011,
        funct3: Some(0b111),
        funct7: None
    },

    // Shift Left Logical Immediate
    "slli" => InstructionFormat {
        fmt: InstructionType::I,
        opcode: 0b0010011,
        funct3: Some(0b001),
        funct7: Some(0b0000000)
    },

    // Shift Left Logical Immediate Word (RV64I)
    "slliw" => InstructionFormat {
        fmt: InstructionType::I,
        opcode: 0b0011011,
        funct3: Some(0b001),
        funct7: Some(0b0000000)
    },

    // Shift Right Logical Immediate
    "srli" => InstructionFormat {
        fmt: InstructionType::I,
        opcode: 0b0010011,
        funct3: Some(0b101),
        funct7: Some(0b0000000)
    },

    // Shift Right Logical Immediate Word (RV64I)
    "srliw" => InstructionFormat {
        fmt: InstructionType::I,
        opcode: 0b0011011,
        funct3: Some(0b101),
        funct7: Some(0b0000000)
    },

    // Shift Right Arithmetic Immediate
    "srai" => InstructionFormat {
        fmt: InstructionType::I,
        opcode: 0b0010011,
        funct3: Some(0b101),
        funct7: Some(0b0100000)
    },

    // Shift Right Arithmetic Immediate Word (RV64I)
    "sraiw" => InstructionFormat {
        fmt: InstructionType::I,
        opcode: 0b0010011,
        funct3: Some(0b101),
        funct7: Some(0b0100000)
    },

    // Set Less Than Immediate
    "slti" => InstructionFormat {
        fmt: InstructionType::I,
        opcode: 0b0010011,
        funct3: Some(0b010),
        funct7: None
    },

    // Set Less Than Immediate Unsigned
    "sltiu" => InstructionFormat {
        fmt: InstructionType::I,
        opcode: 0b0010011,
        funct3: Some(0b011),
        funct7: None
    },

    /* I-type Load Instructions */

    // Load Byte
    "lb" => InstructionFormat {
        fmt: InstructionType::I,
        opcode: 0b0000011,
        funct3: Some(0b000),
        funct7: None
    },

    // Load Halfword
    "lh" => InstructionFormat {
        fmt: InstructionType::I,
        opcode: 0b0000011,
        funct3: Some(0b001),
        funct7: None
    },

    // Load Word
    "lw" => InstructionFormat {
        fmt: InstructionType::I,
        opcode: 0b0000011,
        funct3: Some(0b010),
        funct7: None
    },

    // Load Word Unsigned (RV64I)
    "lwu" => InstructionFormat {
        fmt: InstructionType::I,
        opcode: 0b0000011,
        funct3: Some(0b110),
        funct7: None
    },

    // Load Doubleword (RV64I)
    "ld" => InstructionFormat {
        fmt: InstructionType::I,
        opcode: 0b0000011,
        funct3: Some(0b011),
        funct7: None
    },

    // Load Byte Unsigned
    "lbu" => InstructionFormat {
        fmt: InstructionType::I,
        opcode: 0b0000011,
        funct3: Some(0b100),
        funct7: None
    },

    // Load Halfword Unsigned
    "lhu" => InstructionFormat {
        fmt: InstructionType::I,
        opcode: 0b0000011,
        funct3: Some(0b101),
        funct7: None
    },

    /* S-type Instructions */

    // Store Byte
    "sb" => InstructionFormat {
        fmt: InstructionType::S,
        opcode: 0b0100011,
        funct3: Some(0b000),
        funct7: None
    },

    // Store Halfword
    "sh" => InstructionFormat {
        fmt: InstructionType::S,
        opcode: 0b0100011,
        funct3: Some(0b001),
        funct7: None
    },

    // Store Word
    "sw" => InstructionFormat {
        fmt: InstructionType::S,
        opcode: 0b0100011,
        funct3: Some(0b010),
        funct7: None
    },

    // Store Doubleword (RV64I)
    "sd" => InstructionFormat {
        fmt: InstructionType::S,
        opcode: 0b0100011,
        funct3: Some(0b011),
        funct7: None
    },

    /* B-type Instructions */

    // Branch ==
    "beq" => InstructionFormat {
        fmt: InstructionType::B,
        opcode: 0b1100011,
        funct3: Some(0b000),
        funct7: None
    },

    // Branch !=
    "bne" => InstructionFormat {
        fmt: InstructionType::B,
        opcode: 0b1100011,
        funct3: Some(0b001),
        funct7: None
    },

    // Branch <
    "blt" => InstructionFormat {
        fmt: InstructionType::B,
        opcode: 0b1100011,
        funct3: Some(0b100),
        funct7: None
    },

    // Branch >=
    "bge" => InstructionFormat {
        fmt: InstructionType::B,
        opcode: 0b1100011,
        funct3: Some(0b101),
        funct7: None
    },

    // Branch < Unsigned
    "bltu" => InstructionFormat {
        fmt: InstructionType::B,
        opcode: 0b1100011,
        funct3: Some(0b110),
        funct7: None
    },

    // Branch >= Unsigned
    "bgeu" => InstructionFormat {
        fmt: InstructionType::B,
        opcode: 0b1100011,
        funct3: Some(0b111),
        funct7: None
    },

    /* U-type Instructions */

    // Load Upper Immediate
    "lui" => InstructionFormat {
        fmt: InstructionType::U,
        opcode: 0b0110111,
        funct3: None,
        funct7: None
    },

    // Add Upper Immediate to PC
    "auipc" => InstructionFormat {
        fmt: InstructionType::U,
        opcode: 0b0010111,
        funct3: None,
        funct7: None
    },

    /* Jump Instructions */

    // Jump and Link
    "jal" => InstructionFormat {
        fmt: InstructionType::J,
        opcode: 0b1101111,
        funct3: None,
        funct7: None
    },

    // Jump and Link Register
    "jalr" => InstructionFormat {
        fmt: InstructionType::I,
        opcode: 0b1100111,
        funct3: None,
        funct7: None
    },

    /* System Instructions */

    // Environment Call
    "ecall" => InstructionFormat {
        fmt: InstructionType::I,
        opcode: 0b1110011,
        funct3: Some(0b000),
        funct7: Some(0b0000000)
    },

    // Environment Break
    "ebreak" => InstructionFormat {
        fmt: InstructionType::I,
        opcode: 0b1110011,
        funct3: Some(0b000),
        funct7: Some(0b0000001)
    },
};

pub struct InstructionSet;

impl InstructionSet {
    pub fn new() -> Self {
        InstructionSet
    }

    pub fn get_instruction(&self, instr: &str) -> Option<&InstructionFormat> {
        INSTRUCTIONS.get(instr)
    }
}