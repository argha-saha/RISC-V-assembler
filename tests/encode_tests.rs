#[cfg(test)]
mod tests {
    use riscv_assembler::assembler::assembler::{
        encode_r_type, encode_i_type, encode_s_type, 
        encode_b_type, encode_u_type, encode_j_type,
    };

    #[test]
    fn test_encode_r_type() {
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
        let opcode = 0b001_0011;
        let rd = 4;
        let funct3 = 0b000;
        let rs1 = 5;
        let imm = 0xACE;
        let encoded = encode_i_type(opcode, rd, funct3, rs1, imm);
        assert_eq!(encoded, 0xACE28213);
    }
}