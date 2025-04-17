//! Provides functionality for handling pseudo-instructions

use phf::phf_set;
use crate::assembler::AssemblerError;

pub struct TranslatedInstruction<'a> {
    pub mnemonic: &'a str,
    pub operands: Vec<String>,
}

static PSEUDO_INSTRUCTIONS: phf::Set<&'static str> = phf_set! {
    "nop",
    "mv",
    "not",
    "neg",
    "negw",
    "sext.w",
    "seqz",
    "snez",
    "sltz",
    "sgtz",
    "beqz",
    "bnez",
    "blez",
    "bgez",
    "bltz",
    "bgtz",
    "j",
    "jr",
    "ret"
};

pub struct PseudoInstructions;

impl PseudoInstructions {
    // Check if a mnemonic is a pseudo-instruction
    pub fn is_pseudo_instruction(mnemonic: &str) -> bool {
        if PSEUDO_INSTRUCTIONS.contains(mnemonic) {
            true
        } else {
            false
        }
    }
    
    // Translate a pseudo-instruction into one or more base instructions
    pub fn expand<'a>(
        mnemonic: &'a str, 
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        match mnemonic {
            "nop" => Self::translate_nop(operands),
            "mv" => Self::translate_mv(operands),
            "not" => Self::translate_not(operands),
            "neg" => Self::translate_neg(operands),
            "negw" => Self::translate_negw(operands),
            "sext.w" => Self::translate_sextw(operands),
            "seqz" => Self::translate_seqz(operands),
            "snez" => Self::translate_snez(operands),
            "sltz" => Self::translate_sltz(operands),
            "sgtz" => Self::translate_sgtz(operands),
            "beqz" => Self::translate_beqz(operands),
            "j" => Self::translate_j(operands),
            "jr" => Self::translate_jr(operands),
            "ret" => Self::translate_ret(operands),
            _ => Err(AssemblerError::InvalidInstruction(format!(
                "Unknown pseudo-instruction: {}", mnemonic
            ))),
        }
    }
    
    // nop => addi x0, x0, 0
    fn translate_nop<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        if !operands.is_empty() {
            return Err(AssemblerError::ParseError(format!(
                "Expected 0 operands for nop but received {}", operands.len()
            )));
        }
        
        Ok(vec![
            TranslatedInstruction {
                mnemonic: "addi",
                operands: vec![
                    "x0".to_string(),  // x0
                    "x0".to_string(),  // x0
                    "0".to_string(),   // 0
                ],
            }
        ])
    }

    // mv rd, rs => addi rd, rs, 0
    fn translate_mv<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        if operands.len() != 2 {
            return Err(AssemblerError::ParseError(format!(
                "Expected 2 operands for mv but received {}", operands.len()
            )));
        }
        
        Ok(vec![
            TranslatedInstruction {
                mnemonic: "addi",
                operands: vec![
                    operands[0].to_string(),  // rd
                    operands[1].to_string(),  // rs
                    "0".to_string(),          // 0
                ],
            }
        ])
    }

    // not rd, rs => xori rd, rs, -1
    fn translate_not<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        if operands.len() != 2 {
            return Err(AssemblerError::ParseError(format!(
                "Expected 2 operands for not but received {}", operands.len()
            )));
        }

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "xori",
                operands: vec![
                    operands[0].to_string(),  // rd
                    operands[1].to_string(),  // rs
                    "-1".to_string()          // -1
                ]
            }
        ])
    }

    // neg rd, rs => sub rd, x0, rs
    fn translate_neg<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        if operands.len() != 2 {
            return Err(AssemblerError::InvalidOperand(format!(
                "Expected 2 operands for neg but received {}",
                operands.len()
            )))
        }

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "sub",
                operands: vec![
                    operands[0].to_string(),  // rd
                    "x0".to_string(),         // x0
                    operands[1].to_string(),  // rs
                ]
            }
        ])
    }

    // negw rd, rs => subw rd, x0, rs
    fn translate_negw<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        if operands.len() != 2 {
            return Err(AssemblerError::InvalidOperand(format!(
                "Expected 2 operands for negw but received {}",
                operands.len()
            )))
        }

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "subw",
                operands: vec![
                    operands[0].to_string(),  // rd
                    "x0".to_string(),         // x0
                    operands[1].to_string(),  // rs
                ]
            }
        ])
    }

    // sext.w rd, rs => addiw rd, rs, 0
    fn translate_sextw<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        if operands.len() != 2 {
            return Err(AssemblerError::ParseError(format!(
                "Expected 2 operands for sext.w but received {}", operands.len()
            )));
        }

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "addiw",
                operands: vec![
                    operands[0].to_string(),  // rd
                    operands[1].to_string(),  // rs
                    "0".to_string()           // 0
                ]
            }
        ])
    }

    // seqz rd, rs => sltiu rd, rs, 1
    fn translate_seqz<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        if operands.len() != 2 {
            return Err(AssemblerError::InvalidOperand(format!(
                "Expected 2 operands for seqz but received {}",
                operands.len()
            )))
        }

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "sltiu",
                operands: vec![
                    operands[0].to_string(),  // rd
                    operands[1].to_string(),  // rs
                    "1".to_string()           // 1
                ]
            }
        ])
    }

    // snez rd, rs => sltu rd, x0, rs
    fn translate_snez<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        if operands.len() != 2 {
            return Err(AssemblerError::InvalidOperand(format!(
                "Expected 2 operands for snez but received {}",
                operands.len()
            )))
        }

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "sltu",
                operands: vec![
                    operands[0].to_string(),  // rd
                    "x0".to_string(),         // x0
                    operands[1].to_string()   // rs
                ]
            }
        ])
    }

    // sltz rd, rs => slt rd, rs, x0
    fn translate_sltz<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        if operands.len() != 2 {
            return Err(AssemblerError::InvalidOperand(format!(
                "Expected 2 operands for snez but received {}",
                operands.len()
            )))
        }

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "slt",
                operands: vec![
                    operands[0].to_string(),  // rd
                    operands[1].to_string(),  // rs
                    "x0".to_string()          // x0
                ]
            }
        ])
    }

    // sgtz rd, rs => slt rd, x0, rs
    fn translate_sgtz<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        if operands.len() != 2 {
            return Err(AssemblerError::InvalidOperand(format!(
                "Expected 2 operands for snez but received {}",
                operands.len()
            )))
        }

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "slt",
                operands: vec![
                    operands[0].to_string(),  // rd
                    "x0".to_string(),         // x0
                    operands[1].to_string()   // rs
                ]
            }
        ])
    }

    // beqz rs, offset => beq rs, x0, offset
    fn translate_beqz<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        if operands.len() != 2 {
            return Err(AssemblerError::InvalidOperand(format!(
                "Expected 2 operands for beqz but received {}",
                operands.len()
            )))
        }

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "beq",
                operands: vec![
                    operands[0].to_string(),
                    "x0".to_string(),
                    operands[1].to_string()
                ]
            }
        ])
    }

    // j offset => jal x0, offset
    fn translate_j<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        if operands.len() != 1 {
            return Err(AssemblerError::InvalidOperand(format!(
                "Expected 1 operand for j but received {}",
                operands.len()
            )))
        }

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "jal",
                operands: vec![
                    "x0".to_string(),         // x0
                    operands[0].to_string(),  // offset
                ]
            }
        ])
    }

    // jr rs => jalr x0, rs, 0
    fn translate_jr<'a>(
        operands: &[&str]
    ) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        if operands.len() != 1 {
            return Err(AssemblerError::InvalidOperand(format!(
                "Expected 1 operand for jr but received {}",
                operands.len()
            )))
        }

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "jalr",
                operands: vec![
                    "x0".to_string(),         // x0
                    operands[0].to_string(),  // rs
                    "0".to_string()           // 0
                ]
            }
        ])
    }

    // ret => jalr x0, x1, 0
    fn translate_ret<'a>(operands: &[&str]) -> Result<Vec<TranslatedInstruction<'a>>, AssemblerError> {
        if operands.len() != 0 {
            return Err(AssemblerError::InvalidOperand(format!(
                "Expected 0 operands for ret but received {}",
                operands.len()
            )))
        }

        Ok(vec![
            TranslatedInstruction {
                mnemonic: "jalr",
                operands: vec![
                    "x0".to_string(),  // x0
                    "x1".to_string(),  // x1
                    "0".to_string()    // 0
                ]
            }
        ])
    }
}