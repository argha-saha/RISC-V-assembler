pub mod parser;
mod error;
pub mod instructions;
pub mod encoder;
pub mod hexdump;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub use error::AssemblerError;
pub use parser::Parser;
pub use encoder::*;

pub struct Assembler {
    parser: Parser,
    symbols: HashMap<String, u32>  // Store labels
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
            symbols: HashMap::new()
        }
    }

    pub fn assemble(&mut self, path: &str) -> Result<Vec<u8>, AssemblerError> {
        // Collect labels on the first pass
        let file = File::open(path)?;
        self.collect_labels(file)?;

        // Generate the machine code on the second pass
        let file = File::open(path)?;
        let mut output = Vec::new();
        let mut current_address = 0;

        for line in BufReader::new(file).lines() {
            // Skip comments or extract the asm before a comment
            let line = line?;
            let line = line.splitn(2, '#').next().unwrap().trim();

            // Skip blank lines
            if line.is_empty() {
                continue;
            }

            // Parse using the address and symbol hashmap
            match self.parser.parse_line(line, current_address, &self.symbols) {
                Ok(Some(instruction)) => {
                    output.extend_from_slice(&instruction.to_le_bytes());
                    current_address += 4;
                }

                Ok(None) => {}

                Err(e) => return Err(e)
            }
        }

        Ok(output)
    }

    fn collect_labels(&mut self, file: File)  -> Result<(), AssemblerError> {
        let mut current_address = 0;

        for line in BufReader::new(file).lines() {
            let line = line?;
            let line = line.splitn(2 , '#').next().unwrap().trim();

            // Skip over blank lines
            if line.is_empty() {
                continue;
            }

            // Process labels
            if let Some(colon_index) = line.find(':') {
                let (label, code) = line.split_at(colon_index);
                let label = label.trim();
                let code = code[1..].trim();

                // Insert label into the symbols hashmap
                if !label.is_empty() {
                    self.symbols.insert(label.to_string(), current_address);
                }

                if !code.is_empty() {
                    current_address += 4;
                }
            } else {
                current_address += 4;
            }
        }

        Ok(())
    }
}