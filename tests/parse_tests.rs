#[cfg(test)]
mod tests {
    use riscv_assembler::assembler::assembler::parse_register;

    #[test]
    fn test_regular_names() {
        let reg_name = "x4";
        let reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 4);
    }

    #[test]
    fn test_mips_style_names() {
        let reg_name = "$4";
        let reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 4);
    }

    #[test]
    fn test_abi_names() {
        let reg_name = "a0";
        let reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 10);
    }
}