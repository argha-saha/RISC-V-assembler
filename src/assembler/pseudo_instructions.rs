//! Provides functionality for handling pseudo-instructions

use std::collections::HashMap;
use crate::assembler::AssemblerError;

pub struct ConvertedInstruction<'a> {
    pub mnemonic: &'a str,
    pub operands: Vec<String>,
}

pub struct PseudoInstructions;

impl PseudoInstructions {
    // Check if a mnemonic is a pseudo-instruction
    pub fn is_pseudo_instruction(mnemonic: &str) -> bool {
        match mnemonic {
            "nop" => true,
            _ => false,
        }
    }
    
    // Convert/expand a pseudo-instruction into one or more base instructions
    pub fn expand<'a>(
        mnemonic: &'a str, 
        operands: &[&str], 
        current_address: u32, 
        symbols: &HashMap<String, u32>
    ) -> Result<Vec<ConvertedInstruction<'a>>, AssemblerError> {
        match mnemonic {
            "nop" => Self::convert_nop(operands),
            _ => Err(AssemblerError::InvalidInstruction(format!(
                "Unknown pseudo-instruction: {}", mnemonic
            ))),
        }
    }
    
    // nop => addi x0, x0, 0
    fn convert_nop<'a>(operands: &[&str]) -> Result<Vec<ConvertedInstruction<'a>>, AssemblerError> {
        if !operands.is_empty() {
            return Err(AssemblerError::ParseError(format!(
                "Expected 0 operands for nop but received {}", operands.len()
            )));
        }
        
        Ok(vec![
            ConvertedInstruction {
                mnemonic: "addi",
                operands: vec![
                    "x0".to_string(),  // x0
                    "x0".to_string(),  // x0
                    "0".to_string(),   // 0
                ],
            }
        ])
    }
}