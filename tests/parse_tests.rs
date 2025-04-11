#[cfg(test)]
mod tests {
    use riscv_assembler::assembler::assembler::parse_register;

    #[test]
    fn test_abi_names() {
        let reg_name = "x4";
        let reg_num = parse_register(reg_name).unwrap();
        assert_eq!(reg_num, 4);
    }
}