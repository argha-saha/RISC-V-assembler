#[cfg(test)]
mod tests {
    use riscv_assembler::assembler::parser::*;
    use riscv_assembler::assembler::instructions::{InstructionType, InstructionFormat};

    #[test]
    fn test_parse_immediate() {
        // Valid hexadecimal
        assert_eq!(parse_immediate("0x0"), Ok(0));
        assert_eq!(parse_immediate("0xACE"), Ok(0xACE));
        assert_eq!(parse_immediate("-0xF"), Ok(-15));
        assert_eq!(parse_immediate("0xFFFFFFFF"), Ok(-1));

        // Valid binary
        assert_eq!(parse_immediate("0b0"), Ok(0));
        assert_eq!(parse_immediate("0b1100"), Ok(12));
        assert_eq!(parse_immediate("-0b0100_0000"), Ok(-64));

        // Valid octal
        assert_eq!(parse_immediate("0o0"), Ok(0));
        assert_eq!(parse_immediate("0o123"), Ok(83));
        assert_eq!(parse_immediate("-0o1000"), Ok(-512));

        // Valid decimal
        assert_eq!(parse_immediate("0"), Ok(0));
        assert_eq!(parse_immediate("64"), Ok(64));
        assert_eq!(parse_immediate("-64"), Ok(-64));
    }

    #[test]
    fn test_parse_offset() {
        assert_eq!(parse_offset("64(x4)"), Ok((64, 4)));
        assert_eq!(parse_offset("-16(x8)"), Ok((-16, 8)));

        // Invalid offsets
        assert!(parse_offset("64(x4").is_err());
        assert!(parse_offset("64x4)").is_err());
    }

    #[test]
    fn test_parse_r_type() {
        let parser = Parser::new();
        let add = InstructionFormat {
            fmt: InstructionType::R,
            opcode: 0b011_0011,
            funct3: Some(0),
            funct7: Some(0)
        };

        assert_eq!(
            parser.parse_r_type(&add, &["x4", "x5", "x6"]),
            Ok(0b0000_0000_0110_0010_1000_0010_0011_0011)
        );
    }

    #[test]
    fn test_parse_i_type() {
        let parser = Parser::new();
        let addi = InstructionFormat {
            fmt: InstructionType::I,
            opcode: 0b001_0011,
            funct3: Some(0),
            funct7: None
        };

        assert_eq!(
            parser.parse_i_type(&addi, &["x4", "x5", "16"]),
            Ok(0b0000_0001_0000_0010_1000_0010_0001_0011)
        );
    }

    #[test]
    fn test_parse_s_type() {
        let parser = Parser::new();
        let lb = InstructionFormat {
            fmt: InstructionType::S,
            opcode: 0b010_0011,
            funct3: Some(0),
            funct7: None
        };

        assert_eq!(
            parser.parse_s_type(&lb, &["x6", "0x111(x5)"]),
            Ok(0b0001_0000_0110_0010_1000_1000_1010_0011)
        )
    }

    #[test]
    fn test_parse_b_type() {
        let parser = Parser::new();
        let beq = InstructionFormat {
            fmt: InstructionType::B,
            opcode: 0b110_0011,
            funct3: Some(0),
            funct7: None
        };

        assert_eq!(
            parser.parse_b_type(&beq, &["x5", "x6", "0x123"]),
            Ok(0b0001_0010_0110_0010_1000_0001_0110_0011)
        );
    }

    fn test_parse_u_type() {
        let parser = Parser::new();
        let lui = InstructionFormat {
            fmt: InstructionType::U,
            opcode: 0b011_0111,
            funct3: None,
            funct7: None
        };

        assert_eq!(
            parser.parse_u_type(&lui, &["x4", "0x12345678"]),
            Ok(0b0001_0010_0011_0100_0101_0010_0011_0111)
        )
    }

    fn test_parse_j_type() {
        let parser = Parser::new();
        let jal = InstructionFormat {
            fmt: InstructionType::J,
            opcode: 0b110_1111,
            funct3: None,
            funct7: None
        };

        assert_eq!(
            parser.parse_j_type(&jal, &["x4", "0x7FFFFFFF"]),
            Ok(0b1111_1111_1111_1111_1111_0010_0110_1111)
        )
    }

