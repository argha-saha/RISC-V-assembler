pub mod parser;
mod error;
pub mod instructions;
pub mod encoder;
pub mod hexdump;
pub mod pseudo_instructions;

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

            // Handle label-only lines or lines with both label and instruction
            if let Some(colon_index) = line.find(':') {
                let code = line[colon_index + 1..].trim();
                
                // Check for label-only line
                if code.is_empty() {
                    continue;
                }
                
                // Process the instruction after the label
                self.process_instruction(code, &mut current_address, &mut output)?;
            } else {
                // No label
                self.process_instruction(line, &mut current_address, &mut output)?;
            }
        }

        Ok(output)
    }

    fn collect_labels(&mut self, file: File) -> Result<(), AssemblerError> {
        self.symbols.clear();
        let mut current_address = 0;

        for line in BufReader::new(file).lines() {
            let line = line?;
            let line = line.splitn(2, '#').next().unwrap().trim();

            // Skip blank lines
            if line.is_empty() {
                continue;
            }

            // Process labels
            if let Some(colon_index) = line.find(':') {
                let label = line[..colon_index].trim();
                let code = line[colon_index + 1..].trim();

                // Add the label to our symbol table
                if !label.is_empty() {
                    self.symbols.insert(label.to_string(), current_address);
                }

                // Process the instruction after the label if it exists
                if !code.is_empty() {
                    self.increment_address(line, &mut current_address);
                }
            } else {
                // No label
                self.increment_address(line, &mut current_address);
            }
        }

        Ok(())
    }

    // Helper function to process a single instruction and update the address
    fn process_instruction(
        &self,
        src: &str,
        addr: &mut u32,
        out: &mut Vec<u8>
    ) -> Result<(), AssemblerError> {
        match self.parser.parse_line(src, *addr, &self.symbols) {
            Ok(instructions) => {
                for word in instructions {
                    out.extend_from_slice(&word.to_le_bytes());
                    *addr += 4;
                }

                Ok(())
            }

            Err(e) => Err(e)
        }
    }

    // Helper function to increment the address
    // Necessary to handle pseudo-instructions that split into multiple base instructions
    fn increment_address(&self, src: &str, addr: &mut u32) {
        match self.parser.parse_line(src, *addr, &HashMap::new()) {
            // Ensures symbols have correct addresses
            Ok(instructions) => *addr += instructions.len() as u32 * 4,
            Err(_) => *addr += 4
        }
    }
}