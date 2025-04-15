//! Provides a utility function for generating a hex dump file from binary data

use std::fmt::Write;

pub fn generate_hexdump(binary: &[u8]) -> String {
    let mut hexdump = String::new();
    let mut address = 0;

    for chunk in binary.chunks(4) {
        let word = match chunk.len() {
            4 => u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]),
            3 => u32::from_le_bytes([chunk[0], chunk[1], chunk[2], 0]),
            2 => u32::from_le_bytes([chunk[0], chunk[1], 0, 0]),
            1 => u32::from_le_bytes([chunk[0], 0, 0, 0]),
            _ => 0
        };

        write!(&mut hexdump, "{:08x}: {:08x}\n", address, word).unwrap();

        address += 4;
    }

    hexdump
}