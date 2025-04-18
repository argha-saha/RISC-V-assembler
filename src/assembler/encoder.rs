//! Functions for encoding RISC-V instructions into 32-bit binary format

// R-type Instruction Format
// funct7 | rs2 | rs1 | funct3 | rd | opcode
pub fn encode_r_type(opcode: u32, rd: u32, funct3: u32, rs1: u32, rs2: u32, funct7: u32) -> u32 {
    (funct7 << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode
}

// I-type Instruction Format
// imm[11:0] | rs1 | funct3 | rd | opcode
pub fn encode_i_type(opcode: u32, rd: u32, funct3: u32, rs1: u32, imm: i32) -> u32 {
    let imm = (imm as u32) & 0xFFF;
    (imm << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode
}

// S-type Instruction Format
// imm[11:5] | rs2 | rs1 | funct3 | imm[4:0] | opcode
pub fn encode_s_type(opcode: u32, funct3: u32, rs1: u32, rs2: u32, imm: i32) -> u32 {
    let imm = (imm as u32) & 0xFFF;

    // Lower and upper segments of the immediate
    let imm_lower = imm & 0x1F;
    let imm_upper = (imm >> 5) & 0x7F;

    (imm_upper << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (imm_lower << 7) | opcode
}

// B-type Instruction Format
// imm[12|10:5] | rs2 | rs1 | funct3 | imm[4:1|11] | opcode
pub fn encode_b_type(opcode: u32, funct3: u32, rs1: u32, rs2: u32, imm: i32) -> u32 {
    let imm = (imm as u32) & 0x1FFF;

    let imm_11 = (imm >> 11) & 0x1;
    let imm_4_1 = (imm >> 1) & 0xF;
    let imm_10_5 = (imm >> 5) & 0x3F;
    let imm_12 = (imm >> 12) & 0x1;

    (imm_12 << 31)
        | (imm_10_5 << 25)
        | (rs2 << 20)
        | (rs1 << 15)
        | (funct3 << 12)
        | (imm_4_1 << 8)
        | (imm_11 << 7)
        | opcode
}

// U-type Instruction Format
// imm[31:12] | rd | opcode
pub fn encode_u_type(opcode: u32, rd: u32, imm: i32) -> u32 {
    let imm20 = ((imm >> 12) as u32) & 0xFFFFF;   // 20 bits
    let imm = imm20 << 12;                        // move to bits 31:12
    let rd  = rd << 7;
    imm | rd | opcode
}

// J-type Instruction Format
// imm[20|10:1|11|19:12] | rd | opcode
pub fn encode_j_type(opcode: u32, rd: u32, imm: i32) -> u32 {
    let imm = (imm as u32) & 0x1FFFFF;

    let imm_19_12 = (imm >> 12) & 0xFF;
    let imm_11 = (imm >> 11) & 0x1;
    let imm_10_1 = (imm >> 1) & 0x3FF;
    let imm_20 = (imm >> 20) & 0x1;

    (imm_20 << 31)
        | (imm_10_1 << 21)
        | (imm_11 << 20)
        | (imm_19_12 << 12)
        | (rd << 7)
        | opcode
}