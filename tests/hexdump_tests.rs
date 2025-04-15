#[cfg(test)]
mod tests {
    use riscv_assembler::assembler::hexdump::*;

    #[test]
    fn test_hexdump() {
        // addi a0, x0, 8
        // addi a1, x0, 8
        let binary = vec![0x13, 0x05, 0x80, 0x00, 0x93, 0x05, 0x80, 0x00];
        let expected = "00000000: 00800513\n00000004: 00800593\n";
        assert_eq!(generate_hexdump(&binary), expected);
    }
}