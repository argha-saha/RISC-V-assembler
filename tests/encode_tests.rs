#[cfg(test)]
mod tests {
    use riscv_assembler::assembler::encoder::{
        encode_r_type, encode_i_type, encode_s_type, 
        encode_b_type, encode_u_type, encode_j_type,
    };

    #[test]
    fn test_encode_r_type() {
        // add
        let opcode = 0b011_0011;
        let rd = 4;
        let funct3 = 0;
        let rs1 = 5;
        let rs2 = 6;
        let funct7 = 0b000_0000;
        let encoded = encode_r_type(opcode, rd, funct3, rs1, rs2, funct7);
        assert_eq!(encoded, 0x00628233);
    }

    #[test]
    fn test_encode_i_type() {
        // addi
        let opcode = 0b001_0011;
        let rd = 4;
        let funct3 = 0b000;
        let rs1 = 5;
        let imm = 0xACE;
        let encoded = encode_i_type(opcode, rd, funct3, rs1, imm);
        assert_eq!(encoded, 0xACE28213);
    }

    #[test]
    fn test_encode_s_type() {
        // sb
        let opcode = 0b010_0011;
        let funct3 = 0b000;
        let rs1 = 5;
        let rs2 = 6;
        let imm = 0x111;
        let encoded = encode_s_type(opcode, funct3, rs1, rs2, imm);
        assert_eq!(encoded, 0x106288A3);
    }

    #[test]
    fn test_encode_b_type() {
        // beq
        let opcode = 0b110_0011;
        let funct3 = 0b000;
        let rs1 = 5;
        let rs2 = 6;
        let imm = 0x123;
        let encoded = encode_b_type(opcode, funct3, rs1, rs2, imm);
        assert_eq!(encoded, 0x12628163);
    }

    #[test]
    fn test_encode_u_type() {
        // lui
        let opcode = 0b011_0111;
        let rd = 4;
        let imm = 0x12345678;
        let encoded = encode_u_type(opcode, rd, imm);
        assert_eq!(encoded, 0x45678237);
    }

    #[test]
    fn test_encode_j_type() {
        // jal
        let opcode = 0b110_1111;
        let mut rd = 4;
        let mut imm = 0x7FFFFFFF;
        let mut encoded = encode_j_type(opcode, rd, imm);
        assert_eq!(encoded, 0xFFFFF26F);

        rd = 5;
        imm = 12;
        encoded = encode_j_type(opcode, rd, imm);
        assert_eq!(encoded, 0xC002EF);
    }
}