    #[test]
    fn test_regular_names() {
        let mut reg_name = "x0";
        let mut reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 0);

        reg_name = "x1";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 1);

        reg_name = "x2";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 2);

        reg_name = "x3";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 3);

        reg_name = "x4";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 4);

        reg_name = "x5";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 5);

        reg_name = "x6";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 6);

        reg_name = "x7";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 7);

        reg_name = "x8";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 8);

        reg_name = "x9";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 9);

        reg_name = "x10";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 10);

        reg_name = "x11";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 11);

        reg_name = "x12";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 12);

        reg_name = "x13";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 13);

        reg_name = "x14";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 14);

        reg_name = "x15";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 15);

        reg_name = "x16";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 16);

        reg_name = "x17";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 17);

        reg_name = "x18";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 18);

        reg_name = "x19";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 19);

        reg_name = "x20";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 20);

        reg_name = "x21";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 21);

        reg_name = "x22";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 22);

        reg_name = "x23";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 23);

        reg_name = "x24";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 24);

        reg_name = "x25";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 25);

        reg_name = "x26";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 26);

        reg_name = "x27";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 27);

        reg_name = "x28";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 28);

        reg_name = "x29";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 29);

        reg_name = "x30";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 30);

        reg_name = "x31";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 31);
    }

    #[test]
    fn test_mips_style_names() {
        let mut reg_name = "$0";
        let mut reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 0);

        reg_name = "$1";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 1);

        reg_name = "$2";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 2);

        reg_name = "$3";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 3);

        reg_name = "$4";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 4);

        reg_name = "$5";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 5);

        reg_name = "$6";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 6);

        reg_name = "$7";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 7);

        reg_name = "$8";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 8);

        reg_name = "$9";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 9);

        reg_name = "$10";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 10);

        reg_name = "$11";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 11);

        reg_name = "$12";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 12);

        reg_name = "$13";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 13);

        reg_name = "$14";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 14);

        reg_name = "$15";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 15);

        reg_name = "$16";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 16);

        reg_name = "$17";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 17);

        reg_name = "$18";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 18);

        reg_name = "$19";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 19);

        reg_name = "$20";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 20);

        reg_name = "$21";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 21);

        reg_name = "$22";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 22);

        reg_name = "$23";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 23);

        reg_name = "$24";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 24);

        reg_name = "$25";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 25);

        reg_name = "$26";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 26);

        reg_name = "$27";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 27);

        reg_name = "$28";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 28);

        reg_name = "$29";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 29);

        reg_name = "$30";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 30);

        reg_name = "$31";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 31);
    }

    #[test]
    fn test_abi_names() {
        let mut reg_name = "zero";
        let mut reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 0);

        reg_name = "ra";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 1);

        reg_name = "sp";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 2);

        reg_name = "gp";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 3);

        reg_name = "tp";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 4);

        reg_name = "t0";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 5);

        reg_name = "t1";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 6);

        reg_name = "t2";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 7);

        reg_name = "s0";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 8);

        reg_name = "s1";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 9);

        reg_name = "a0";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 10);

        reg_name = "a1";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 11);

        reg_name = "a2";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 12);

        reg_name = "a3";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 13);

        reg_name = "a4";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 14);

        reg_name = "a5";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 15);

        reg_name = "a6";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 16);

        reg_name = "a7";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 17);

        reg_name = "s2";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 18);

        reg_name = "s3";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 19);

        reg_name = "s4";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 20);

        reg_name = "s5";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 21);

        reg_name = "s6";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 22);

        reg_name = "s7";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 23);

        reg_name = "s8";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 24);

        reg_name = "s9";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 25);

        reg_name = "s10";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 26);

        reg_name = "s11";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 27);

        reg_name = "t3";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 28);

        reg_name = "t4";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 29);

        reg_name = "t5";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 30);

        reg_name = "t6";
        reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 31);
    }
}